
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder,
    middleware::Logger};
use rustmetric::application::usecases::usecase::UseCase;
use rustmetric::application::repositories::map::map_storage::Storage;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

   let storage: Storage = Storage::new();
   let logic = UseCase::new(Box::new(storage));

   logic.start();

   Ok(())
}