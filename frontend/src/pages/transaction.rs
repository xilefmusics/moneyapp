use fancy_yew::components::input::{
    DateInput, NumberInput, RemoteFileInput, StringInput, StringNumberMap, StringStringMap,
};
use money_app_shared::attachment::Attachment;
use money_app_shared::transaction::{Transaction, Type};

use chrono::{DateTime, Local};
use gloo::net::http::Request;
use std::collections::HashMap;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

fn destruct_transaction(
    transaction: &Transaction,
    id_handle: UseStateHandle<Option<String>>,
    ttype_handle: UseStateHandle<String>,
    date_handle: UseStateHandle<DateTime<Local>>,
    amount_handle: UseStateHandle<f64>,
    sender_handle: UseStateHandle<String>,
    receiver_handle: UseStateHandle<String>,
    budgets_handle: UseStateHandle<HashMap<String, f64>>,
    inbudgets_handle: UseStateHandle<HashMap<String, f64>>,
    debts_handle: UseStateHandle<HashMap<String, f64>>,
    tags_handle: UseStateHandle<HashMap<String, String>>,
    attachments_handle: UseStateHandle<Vec<String>>,
) {
    id_handle.set(transaction.id.clone());
    ttype_handle.set(match transaction.ttype {
        Type::In => "In".into(),
        Type::Out => "Out".into(),
        Type::Move => "Move".into(),
    });
    date_handle.set(transaction.date);
    amount_handle.set(transaction.amount as f64);
    sender_handle.set(transaction.sender.clone().unwrap_or("".into()));
    receiver_handle.set(transaction.receiver.clone().unwrap_or("".into()));
    budgets_handle.set(
        transaction
            .budgets
            .iter()
            .map(|(key, value)| (key.to_string(), *value as f64))
            .collect::<HashMap<String, f64>>(),
    );
    inbudgets_handle.set(
        transaction
            .inbudgets
            .iter()
            .map(|(key, value)| (key.to_string(), *value as f64))
            .collect::<HashMap<String, f64>>(),
    );
    debts_handle.set(
        transaction
            .debts
            .iter()
            .map(|(key, value)| (key.to_string(), *value as f64))
            .collect::<HashMap<String, f64>>(),
    );
    tags_handle.set(transaction.tags.clone());
    attachments_handle.set(transaction.attachments.clone());
}

fn construct_transaction(
    id: Option<String>,
    ttype: String,
    date: DateTime<Local>,
    amount: f64,
    sender: String,
    receiver: String,
    budgets: HashMap<String, f64>,
    inbudgets: HashMap<String, f64>,
    debts: HashMap<String, f64>,
    tags: HashMap<String, String>,
    attachments: Vec<String>,
) -> Transaction {
    Transaction {
        id,
        ttype: match ttype.as_ref() {
            "In" => Type::In,
            "Move" => Type::Move,
            _ => Type::Out,
        },
        date,
        amount: amount as usize,
        sender: if ttype != "In" { Some(sender) } else { None },
        receiver: if ttype != "Out" { Some(receiver) } else { None },
        budgets: if ttype != "In" {
            budgets
                .iter()
                .map(|(key, value)| (key.to_string(), *value as usize))
                .collect::<HashMap<String, usize>>()
        } else {
            HashMap::default()
        },
        inbudgets: if ttype != "Out" {
            inbudgets
                .iter()
                .map(|(key, value)| (key.to_string(), *value as usize))
                .collect::<HashMap<String, usize>>()
        } else {
            HashMap::default()
        },
        debts: debts
            .iter()
            .map(|(key, value)| (key.to_string(), *value as usize))
            .collect::<HashMap<String, usize>>(),
        tags,
        attachments: attachments.to_owned(),
    }
}

#[function_component]
pub fn TransactionPage(props: &Props) -> Html {
    let pod_types: UseStateHandle<Vec<String>> = use_state(|| Vec::default());
    let budget_types: UseStateHandle<Vec<String>> = use_state(|| Vec::default());
    let inbudget_types: UseStateHandle<Vec<String>> = use_state(|| Vec::default());
    let debt_types: UseStateHandle<Vec<String>> = use_state(|| Vec::default());
    let tag_types: UseStateHandle<Vec<String>> = use_state(|| Vec::default());
    let id = use_state(|| None);
    let ttype = use_state(|| String::default());
    let date = use_state(|| Local::now());
    let amount = use_state(|| 0.);
    let sender = use_state(|| String::default());
    let receiver = use_state(|| String::default());
    let budgets = use_state(|| HashMap::default());
    let inbudgets = use_state(|| HashMap::default());
    let debts = use_state(|| HashMap::default());
    let tags = use_state(|| HashMap::default());
    let attachments = use_state(|| Vec::default());
    {
        let pod_types = pod_types.clone();
        let budget_types = budget_types.clone();
        let inbudget_types = inbudget_types.clone();
        let debt_types = debt_types.clone();
        let tag_types = tag_types.clone();
        let id = id.clone();
        let ttype = ttype.clone();
        let date = date.clone();
        let amount = amount.clone();
        let sender = sender.clone();
        let receiver = receiver.clone();
        let budgets = budgets.clone();
        let inbudgets = inbudgets.clone();
        let debts = debts.clone();
        let tags = tags.clone();
        let attachments = attachments.clone();
        let query = format!("/api/v1/transactions/{}", props.id);
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_transaction: Transaction = Request::get(&query)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                destruct_transaction(
                    &fetched_transaction,
                    id,
                    ttype,
                    date,
                    amount,
                    sender,
                    receiver,
                    budgets,
                    inbudgets,
                    debts,
                    tags,
                    attachments,
                );
                let fetched_pod_types: Vec<String> = Request::get("/api/v1/transactions/pods")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                pod_types.set(fetched_pod_types);
                let fetched_budget_types: Vec<String> = Request::get("/api/v1/transactions/budgets")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                budget_types.set(fetched_budget_types);
                let fetched_inbudget_types: Vec<String> = Request::get("/api/v1/transactions/inbudgets")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                inbudget_types.set(fetched_inbudget_types);
                let fetched_debt_types: Vec<String> = Request::get("/api/debts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                debt_types.set(fetched_debt_types);
                // TODO: fix
                // let fetched_tag_types: Vec<String> = Request::get("/api/v1/transactions/tags")
                //     .send()
                //     .await
                //     .unwrap()
                //     .json()
                //     .await
                //     .unwrap();
                // tag_types.set(fetched_tag_types);
            });
            || ()
        });
    }

    let update = {
        let id = id.clone();
        let ttype = ttype.clone();
        let date = date.clone();
        let amount = amount.clone();
        let sender = sender.clone();
        let receiver = receiver.clone();
        let budgets = budgets.clone();
        let inbudgets = inbudgets.clone();
        let debts = debts.clone();
        let tags = tags.clone();
        let attachments = attachments.clone();
        move |_: MouseEvent| {
            let transactions = vec![construct_transaction(
                (*id).clone(),
                (*ttype).clone(),
                *date,
                *amount,
                (*sender).clone(),
                (*receiver).clone(),
                (*budgets).clone(),
                (*inbudgets).clone(),
                (*debts).clone(),
                (*tags).clone(),
                (*attachments).clone(),
            )];
            wasm_bindgen_futures::spawn_local(async move {
                let _: Vec<Transaction> = Request::put("/api/v1/transactions")
                    .json(&transactions)
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
        <div class={Style::new(include_str!("transaction.css")).expect("Unwrapping CSS should work!")}>
            <table>
                <tr>
                    <td>{"Type"}</td>
                    <td>
                        <StringInput
                            bind_handle={ttype.clone()}
                            options={vec!["In".into(), "Out".into(), "Move".into()]}
                            strict=true
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Date"}</td>
                    <td>
                        <DateInput
                            bind_handle={date}
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Amount"}</td>
                    <td>
                        <NumberInput
                            bind_handle={amount.clone()}
                            min=0.
                        />
                    </td>
                </tr>
                {
                    if *ttype != "In" {html! {
                        <tr>
                            <td>{"Sender"}</td>
                            <td>
                                <StringInput
                                    bind_handle={sender}
                                    options={(*pod_types).clone()}
                                />
                            </td>
                        </tr>
                    }} else {
                        html!{}
                    }
                }
                {
                    if *ttype != "Out" {html! {
                        <tr>
                            <td>{"Receiver"}</td>
                            <td>
                                <StringInput
                                    bind_handle={receiver}
                                    options={(*pod_types).clone()}
                                />
                            </td>
                        </tr>
                    }} else {
                        html!{}
                    }
                }
                {
                    if *ttype == "Out" {html! {
                        <>
                            <tr>
                                <td colspan="2">
                                    {"Budgets"}
                                </td>
                            </tr>
                            <tr>
                                <td colspan="2">
                                    <StringNumberMap
                                        bind_handle={budgets}
                                        min=0.
                                        max={*amount}
                                        options={(*budget_types).clone()}
                                    />
                                </td>
                            </tr>
                        </>
                    }} else {
                        html!{}
                    }
                }
                {
                    if *ttype == "In" {html! {
                        <>
                            <tr>
                                <td colspan="2">
                                    {"Inbudgets"}
                                </td>
                            </tr>
                            <tr>
                                <td colspan="2">
                                    <StringNumberMap
                                        bind_handle={inbudgets}
                                        min=0.
                                        max={*amount}
                                        options={(*inbudget_types).clone()}
                                    />
                                </td>
                            </tr>
                        </>
                    }} else {
                        html!{}
                    }
                }
                <tr>
                    <td colspan="2">
                        {"Debts"}
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        <StringNumberMap
                            bind_handle={debts}
                            min=0.
                            max={*amount}
                            options={(*debt_types).clone()}
                        />
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        {"Tags"}
                    </td>
                </tr>
                <tr>
                    <td colspan="2">
                        <StringStringMap
                            bind_handle={tags}
                            options={(*tag_types).clone()}
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
