mod charts;
mod components;
mod pages;

use pages::{
    Budgets, Contract, Contracts, Dashboard, Debts, Goal, Goals, Pods, Transaction, Transactions,
};

use fancy_yew::layouts::{HorizontalLayout, NavItemBuilder, Navable};

use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Index,
    #[at("/dashboard")]
    Dashboard,
    #[at("/transactions/:id")]
    Transaction { id: String },
    #[at("/transactions")]
    Transactions,
    #[at("/pods")]
    Pods,
    #[at("/budgets")]
    Budgets,
    #[at("/debts")]
    Debts,
    #[at("/contracts/:id")]
    Contract { id: String },
    #[at("/contracts")]
    Contracts,
    #[at("/goals/:id")]
    Goal { id: String },
    #[at("/goals")]
    Goals,
    #[not_found]
    #[at("/404")]
    NotFound,
    #[at("/login")]
    Login,
}

impl Route {}

impl Navable for Route {
    fn route_items() -> Vec<Self> {
        vec![
            Route::Dashboard,
            Route::Transactions,
            Route::Pods,
            Route::Budgets,
            Route::Debts,
            Route::Contracts,
            Route::Goals,
        ]
    }

    fn to_nav_item(self) -> NavItemBuilder<'static> {
        match self {
            Route::Dashboard => NavItemBuilder::new()
                .path("/dashboard")
                .icon("dashboard")
                .text("Dashboard")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Dashboard)
                }))
                .index(),
            Route::Transactions => NavItemBuilder::new()
                .path("/transactions")
                .icon("list_alt")
                .text("Transactions")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Transactions)
                })),
            Route::Pods => NavItemBuilder::new()
                .path("/pods")
                .icon("wallet")
                .text("Pods")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Pods)
                })),
            Route::Budgets => NavItemBuilder::new()
                .path("/budgets")
                .icon("donut_large")
                .text("Budgets")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Budgets)
                })),
            Route::Debts => NavItemBuilder::new()
                .path("/debts")
                .icon("credit_card_off")
                .text("Debts")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Debts)
                })),
            Route::Contracts => NavItemBuilder::new()
                .path("/contracts")
                .icon("contract")
                .text("Contracts")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Contracts)
                })),
            Route::Goals => NavItemBuilder::new()
                .path("/goals")
                .icon("flag")
                .text("Goals")
                .callback(Callback::from(|navigator: Navigator| {
                    navigator.push(&Route::Goals)
                })),
            _ => NavItemBuilder::new(),
        }
    }

    fn render(route: Route) -> Html {
        html! {
            <HorizontalLayout<Route> nav_routes={Route::route_items()}>{
                match route {
                    Route::Index => html! { <Dashboard /> },
                    Route::Dashboard => html! { <Dashboard /> },
                    Route::Transaction{id} => html! { <Transaction id={id} /> },
                    Route::Transactions => html! { <Transactions /> },
                    Route::Pods => html! { <Pods /> },
                    Route::Budgets => html! { <Budgets /> },
                    Route::Debts => html! { <Debts /> },
                    Route::Contract{id} => html! { <Contract id={id} /> },
                    Route::Contracts => html! { <Contracts /> },
                    Route::Goal{id} => html! { <Goal id={id} /> },
                    Route::Goals => html! { <Goals /> },
                    Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> },
                    Route::Login => html! {
                        <div style="padding: 2rem; max-width: 32rem;">
                            <h1>{ "Sign in" }</h1>
                            <p>{ "Use the same origin as this app. Session cookie is set after OAuth completes." }</p>
                            <p><a href="/auth/login">{ "Continue with Google (OIDC)" }</a></p>
                        </div>
                    },
        }}
            </HorizontalLayout<Route>>
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Route::render} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
