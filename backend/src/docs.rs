use utoipa::openapi::info::ContactBuilder;
use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::about::AboutResponse;
use crate::settings::Settings;
use crate::resources::user::Role;
use crate::resources::{CreateUser, User};
use money_app_shared::auth::otp::{OtpRequest, OtpVerify};
pub use money_app_shared::error::{ErrorResponse, Problem, ProblemDetails};
use money_app_shared::user::{SessionBody, SessionUserBody};

pub mod rest {
    use super::{Settings, openapi_document};
    use utoipa_swagger_ui::SwaggerUi;

    pub fn scope(settings: Settings) -> SwaggerUi {
        SwaggerUi::new("/api/docs/{_:.*}")
            .url("/api/docs/openapi.json", openapi_document(&settings))
    }
}

pub fn openapi_document(settings: &Settings) -> utoipa::openapi::OpenApi {
    let mut doc = ApiDoc::openapi();
    apply_openapi_runtime_metadata(&mut doc, settings);
    doc
}

fn apply_openapi_runtime_metadata(doc: &mut utoipa::openapi::OpenApi, settings: &Settings) {
    if settings.openapi_contact_email.is_some() || settings.openapi_imprint_url.is_some() {
        let contact = ContactBuilder::new()
            .name(Some("Moneyapp"))
            .email(settings.openapi_contact_email.clone())
            .url(settings.openapi_imprint_url.clone())
            .build();
        doc.info.contact = Some(contact);
    }
}

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Moneyapp API",
        version = "1.0.0",
        description = "Personal finance API under `/api/v1`. Unversioned authentication at `/auth/*`. `GET /api/v1/about` is public.",
        license(name = "MIT", url = "https://opensource.org/licenses/MIT")
    ),
    servers(
        (url = "/", description = "Same origin as the web app."),
    ),
    paths(
        crate::about::get_about,
        crate::auth::oidc::rest::login,
        crate::auth::oidc::rest::callback,
        crate::auth::otp::rest::otp_request,
        crate::auth::otp::rest::otp_verify,
        crate::auth::rest::logout,
        crate::resources::user::rest::get_users_me,
        crate::resources::user::rest::get_users,
        crate::resources::user::rest::get_user,
        crate::resources::user::rest::create_user,
        crate::resources::user::rest::delete_user,
        crate::resources::user::session::rest::get_sessions_for_current_user,
        crate::resources::user::session::rest::get_session_for_current_user,
        crate::resources::user::session::rest::delete_session_for_current_user,
        crate::resources::user::session::rest::get_sessions_for_user,
        crate::resources::user::session::rest::get_session_for_user,
        crate::resources::user::session::rest::create_session_for_user,
        crate::resources::user::session::rest::delete_session_for_user,
        crate::resources::transaction::rest::get_transactions,
        crate::resources::transaction::rest::get_transaction,
        crate::resources::transaction::rest::put_transactions,
        crate::resources::transaction::rest::delete_transactions,
        crate::resources::transaction::rest::post_transactions,
        crate::resources::transaction::rest::get_pods,
        crate::resources::transaction::rest::get_budgets,
        crate::resources::transaction::rest::get_inbudgets,
        crate::resources::transaction::rest::get_debts,
        crate::resources::transaction::rest::get_tags,
        crate::history::rest::get_wealth,
        crate::history::rest::get_pods,
        crate::history::rest::get_budgets,
        crate::history::rest::get_inbudgets,
        crate::history::rest::get_debts,
        crate::extrapolation::rest::get_extrapolation,
        crate::import::rest::post_import,
        crate::resources::attachment::rest::get_attachment,
        crate::resources::attachment::rest::post_attachment,
        crate::resources::contract::rest::get_contracts,
        crate::resources::contract::rest::get_contract,
        crate::resources::contract::rest::put_contracts,
        crate::resources::contract::rest::delete_contracts,
        crate::resources::contract::rest::post_contracts,
        crate::resources::goal::rest::get_goals,
        crate::resources::goal::rest::get_goal,
        crate::resources::goal::rest::put_goals,
        crate::resources::goal::rest::delete_goals,
        crate::resources::goal::rest::post_goals,
    ),
    components(
        schemas(
            AboutResponse,
            User,
            SessionBody,
            SessionUserBody,
            Role,
            CreateUser,
            OtpRequest,
            OtpVerify,
            Problem,
            ErrorResponse,
            ProblemDetails,
        )
    ),
    tags(
        (name = "About", description = "Public server metadata."),
        (name = "Auth", description = "OAuth/OIDC and OTP email authentication."),
        (name = "Users", description = "Users and sessions."),
        (name = "Transactions", description = "Transactions and derived keys."),
        (name = "History", description = "Aggregated history series."),
        (name = "Extrapolation", description = "Monthly extrapolation."),
        (name = "Import", description = "CSV import."),
        (name = "Attachments", description = "Binary attachments."),
        (name = "Contracts", description = "Contracts."),
        (name = "Goals", description = "Goals."),
    ),
    modifiers(&SessionSecurity)
)]
pub struct ApiDoc;

struct SessionSecurity;

impl Modify for SessionSecurity {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "SessionCookie",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::with_description(
                "sso_session",
                "Session cookie returned after a successful authentication flow",
            ))),
        );
        components.add_security_scheme(
            "SessionToken",
            SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::with_description(
                "Authorization",
                "Session override using `Authorization: Bearer <session>` header",
            ))),
        );
    }
}
