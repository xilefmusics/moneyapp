use fancy_yew::components::{Chart, ConfigBuilder};
use money_app_shared::history::Wealth;

use gloo::net::http::Request;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub year: i64,
    pub month: i64,
    pub day: i64,
    pub len: i64,
}

#[function_component]
pub fn WealthHistory(props: &Props) -> Html {
    let data = use_state(|| vec![]);
    {
        let data = data.clone();
        let query = format!(
            "/api/v1/history/wealth?year={}&month={}&day={}&len={}",
            props.year, props.month, props.day, props.len
        );
        use_effect_with((), move |_| {
            let data = data.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_data: Vec<Wealth> = Request::get(&query)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                data.set(fetched_data);
            });
            || ()
        });
    }

    let chart_config = if data.len() > 0 {
        Some(
            ConfigBuilder::line()
                .labels(
                    &data
                        .iter()
                        .map(|item| item.date.format("%b-%Y").to_string())
                        .collect::<Vec<String>>(),
                )
                .dataset(
                    &data
                        .iter()
                        .map(|item| item.sum.value as f64 / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_label("Sum")
                .dataset_border_width(1)
                .dataset_border_color_str("blue")
                .dataset_background_color_str("blue")
                .dataset(
                    &data
                        .iter()
                        .map(|item| item.real.value as f64 / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_label("Real")
                .dataset_border_width(1)
                .dataset_border_color_str("lightgreen")
                .dataset_background_color_str("lightgreen")
                .dataset(
                    &data
                        .iter()
                        .map(|item| item.debt.value as f64 / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_label("Debt")
                .dataset_border_width(1)
                .dataset_border_color_str("purple")
                .dataset_background_color_str("purple")
                .dataset(
                    &data
                        .iter()
                        .map(|item| item.income.value as f64 / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_label("In")
                .dataset_border_width(1)
                .dataset_border_color_str("green")
                .dataset_background_color_str("green")
                .dataset(
                    &data
                        .iter()
                        .map(|item| item.out.value as f64 / 100.)
                        .collect::<Vec<f64>>(),
                )
                .dataset_label("Out")
                .dataset_border_width(1)
                .dataset_border_color_str("red")
                .dataset_background_color_str("red")
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
