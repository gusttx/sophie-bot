mod model {
    pub mod user;
}
pub use model::user::User;

use sqlx::{MySqlPool, Transaction, MySql, Result};
use std::marker::PhantomData;

pub struct Initialized;
pub struct Uninitialized;

pub struct Database<State = Uninitialized> {
    pub pool: MySqlPool,
    state: PhantomData<State>,
}

impl Database<Uninitialized> {
    pub async fn new(database_url: impl AsRef<str>) -> Result<Self> {
        Ok(Self {
            pool: MySqlPool::connect(database_url.as_ref()).await?,
            state: PhantomData,
        })
    }

    pub async fn init(self) -> Result<Database<Initialized>> {
        sqlx::migrate!("./migrations")
            .run(&self.pool)
            .await?;

        Ok(Database {
            pool: self.pool,
            state: PhantomData,
        })
    }
}

impl Database<Initialized> {
    pub async fn get_tx(&self) -> Result<Transaction<MySql>> {
        self.pool.begin().await
    }
}

pub type Db = Database<Initialized>;