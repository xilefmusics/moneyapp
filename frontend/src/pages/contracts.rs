use crate::components::ListItem;
use crate::Route;
use fancy_yew::components::ResourceHeader;
use fancy_yew::rest::Resource;
use money_app_shared::contract::Contract;

use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Contracts() -> Html {
    let contracts: UseStateHandle<Resource<Contract>> =
        use_state(|| Resource::new("/api/v1/contracts".into(), "contracts".into(), Vec::default()));
    {
        let contracts = contracts.clone();
        use_effect_with((), move |_| {
            Resource::reload(contracts);
            || ()
        });
    }

    let navigator = use_navigator().unwrap();
    let mut contracts_sorted = (*contracts).clone();
    contracts_sorted.sort_by(|a, b| {
        a.payment
            .date_of_next_payment()
            .cmp(&b.payment.date_of_next_payment())
    });
    let contract_items = contracts_sorted
        .iter()
        .map(|contract| {
            let navigator = navigator.clone();
            let id = contract.id.clone().unwrap();
            let onedit = {
                let id = id.clone();
                Callback::from(move |_: MouseEvent| {
                    navigator.push(&Route::Contract { id: id.clone() })
                })
            };
            html! {<ListItem
                title={contract.title.clone()}
                subtitle={contract.payment.date_of_next_payment().format("%d %b %Y").to_string()}
                amount={contract.payment.signed_monthly_amount()}
                onedit={onedit}
                ondelete={Resource::delete_callback(contract.clone(),contracts.clone())}
            />}
        })
        .collect::<Vec<Html>>();

    html! {
        <div class={Style::new(include_str!("contracts.css")).expect("Unwrapping CSS should work!")}>
            <ResourceHeader<Contract> handle={contracts.clone()} />
            <ul>
                {contract_items}
            </ul>
        </div>
    }
}
