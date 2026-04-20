use super::DateIterator;

use crate::database::Database;
use crate::error::AppError;
use crate::history::{History, QueryParams};
use crate::resources::contract::store::ContractStore;
use crate::resources::goal::store::GoalStore;

use money_app_shared::contract::Contract;
use money_app_shared::extrapolation::{Extrapolation, ExtrapolationItem};
use money_app_shared::goal::{Goal, GoalData};
use money_app_shared::history::Wealth;

use chrono::Datelike;
use std::sync::Arc;

fn savings_rate(goals: &[Goal], wealth: &Wealth) -> u32 {
    let max = goals
        .iter()
        .map(|goal| {
            let months = goal.due.month() as i64 - wealth.date.month() as i64
                + (goal.due.year() as i64 - wealth.date.year() as i64) * 12;
            if months > 0 {
                match goal.data {
                    GoalData::RealWealth(goal) => (goal as i64 - wealth.real.value) / months,
                }
            } else {
                0
            }
        })
        .max()
        .unwrap_or(0);
    if max > 0 {
        max as u32
    } else {
        0
    }
}

pub fn create_extrapolation_pure(
    contracts: &[Contract],
    goals: &[Goal],
    history_wealth: &[Wealth],
    current_wealth: &Wealth,
    year: i32,
    month: u32,
) -> Extrapolation {
    let savings_rate = savings_rate(goals, current_wealth);
    let normal = history_wealth
        .iter()
        .map(|item| ExtrapolationItem::from(item.clone()))
        .chain(
            DateIterator::new(year, month)
                .map(|date| ExtrapolationItem::new(contracts, savings_rate, date)),
        )
        .collect::<Vec<ExtrapolationItem>>();

    let freely_available = normal.iter().map(|item| item.freely_available).sum::<i64>();
    let planned_savings = normal.iter().map(|item| item.planned_savings).sum::<u32>();

    let equalized_free_money = {
        let freely_available = std::cmp::min(
            freely_available / 12,
            normal
                .iter()
                .map(|item| item.max_freely_available())
                .min()
                .unwrap_or(0),
        );
        normal
            .iter()
            .map(|item| item.equalize_freely_available(freely_available))
            .collect::<Vec<ExtrapolationItem>>()
    };

    Extrapolation {
        date: normal[0].date.clone(),
        freely_available,
        planned_savings,
        normal,
        equalized_free_money,
    }
}

pub async fn create_extrapolation(
    db: Arc<Database>,
    user: &str,
    year: i32,
    month: u32,
) -> Result<Extrapolation, AppError> {
    let contracts = ContractStore::list(db.clone(), user).await?;
    let goals = GoalStore::list(db.clone(), user).await?;
    let wealth = History::wealth(
        db.clone(),
        user,
        QueryParams {
            year: None,
            month: Some(1),
            day: None,
            len: Some(month + 1),
            start: None,
            end: None,
        },
    )
    .await?;

    if wealth.len() < 2 {
        return Err(AppError::invalid_request(
            "not enough history for extrapolation (need at least 2 wealth points)",
        ));
    }

    let n = wealth.len();
    Ok(create_extrapolation_pure(
        &contracts,
        &goals,
        &wealth[..(n - 2)],
        &wealth[n - 2],
        year,
        month,
    ))
}
