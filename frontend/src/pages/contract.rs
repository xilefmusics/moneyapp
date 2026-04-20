use fancy_yew::components::input::{
    BoolInput, DateInput, NumberInput, RemoteFileInput, StringInput, StringNumberMap,
};
use money_app_shared::attachment::Attachment;
use money_app_shared::contract::{Contract, Payment, PaymentKind, State};

use chrono::{DateTime, Local};
use gloo::net::http::Request;
use std::collections::HashMap;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

fn destruct_contract(
    contract: &Contract,
    id_handle: UseStateHandle<Option<String>>,
    title_handle: UseStateHandle<String>,
    partner_handle: UseStateHandle<String>,
    start_handle: UseStateHandle<DateTime<Local>>,
    state_handle: UseStateHandle<String>,
    term_handle: UseStateHandle<f64>,
    notice_handle: UseStateHandle<f64>,
    management_handle: UseStateHandle<String>,
    payment_first_handle: UseStateHandle<DateTime<Local>>,
    payment_amount_handle: UseStateHandle<f64>,
    payment_fix_handle: UseStateHandle<bool>,
    payment_cycle_handle: UseStateHandle<f64>,
    payment_kind_handle: UseStateHandle<String>,
    payment_pod_handle: UseStateHandle<String>,
    payment_debts_handle: UseStateHandle<HashMap<String, f64>>,
    attachments_handle: UseStateHandle<Vec<String>>,
) {
    id_handle.set(contract.id.clone());
    title_handle.set(contract.title.clone());
    partner_handle.set(contract.partner.clone());
    start_handle.set(contract.start.clone());
    state_handle.set(match contract.state {
        State::Active => "Active".into(),
        State::Terminated => "Terminated".into(),
        State::Expired => "Expired".into(),
    });
    term_handle.set(contract.term as f64);
    notice_handle.set(contract.notice as f64);
    management_handle.set(contract.management.clone());
    payment_first_handle.set(contract.payment.first);
    payment_amount_handle.set(contract.payment.amount as f64);
    payment_fix_handle.set(contract.payment.fix);
    payment_cycle_handle.set(contract.payment.cycle as f64);
    payment_kind_handle.set(match contract.payment.kind {
        PaymentKind::Active => "Active".into(),
        PaymentKind::Debit => "Debit".into(),
        PaymentKind::Credit => "Credit".into(),
        PaymentKind::PayPal => "PayPal".into(),
        PaymentKind::GooglePay => "GooglePay".into(),
    });
    payment_pod_handle.set(contract.payment.pod.clone());
    payment_debts_handle.set(
        contract
            .payment
            .debts
            .iter()
            .map(|(key, value)| (key.to_string(), *value as f64))
            .collect::<HashMap<String, f64>>(),
    );
    attachments_handle.set(contract.attachments.clone());
}

fn construct_contract(
    id: Option<String>,
    title: String,
    partner: String,
    start: DateTime<Local>,
    state: String,
    term: f64,
    notice: f64,
    management: String,
    payment_first: DateTime<Local>,
    payment_amount: f64,
    payment_fix: bool,
    payment_cycle: f64,
    payment_kind: String,
    payment_pod: String,
    payment_debts: HashMap<String, f64>,
    attachments: Vec<String>,
) -> Contract {
    Contract {
        id,
        title,
        partner,
        start,
        state: match state.as_str() {
            "Terminated" => State::Terminated,
            "Expired" => State::Expired,
            _ => State::Active,
        },
        term: term as u32,
        notice: notice as u32,
        management,
        payment: Payment {
            first: payment_first,
            amount: payment_amount as u32,
            fix: payment_fix,
            cycle: payment_cycle as u32,
            kind: match payment_kind.as_str() {
                "Active" => PaymentKind::Active,
                "Credit" => PaymentKind::Credit,
                "PayPal" => PaymentKind::PayPal,
                "GooglePay" => PaymentKind::GooglePay,
                _ => PaymentKind::Debit,
            },
            pod: payment_pod,
            debts: payment_debts
                .iter()
                .map(|(key, value)| (key.to_string(), *value as u32))
                .collect(),
        },
        attachments,
    }
}

#[function_component]
pub fn ContractPage(props: &Props) -> Html {
    let pods: UseStateHandle<Vec<String>> = use_state(|| Vec::default());
    let debts: UseStateHandle<Vec<String>> = use_state(|| Vec::default());
    let id = use_state(|| None);
    let title = use_state(|| String::default());
    let partner = use_state(|| String::default());
    let start = use_state(|| Local::now());
    let state = use_state(|| String::default()); // TODO enum
    let term = use_state(|| 0.);
    let notice = use_state(|| 0.);
    let management = use_state(|| String::default());
    let payment_first = use_state(|| Local::now());
    let payment_amount = use_state(|| 0.);
    let payment_fix = use_state(|| true);
    let payment_cycle = use_state(|| 0.);
    let payment_kind = use_state(|| String::default());
    let payment_pod = use_state(|| String::default());
    let payment_debts = use_state(|| HashMap::default());
    let attachments = use_state(|| Vec::default());

    {
        let pods = pods.clone();
        let debts = debts.clone();
        let id = id.clone();
        let title = title.clone();
        let partner = partner.clone();
        let start = start.clone();
        let state = state.clone();
        let term = term.clone();
        let notice = notice.clone();
        let management = management.clone();
        let payment_first = payment_first.clone();
        let payment_amount = payment_amount.clone();
        let payment_fix = payment_fix.clone();
        let payment_cycle = payment_cycle.clone();
        let payment_kind = payment_kind.clone();
        let payment_pod = payment_pod.clone();
        let payment_debts = payment_debts.clone();
        let attachments = attachments.clone();
        let query = format!("/api/v1/contracts/{}", props.id);
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_contract: Contract = Request::get(&query)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                destruct_contract(
                    &fetched_contract,
                    id,
                    title,
                    partner,
                    start,
                    state,
                    term,
                    notice,
                    management,
                    payment_first,
                    payment_amount,
                    payment_fix,
                    payment_cycle,
                    payment_kind,
                    payment_pod,
                    payment_debts,
                    attachments,
                );
                let fetched_pods: Vec<String> = Request::get("/api/v1/transactions/pods")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                pods.set(fetched_pods);
                let fetched_debts: Vec<String> = Request::get("/api/v1/transactions/debts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                debts.set(fetched_debts);
            });
            || ()
        });
    }

    let update = {
        let id = id.clone();
        let title = title.clone();
        let partner = partner.clone();
        let start = start.clone();
        let state = state.clone();
        let term = term.clone();
        let notice = notice.clone();
        let management = management.clone();
        let payment_first = payment_first.clone();
        let payment_amount = payment_amount.clone();
        let payment_fix = payment_fix.clone();
        let payment_cycle = payment_cycle.clone();
        let payment_kind = payment_kind.clone();
        let payment_pod = payment_pod.clone();
        let payment_debts = payment_debts.clone();
        let attachments = attachments.clone();
        move |_: MouseEvent| {
            let contracts = vec![construct_contract(
                (*id).clone(),
                (*title).clone(),
                (*partner).clone(),
                *start,
                (*state).clone(),
                *term,
                *notice,
                (*management).clone(),
                *payment_first,
                *payment_amount,
                *payment_fix,
                *payment_cycle,
                (*payment_kind).clone(),
                (*payment_pod).clone(),
                (*payment_debts).clone(),
                (*attachments).clone(),
            )];
            wasm_bindgen_futures::spawn_local(async move {
                let _: Vec<Contract> = Request::put("/api/v1/contracts")
                    .json(&contracts)
                    .unwrap()
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
            });
        }
    };

    html! {
        <div class={Style::new(include_str!("contract.css")).expect("Unwrapping CSS should work!")}>
            <table>
                <tr>
                    <td>{"Title"}</td>
                    <td>
                        <StringInput
                            bind_handle={title}
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Partner"}</td>
                    <td>
                        <StringInput
                            bind_handle={partner}
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Start"}</td>
                    <td>
                        <DateInput
                            bind_handle={start}
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"State"}</td>
                    <td>
                        <StringInput
                            bind_handle={state}
                            options={vec!["Active".into(), "Terminated".into(), "Expired".into()]}
                            strict=true
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Term"}</td>
                    <td>
                        <NumberInput
                            bind_handle={term}
                            min=0.
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Notice"}</td>
                    <td>
                        <NumberInput
                            bind_handle={notice}
                            min=0.
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Management"}</td>
                    <td>
                        <StringInput
                            bind_handle={management}
                        />
                    </td>
                </tr>
                <tr>
                    <td colspan="2">{"Payment"}</td>
                </tr>
                <tr>
                    <td>{"First"}</td>
                    <td>
                        <DateInput
                            bind_handle={payment_first}
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Amount"}</td>
                    <td>
                        <NumberInput
                            bind_handle={payment_amount.clone()}
                            min=0.
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Fix"}</td>
                    <td>
                        <BoolInput
                            bind_handle={payment_fix}
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Cycle"}</td>
                    <td>
                        <NumberInput
                            bind_handle={payment_cycle}
                            min=0.
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Kind"}</td>
                    <td>
                        <StringInput
                            bind_handle={payment_kind}
                            options={vec!["Active".into(), "Debit".into(), "Credit".into(), "PayPal".into(), "GooglePay".into()]}
                            strict=true
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Pod"}</td>
                    <td>
                        <StringInput
                            bind_handle={payment_pod}
                            options={(*pods).clone()}
                        />
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        {"Debts"}
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        <StringNumberMap
                            bind_handle={payment_debts}
                            min=0.
                            max={*payment_amount}
                            options={(*debts).clone()}
                        />
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        {"Attachments"}
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        <RemoteFileInput<Attachment>
                            bind_handle={attachments}
                            endpoint="/api/v1/attachments"
                        />
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        <button onclick={update} class="update">{"Update"}</button>
                    </td>
                </tr>
           </table>
        </div>
    }
}
