use sea_orm::{ConnectOptions, Database, DatabaseConnection};
use crate::error::Error;

pub trait GlobalDatabase {
    fn get_database(&self) -> Option<&DatabaseConnection>;
    async fn get_or_try_init_database(&self) -> Result<&DatabaseConnection, Error>;
    async fn get_or_try_init_database_with_connect_options<T>(&self, options: T) -> Result<&DatabaseConnection, Error> where
    T: Into<ConnectOptions>;
}