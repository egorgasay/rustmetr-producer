use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use crate::domain::entity::Metric;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn set_gauge(&self, metric: &Metric, id: usize);
    fn set_counter(&self, metric: &Metric, id: usize);
    fn get_counter_by_id(&self, id: usize) -> Metric;
    fn get_gauge_by_id(&self, id: usize) -> Metric;
}
