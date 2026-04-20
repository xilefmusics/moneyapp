use fancy_yew::components::input::{DateInput, NumberInput, StringInput};
use money_app_shared::goal::{Goal, GoalData};

use chrono::{DateTime, Local};
use gloo::net::http::Request;
use stylist::Style;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: String,
}

fn destruct_goal(
    goal: &Goal,
    id_handle: UseStateHandle<Option<String>>,
    due_handle: UseStateHandle<DateTime<Local>>,
    goal_type_handle: UseStateHandle<String>,
    amount_handle: UseStateHandle<f64>,
) {
    id_handle.set(goal.id.clone());
    due_handle.set(goal.due);
    match goal.data {
        GoalData::RealWealth(amount) => {
            goal_type_handle.set("RealWealth".into());
            amount_handle.set(amount as f64);
        }
    }
}

fn construct_goal(
    id: Option<String>,
    due: DateTime<Local>,
    goal_type: String,
    amount: f64,
) -> Goal {
    Goal {
        id,
        due,
        data: match goal_type {
            _ => GoalData::RealWealth(amount as u32),
        },
    }
}

#[function_component]
pub fn GoalPage(props: &Props) -> Html {
    let id = use_state(|| None);
    let due = use_state(|| Local::now());
    let goal_type = use_state(|| String::default());
    let amount = use_state(|| 0.);

    {
        let id = id.clone();
        let due = due.clone();
        let goal_type = goal_type.clone();
        let amount = amount.clone();
        let query = format!("/api/v1/goals/{}", props.id);
        use_effect_with((), move |_| {
            let id = id.clone();
            let due = due.clone();
            let goal_type = goal_type.clone();
            let amount = amount.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_goal: Goal = Request::get(&query)
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                destruct_goal(&fetched_goal, id, due, goal_type, amount);
            });
            || ()
        });
    }

    let update = {
        let id = id.clone();
        let due = due.clone();
        let goal_type = goal_type.clone();
        let amount = amount.clone();
        move |_: MouseEvent| {
            let goals = vec![construct_goal(
                (*id).clone(),
                *due,
                (*goal_type).clone(),
                *amount,
            )];
            wasm_bindgen_futures::spawn_local(async move {
                let _: Vec<Goal> = Request::put("/api/v1/goals")
                    .json(&goals)
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
        <div class={Style::new(include_str!("goal.css")).expect("Unwrapping CSS should work!")}>
            <table>
                <tr>
                    <td>{"Due Date"}</td>
                    <td>
                        <DateInput
                            bind_handle={due.clone()}
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Type"}</td>
                    <td>
                        <StringInput
                            bind_handle={goal_type.clone()}
                            options={vec!["RealWealth".into()]}
                            strict=true
                        />
                    </td>
                </tr>
                <tr>
                    <td>{"Amount"}</td>
                    <td>
                        <NumberInput
                            bind_handle={amount.clone()}
                            min={0.}
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
