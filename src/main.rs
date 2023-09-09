use rustmetric::application::usecases::usecase::UseCase;
use rustmetric::application::repositories::map::map_storage::Storage;
use tokio;

#[tokio::main]
async fn main() -> std::io::Result<()> {
   let storage: Storage = Storage::default();
   let mut logic = UseCase::new(Box::new(storage));

   let _ = logic.start().await;

   Ok(())
}