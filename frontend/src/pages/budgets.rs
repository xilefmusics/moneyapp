use crate::components::ListItem;
use crate::Route;
use fancy_yew::components::{Chart, ConfigBuilder};
use money_app_shared::history::AssociatedTypeValues;

use chrono::{Datelike, Local};
use gloo::net::http::Request;
use serde::Deserialize;
use std::collections::HashMap;
use stylist::Style;
use url::Url;
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Deserialize, Debug, Clone, Default)]
pub struct Query {
    pub start: Option<String>,
    pub end: Option<String>,
}
impl Query {
    pub fn api_url(&self, t: &str) -> String {
        let base = Url::parse("https://example.net").unwrap();
        let mut url = Url::parse(&format!("https://example.net/api/v1/history/{}", t)).unwrap();
        {
            let mut query_pairs = url.query_pairs_mut();
            query_pairs.append_pair(
                "start",
                &self
                    .start
                    .clone()
                    .unwrap_or(Local::now().year().to_string()),
            );
            if let Some(end) = &self.end {
                query_pairs.append_pair("end", end);
            }
        }
        base.make_relative(&url).unwrap().to_string()
    }
}
#[function_component]
pub fn Budgets() -> Html {
    let query = use_location()
        .unwrap()
        .query::<Query>()
        .unwrap_or(Query::default());

    let budget_history = use_state(|| vec![]);
    {
        let budget_history = budget_history.clone();
        let query = query.clone();
        use_effect_with((), move |_| {
            let budget_history = budget_history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_budget_history: Vec<AssociatedTypeValues> =
                    Request::get(&query.api_url("budgets"))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                budget_history.set(fetched_budget_history);
            });
            || ()
        });
    }

    let inbudget_history = use_state(|| vec![]);
    {
        let inbudget_history = inbudget_history.clone();
        use_effect_with((), move |_| {
            let inbudget_history = inbudget_history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_inbudget_history: Vec<AssociatedTypeValues> =
                    Request::get(&query.api_url("inbudgets"))
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                inbudget_history.set(fetched_inbudget_history);
            });
            || ()
        });
    }

    let navigator = use_navigator().unwrap();
    let list_items = if budget_history.len() > 0 {
        let mut tupels = budget_history[budget_history.len() - 1]
            .data
            .clone()
            .into_iter()
            .map(|(key, value)| (key, value.value))
            .filter(|(_, value)| *value != 0)
            .collect::<Vec<(String, i64)>>();
        tupels.sort_by(|a, b| a.0.cmp(&b.0));
        Some(
            tupels
                .iter()
                .map(|(key, value)| {
                    let onfilter = {
                        let name = key.clone();
                        let navigator = navigator.clone();
                        Callback::from(move |_: MouseEvent| {
                            navigator
                                .push_with_query(
                                    &Route::Transactions,
                                    &([("budget", &name)]
                                        .iter()
                                        .cloned()
                                        .collect::<HashMap<_, _>>()),
                                )
                                .unwrap()
                        })
                    };
                    html! {<ListItem
                        title={key.clone()}
                        amount={value}
                        color_amount={-value}
                        onfilter={onfilter}
                    />}
                })
                .collect::<Vec<Html>>(),
        )
    } else {
        None
    };

    let list_items_in = if inbudget_history.len() > 0 {
        let mut tupels = inbudget_history[inbudget_history.len() - 1]
            .data
            .clone()
            .into_iter()
            .map(|(key, value)| (key, value.value))
            .filter(|(_, value)| *value != 0)
            .collect::<Vec<(String, i64)>>();
        tupels.sort_by(|a, b| a.0.cmp(&b.0));
        Some(
            tupels
                .iter()
                .map(|(key, value)| {
                    let onfilter = {
                        let name = key.clone();
                        let navigator = navigator.clone();
                        Callback::from(move |_: MouseEvent| {
                            navigator
                                .push_with_query(
                                    &Route::Transactions,
                                    &([("inbudget", &name)]
                                        .iter()
                                        .cloned()
                                        .collect::<HashMap<_, _>>()),
                                )
                                .unwrap()
                        })
                    };
                    html! {<ListItem
                        title={key.clone()}
                        amount={value}
                        onfilter={onfilter}
                    />}
                })
                .collect::<Vec<Html>>(),
        )
    } else {
        None
    };

    let chart_config = if budget_history.len() > 0 {
        let (labels, dataset): (Vec<String>, Vec<f64>) = budget_history[0]
            .data
            .iter()
            .map(|(key, value)| (key.to_string(), value.value as f64 / 100.))
            .unzip();
        Some(
            ConfigBuilder::doughnut()
                .labels(&labels)
                .dataset(&dataset)
                .build()
                .unwrap(),
        )
    } else {
        None
    };

    let chart_config_in = if inbudget_history.len() > 0 {
        let (labels, dataset): (Vec<String>, Vec<f64>) = inbudget_history[0]
            .data
            .iter()
            .map(|(key, value)| (key.to_string(), value.value as f64 / 100.))
            .unzip();
        Some(
            ConfigBuilder::doughnut()
                .labels(&labels)
                .dataset(&dataset)
                .build()
                .unwrap(),
        )
    } else {
        None
    };

    html! {
        <div class={Style::new(include_str!("budgets.css")).expect("Unwrapping CSS should work!")}>
            {
                if let Some(chart_config) = chart_config {
                    html! {
                        <div class="chart-wrapper">
                            <Chart config={chart_config}/>
                        </div>
                    }
                } else {
                    html! {}

                }

            }
            <ul>
                {list_items}
            </ul>
            {
                if let Some(chart_config) = chart_config_in {
                    html! {
                        <div class="chart-wrapper">
                            <Chart config={chart_config}/>
                        </div>
                    }
                } else {
                    html! {}

                }

            }
            <ul>
                {list_items_in}
            </ul>
        </div>
    }
}
