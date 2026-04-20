use fancy_yew::components::{Chart, ConfigBuilder};
use money_app_shared::extrapolation::Extrapolation;

use gloo::net::http::Request;
use yew::prelude::*;

#[function_component]
pub fn MonthlyExtrapolation() -> Html {
    let extrapolation = use_state(|| None);
    {
        let extrapolation = extrapolation.clone();
        use_effect_with((), move |_| {
            let extrapolation = extrapolation.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_extrapolation: Extrapolation = Request::get("/api/v1/extrapolation")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                extrapolation.set(Some(fetched_extrapolation));
            });
            || ()
        });
    }

    let chart_config = if let Some(extrapolation) = (*extrapolation).clone() {
        Some(
            ConfigBuilder::bar()
                .labels(
                    &extrapolation
                        .normal
                        .iter()
                        .map(|item| item.date.format("%b").to_string())
                        .collect::<Vec<String>>(),
                )
                .dataset(
                    &extrapolation
                        .normal
                        .iter()
                        .map(|item| (item.contract_expenses as f64) / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_stack(0)
                .dataset_label("Expenses")
                .dataset_border_width(1)
                .dataset_border_color_str("orange")
                .dataset_background_color_str("orange")
                .dataset(
                    &extrapolation
                        .normal
                        .iter()
                        .map(|item| (item.planned_savings as f64) / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_stack(0)
                .dataset_label("Savings")
                .dataset_border_width(1)
                .dataset_border_color_str("blue")
                .dataset_background_color_str("blue")
                .dataset(
                    &extrapolation
                        .normal
                        .iter()
                        .map(|item| (item.freely_available as f64) / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_stack(0)
                .dataset_label("Free")
                .dataset_border_width(1)
                .dataset_border_color_str("green")
                .dataset_background_color_str("green")
                .dataset(
                    &extrapolation
                        .normal
                        .iter()
                        .map(|item| (item.actual_expenses as f64) / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_stack(0)
                .dataset_label("Spent")
                .dataset_border_width(1)
                .dataset_border_color_str("red")
                .dataset_background_color_str("red")
                .dataset(
                    &extrapolation
                        .normal
                        .iter()
                        .map(|item| (item.actual_overspent as f64) / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_stack(0)
                .dataset_label("Over-Spent")
                .dataset_border_width(1)
                .dataset_border_color_str("darkred")
                .dataset_background_color_str("darkred")
                .dataset(
                    &extrapolation
                        .normal
                        .iter()
                        .map(|item| (item.actual_savings as f64) / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_stack(0)
                .dataset_label("Saved")
                .dataset_border_width(1)
                .dataset_border_color_str("green")
                .dataset_background_color_str("green")
                .build()
                .unwrap(),
        )
    } else {
        None
    };

    if let Some(chart_config) = chart_config {
        html! {
            <Chart config={chart_config}/>
        }
    } else {
        html! {}
    }
}
