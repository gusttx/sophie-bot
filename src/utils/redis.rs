use deadpool_redis::{Config, Connection, CreatePoolError, Pool, Runtime};
use log::warn;
use redis::{AsyncCommands, RedisFuture};

pub struct Redis {
    pool: Pool,
}

impl Redis {
    pub fn new(redis_url: impl Into<String>) -> Result<Self, CreatePoolError> {
        let cfg = Config::from_url(redis_url);
        Ok(Self {
            pool: cfg.create_pool(Some(Runtime::Tokio1))?,
        })
    }

    pub async fn get_conn(&self) -> Option<Connection> {
        match self.pool.get().await {
            Ok(conn) => Some(conn),
            Err(err) => {
                warn!("Failed to get connection from Redis pool: {}", err);
                None
            }
        }
    }

    pub async fn get<T: serde::de::DeserializeOwned>(&self, key: impl AsRef<str>) -> Option<T> {
        let key = key.as_ref();
        let mut conn = self.get_conn().await?;

        let result = match conn.get::<_, Option<String>>(key).await {
            Ok(Some(result)) => result,
            Ok(None) => return None,
            Err(err) => {
                warn!("Failed to get redis key '{}': {}", key, err);
                return None;
            }
        };

        serde_json::from_str(&result).map_err(|err| {
            warn!(
                "Failed to deserialize redis key '{}' into {}: {}",
                key, std::any::type_name::<T>(), err
            )
        }).ok()
    }

    pub async fn set<T: serde::ser::Serialize>(&self, key: impl AsRef<str>, value: &T, ttl: Option<u64>) {
        let key = key.as_ref();

        let value = match serde_json::to_string(value) {
            Ok(value) => value,
            Err(err) => {
                warn!(
                    "Failed to serialize redis key '{}' into {}: {}",
                    key, std::any::type_name::<T>(), err
                );
                return;
            }
        };

        let Some(mut conn) = self.get_conn().await else { return };
        let cmd: RedisFuture<String> = match ttl {
            Some(seconds) => conn.set_ex(key, &value, seconds),
            None => conn.set(key, &value),
        };

        if let Err(err) = cmd.await {
            warn!(
                "Failed to set redis key '{}' to value '{}': {}",
                key, value, err
            );
        }
    }

    // pub async fn get(&self, key: impl AsRef<str>) -> Option<String> {
    //     if let Some(mut conn) = self.get_conn().await {
    //         return conn.get(key.as_ref()).await.ok();
    //     }

    //     std::any::type_name()
    //     None
    // }

    // pub async fn set(&self, key: impl AsRef<str>, value: impl AsRef<str>, ttl: Option<u64>) {
    //     let key = key.as_ref();
    //     let value = value.as_ref();

    //     if let Some(mut conn) = self.get_conn().await {
    //         let cmd: RedisFuture<String> = match ttl {
    //             Some(seconds) => conn.set_ex(key, value, seconds),
    //             None => conn.set(key, value),
    //         };

    //         if let Err(err) = cmd.await {
    //             warn!(
    //                 "Failed to set redis key '{}' to value '{}': {}",
    //                 key, value, err
    //             );
    //         }
    //     }
    // }

    // pub async fn sadd(
    //     &self,
    //     key: impl AsRef<str>,
    //     values: &[&str]
    // ) {
    //     let key = key.as_ref();

    //     match self.pool.get().await {
    //         Ok(mut conn) => {
    //             let cmd: RedisFuture<u64> = conn.sadd(key, values);

    //             if let Err(err) = cmd.await {
    //                 warn!("Failed to add itens {:?} into redis set '{}': {}", values, key, err);
    //             }
    //         },
    //         Err(err) => {
    //             warn!("Failed to get connection from Redis pool: {}", err);
    //         }
    //     }
    // }

    // pub async fn sismember(
    //     &self,
    //     key: impl AsRef<str>,
    //     value: impl AsRef<str>
    // ) -> bool {
    //     let key = key.as_ref();
    //     let value = value.as_ref();

    //     match self.pool.get().await {
    //         Ok(mut conn) => {
    //             let cmd: RedisFuture<bool> = conn.sismember(key, value);

    //             match cmd.await {
    //                 Err(err) => {
    //                     warn!("Failed to add item '{}' into redis set '{}': {}", value, key, err);
    //                     return false;
    //                 }
    //                 Ok(result) => {
    //                     return result;
    //                 }
    //             }
    //         },
    //         Err(err) => {
    //             warn!("Failed to get connection from Redis pool: {}", err);
    //             return false;
    //         }
    //     }
    // }
}
