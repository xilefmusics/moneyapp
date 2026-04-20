use super::{AssociatedTypeValues, QueryParams, Wealth};

use crate::database::Database;
use crate::error::AppError;
use crate::resources::transaction::filter::Filter;
use crate::resources::transaction::store::TransactionStore;

use chrono::Local;
use std::sync::Arc;

pub struct History;

impl History {
    pub async fn wealth(
        db: Arc<Database>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<Wealth>, AppError> {
        let txs = TransactionStore::list(
            db,
            user,
            &Filter::default().start_option(query_params.start),
            false,
        )
        .await?;
        Ok(Wealth::history(
            txs,
            query_params.end.unwrap_or(Local::now()),
            query_params.year.unwrap_or(0),
            query_params.month.unwrap_or(0),
            query_params.day.unwrap_or(0),
            query_params.len.unwrap_or(1),
        ))
    }

    async fn associated_type_values(
        db: Arc<Database>,
        user: &str,
        query_params: QueryParams,
        t: &str,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        let raw = TransactionStore::associated_type_values(
            db,
            user,
            &Filter::default().start_option(query_params.start),
            t,
        )
        .await?;
        Ok(AssociatedTypeValues::history(
            raw,
            query_params.end.unwrap_or(Local::now()),
            query_params.year.unwrap_or(0),
            query_params.month.unwrap_or(0),
            query_params.day.unwrap_or(0),
            query_params.len.unwrap_or(1),
        ))
    }

    pub async fn pod(
        db: Arc<Database>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Self::associated_type_values(db, user, query_params, "pods").await
    }

    pub async fn budget(
        db: Arc<Database>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Self::associated_type_values(db, user, query_params, "budgets").await
    }

    pub async fn inbudget(
        db: Arc<Database>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Self::associated_type_values(db, user, query_params, "inbudgets").await
    }

    pub async fn debt(
        db: Arc<Database>,
        user: &str,
        query_params: QueryParams,
    ) -> Result<Vec<AssociatedTypeValues>, AppError> {
        Self::associated_type_values(db, user, query_params, "debts").await
    }
}
