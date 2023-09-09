use derive_more::Display;

#[derive(Debug, Display)]
pub enum RepositoryError {
    #[display(fmt = "NotFound")]
    NotFound
}