use crate::components::ListItem;
use crate::Route;
use fancy_yew::components::ResourceHeader;
use fancy_yew::rest::Resource;
use money_app_shared::goal::{Goal, GoalData};

use stylist::Style;
use yew::prelude::*;
use yew_router::prelude::*;

#[function_component]
pub fn Goals() -> Html {
    let goals: UseStateHandle<Resource<Goal>> =
        use_state(|| Resource::new("/api/v1/goals".into(), "goals".into(), Vec::default()));
    {
        let goals = goals.clone();
        use_effect_with((), move |_| {
            Resource::reload(goals);
            || ()
        });
    }

    let navigator = use_navigator().unwrap();
    let goal_items = goals
        .iter()
        .map(|goal| match goal.data {
            GoalData::RealWealth(wealth) => {
                let navigator = navigator.clone();
                let id = goal.id.clone().unwrap();
                let onedit = {
                    let id = id.clone();
                    Callback::from(move |_: MouseEvent| {
                        navigator.push(&Route::Goal { id: id.clone() })
                    })
                };
                html! {<ListItem
                    title={"Wealth"}
                    subtitle={goal.due.format("%d %b %Y").to_string()}
                    amount={wealth as i64}
                    onedit={onedit}
                    ondelete={Resource::delete_callback(goal.clone(),goals.clone())}
                />}
            }
        })
        .collect::<Vec<Html>>();

    html! {
        <div class={Style::new(include_str!("goals.css")).expect("Unwrapping CSS should work!")}>
            <ResourceHeader<Goal> handle={goals.clone()} />
            <ul>
                {goal_items}
            </ul>
        </div>
    }
}
