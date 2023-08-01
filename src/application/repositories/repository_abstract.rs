use async_trait::async_trait;

#[cfg(test)]
use mockall::{predicate::*, *};
use crate::domain::entity::Metric;

#[cfg_attr(test, automock)]
#[async_trait(?Send)]
pub trait RepositoryAbstract: Send + Sync {
    fn set(&self, metric: String, value: f32);
    fn get_all(&self) -> [Metric; 28];
}
//
//#[cfg_attr(test, automock)]
//#[async_trait(?Send)]
//pub trait ValueAbstract: Send + Sync {
//    fn add(&mut self, another: &dyn ValueAbstract);
//}

//
//unsafe impl Sync for dyn CatFactsRepositoryAbstract {}