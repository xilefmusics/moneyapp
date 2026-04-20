use crate::about;
use crate::auth::middleware::RequireUser;
use crate::extrapolation;
use crate::governor_audit::AuditRateLimit429;
use crate::governor_peer::PeerOrFallbackIpKeyExtractor;
use crate::history;
use crate::import;

use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::{dev::HttpServiceFactory, web};

pub fn scope(
    settings: crate::settings::Settings,
    api_rate_limit_rps: u64,
    api_rate_limit_burst: u32,
) -> impl HttpServiceFactory {
    let api_governor = GovernorConfigBuilder::default()
        .requests_per_second(api_rate_limit_rps)
        .burst_size(api_rate_limit_burst)
        .key_extractor(PeerOrFallbackIpKeyExtractor)
        .use_headers()
        .finish()
        .expect("API rate-limit configuration");
    let settings_data = web::Data::new(settings);
    web::scope("/api/v1")
        .app_data(settings_data.clone())
        .wrap(Governor::new(&api_governor))
        .wrap(AuditRateLimit429)
        .service(about::get_about)
        .service(
            web::scope("")
                .wrap(RequireUser)
                .service(
                    web::scope("/transactions")
                        .service(crate::resources::transaction::rest::get_transactions)
                        .service(crate::resources::transaction::rest::get_transaction)
                        .service(crate::resources::transaction::rest::put_transactions)
                        .service(crate::resources::transaction::rest::delete_transactions)
                        .service(crate::resources::transaction::rest::post_transactions)
                        .service(crate::resources::transaction::rest::get_pods)
                        .service(crate::resources::transaction::rest::get_budgets)
                        .service(crate::resources::transaction::rest::get_inbudgets)
                        .service(crate::resources::transaction::rest::get_debts)
                        .service(crate::resources::transaction::rest::get_tags),
                )
                .service(
                    web::scope("/history")
                        .service(history::rest::get_wealth)
                        .service(history::rest::get_pods)
                        .service(history::rest::get_budgets)
                        .service(history::rest::get_inbudgets)
                        .service(history::rest::get_debts),
                )
                .service(
                    web::scope("/extrapolation").service(extrapolation::rest::get_extrapolation),
                )
                .service(import::rest::post_import)
                .service(
                    web::scope("")
                        .service(crate::resources::attachment::rest::get_attachment)
                        .service(crate::resources::attachment::rest::post_attachment),
                )
                .service(
                    web::scope("/contracts")
                        .service(crate::resources::contract::rest::get_contracts)
                        .service(crate::resources::contract::rest::get_contract)
                        .service(crate::resources::contract::rest::put_contracts)
                        .service(crate::resources::contract::rest::delete_contracts)
                        .service(crate::resources::contract::rest::post_contracts),
                )
                .service(
                    web::scope("/goals")
                        .service(crate::resources::goal::rest::get_goals)
                        .service(crate::resources::goal::rest::get_goal)
                        .service(crate::resources::goal::rest::put_goals)
                        .service(crate::resources::goal::rest::delete_goals)
                        .service(crate::resources::goal::rest::post_goals),
                )
                .service(crate::resources::user::rest::scope()),
        )
}
