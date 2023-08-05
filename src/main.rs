
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,
    middleware::Logger};
use rustmetric::application::usecases::usecase::UseCase;
use rustmetric::application::repositories::map::map_storage::Storage;
use rustmetric::domain::entity::{Metric, MetricKind};
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
   let storage: Storage = Storage::new();
   let mut logic = UseCase::new(Box::new(storage));

   logic.start().await;

   Ok(())
}