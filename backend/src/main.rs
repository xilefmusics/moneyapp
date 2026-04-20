use std::sync::Arc;

use actix_web::middleware::Compat;
use actix_web::{App, HttpServer, web::Data};
use anyhow::{Context, Result as AnyResult};
use chrono::Utc;
use lettre::transport::smtp::authentication::Credentials;
use tracing::info;
use tracing_actix_web::TracingLogger;

use backend::auth;
use backend::auth::oidc;
use backend::database;
use backend::docs;
use backend::frontend;
use backend::mail::MailService;
use backend::resources;
use backend::resources::Session;
use backend::resources::user::service::UserServiceHandle;
use backend::resources::user::session::service::SessionServiceHandle;
use backend::resources::user::{Role as UserRole, User};
use backend::settings::Settings;

#[actix_web::main]
async fn main() -> AnyResult<()> {
    backend::observability::init()?;

    let settings = Settings::from_env()?;

    let production = backend::observability::is_production();
    if production && settings.initial_admin_user_test_session {
        anyhow::bail!(
            "refusing to start: initial_admin_user_test_session is enabled under MONEYAPP_PRODUCTION or RUST_ENV=production"
        );
    }

    let static_dir = std::fs::canonicalize(settings.static_dir.as_str())
        .with_context(|| format!("static_dir {:?} could not be resolved", settings.static_dir))?
        .to_string_lossy()
        .into_owned();

    let cookie_config = Data::new(settings.cookie_config());
    let otp_config = Data::new(settings.otp_config());

    let mail_service = MailService::new(
        settings.gmail_from.clone(),
        Credentials::new(
            settings.gmail_from.clone(),
            settings.gmail_app_password.clone(),
        ),
    )?;

    let db = Arc::new(
        database::Database::connect(
            &settings.db_address,
            &settings.db_namespace,
            &settings.db_database,
            settings.db_username.as_deref(),
            settings.db_password.as_deref(),
        )
        .await?,
    );
    db.migrate(settings.db_migration_path.as_str())
        .await
        .context("database migration failed")?;

    let user_service = UserServiceHandle::build(db.clone());
    let session_service = SessionServiceHandle::build(db.clone());

    if let Some(email) = settings.initial_admin_user_email.as_ref() {
        let (admin, created_initial_admin) = if let Some(user) = user_service
            .get_user_by_email(email)
            .await
            .context("failed to look up initial admin user by email")?
        {
            info!(
                "Initial admin user already exists ({}), not creating: {}",
                user.id, user.email
            );
            (user, false)
        } else {
            let admin = user_service
                .create_user(User {
                    id: String::new(),
                    email: email.to_owned(),
                    role: UserRole::Admin,
                    default_collection: None,
                    created_at: Utc::now(),
                    last_login_at: None,
                    request_count: 0,
                    oauth_picture_url: None,
                    oauth_avatar_blob_id: None,
                    avatar_blob_id: None,
                })
                .await
                .context("failed to create admin user")?;
            info!(
                "Created admin user {} with email: {}",
                admin.id, admin.email
            );
            (admin, true)
        };

        if settings.initial_admin_user_test_session {
            if created_initial_admin {
                let session = session_service
                    .create_session(Session::admin(admin, settings.session_ttl_seconds as i64))
                    .await
                    .context("failed to create a test session for the admin user")?;
                info!(
                    "Created a test session {} for the admin user. DO NOT USE THIS IN PRODUCTION",
                    session.id,
                );
            } else {
                info!("Initial admin user was not created on this run, not creating test session");
            }
        }
    }

    let oidc_clients_arc = Arc::new(oidc::build_clients(&settings).await?);
    let oidc_provider_ids = oidc_clients_arc.registered_provider_ids();
    let oidc_clients = Data::new(oidc_clients_arc);

    info!(
        event = "startup",
        host = %settings.host,
        port = settings.port,
        cookie_secure = settings.cookie_secure,
        session_ttl_seconds = settings.session_ttl_seconds,
        otp_ttl_seconds = settings.otp_ttl_seconds,
        otp_allow_self_signup = settings.otp_allow_self_signup,
        otp_max_attempts = settings.otp_max_attempts,
        auth_rate_limit_rps = settings.auth_rate_limit_rps,
        auth_rate_limit_burst = settings.auth_rate_limit_burst,
        api_rate_limit_rps = settings.api_rate_limit_rps,
        api_rate_limit_burst = settings.api_rate_limit_burst,
        production = production,
        static_dir = %static_dir,
        oidc_providers = ?oidc_provider_ids,
        "moneyapp backend starting"
    );

    let db_data = Data::from(db);

    let docs_settings = settings.clone();
    let api_rps = settings.api_rate_limit_rps;
    let api_burst = settings.api_rate_limit_burst;
    let settings_for_app = settings.clone();
    let bind_host = settings.host.clone();
    let bind_port = settings.port;

    HttpServer::new(move || {
        App::new()
            .wrap(backend::request_id::RequestId)
            .wrap(backend::http_audit::HttpAudit::new(db_data.clone()))
            .wrap(Compat::new(TracingLogger::<
                backend::request_id::MoneyRootSpan,
            >::new()))
            .app_data(backend::error::json_config())
            .app_data(db_data.clone())
            .app_data(Data::new(mail_service.clone()))
            .app_data(Data::new(user_service.clone()))
            .app_data(Data::new(session_service.clone()))
            .app_data(oidc_clients.clone())
            .app_data(cookie_config.clone())
            .app_data(otp_config.clone())
            .app_data(Data::new(settings_for_app.clone()))
            .service(auth::rest::scope(
                settings_for_app.auth_rate_limit_rps,
                settings_for_app.auth_rate_limit_burst,
            ))
            .service(docs::rest::scope(docs_settings.clone()))
            .service(resources::rest::scope(
                settings_for_app.clone(),
                api_rps,
                api_burst,
            ))
            .service(frontend::rest::scope(&static_dir))
    })
    .bind((bind_host, bind_port))?
    .run()
    .await
    .context("server exited unexpectedly")
}
