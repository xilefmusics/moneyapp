use crate::components::ListItem;
use crate::Route;
use fancy_yew::components::{Chart, ConfigBuilder};
use money_app_shared::history::{AssociatedTypeValues, ValueDiff};

use gloo::net::http::Request;
use std::collections::HashMap;
use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Pods() -> Html {
    let year = 0;
    let month = 3;
    let day = 0;
    let len = 13;
    let query = format!(
        "/api/v1/history/pods?year={}&month={}&day={}&len={}",
        year, month, day, len
    );

    let pod_history = use_state(|| vec![]);
    {
        let pod_history = pod_history.clone();
        use_effect_with((), move |_| {
            let pod_history = pod_history.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_pod_history: Vec<AssociatedTypeValues> = Request::get(&query)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                pod_history.set(fetched_pod_history);
            });
            || ()
        });
    }

    let navigator = use_navigator().unwrap();
    let list_items = if pod_history.len() > 0 {
        let mut tupels = pod_history[pod_history.len() - 1]
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
                                    &([("pod", &name)].iter().cloned().collect::<HashMap<_, _>>()),
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

    let chart_config = if pod_history.len() > 0 {
        let labels = pod_history
            .iter()
            .map(|item| item.date.format("%b-%Y").to_string())
            .collect::<Vec<String>>();
        let mut config_builder = ConfigBuilder::line().labels(&labels);

        let datasets = pod_history[pod_history.len() - 1]
            .data
            .keys()
            .map(|pod| {
                (
                    pod.to_string(),
                    pod_history
                        .iter()
                        .map(|item| {
                            item.data.get(pod).unwrap_or(&ValueDiff::default()).value as f64 / 100.
                        })
                        .collect::<Vec<f64>>(),
                )
            })
            .collect::<Vec<(String, Vec<f64>)>>();

        for dataset in &datasets {
            config_builder = config_builder
                .dataset(&dataset.1)
                .dataset_label(&dataset.0)
                .dataset_border_width(1);
        }

        Some(config_builder.build().unwrap())
    } else {
        None
    };

    html! {
        <div class={Style::new(include_str!("pods.css")).expect("Unwrapping CSS should work!")}>
            {
                if let Some(chart_config) = chart_config {
                    html! {
                        <Chart config={chart_config}/>
                    }
                } else {
                    html! {}

                }

            }
            <ul>
                {list_items}
            </ul>
        </div>
    }
}
