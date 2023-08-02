use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use crate::domain::entity::Metric;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn set(&self, metric: &Metric, id: usize);
    fn inc(&self, metric: &Metric, id: usize);
    fn get_all(&self) -> Vec<Metric>;
    fn get_by_id(&self, id: usize) -> Metric;
}
//
//#[cfg_attr(test, automock)]
//#[async_trait(?Send)]
//pub trait ValueAbstract: Send + Sync {
//    fn add(&mut self, another: &dyn ValueAbstract);
//}

//
//unsafe impl Sync for dyn CatFactsRepositoryAbstract {}