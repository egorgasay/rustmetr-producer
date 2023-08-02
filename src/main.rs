
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,
    middleware::Logger};
use rustmetric::application::metrics::metrics::Metrics;
use rustmetric::application::usecases::usecase::UseCase;
use rustmetric::application::repositories::map::map_storage::Storage;
use rustmetric::domain::entity::{Metric, MetricKind};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
   let storage: Storage = Storage::new();
   let mut logic = UseCase::new(Box::new(storage));

   logic.start();

   Ok(())
}