use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use crate::domain::entity::Metric;
use super::error::RepositoryError;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn set_gauge(&self, metric: &Metric, id: usize);
    fn inc_counter(&self, id: usize);
    fn get_counter_by_id(&self, id: usize) -> Result<Metric, RepositoryError>;
    fn get_gauge_by_id(&self, id: usize) -> Result<Metric, RepositoryError>;
    fn drop_all_counter(&mut self);
}
