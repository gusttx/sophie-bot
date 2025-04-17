use crate::{database::Db, get_config};
use poise::serenity_prelude::UserId;
use sqlx::{mysql::MySqlRow, Error, FromRow, Result, Row};

#[derive(Clone)]
pub struct User {
    pub user_id: UserId,
    pub coins: u32,
}

impl<'r> FromRow<'r, MySqlRow> for User {
    fn from_row(row: &'r MySqlRow) -> Result<Self> {
        let user_id: u64 = row.try_get("user_id")?;
        let coins: u32 = row.try_get("coins")?;

        if user_id == 0 {
            return Err(Error::ColumnDecode {
                index: "user_id".to_string(),
                source: "user id cannot be 0".into(),
            });
        }

        Ok(User {
            user_id: UserId::new(user_id),
            coins,
        })
    }
}

impl User {
    pub async fn get_or_create(db: &Db, user_id: UserId) -> Result<Self> {
        sqlx::query("INSERT IGNORE INTO users (user_id, coins) VALUES (?, ?)")
            .bind(user_id.get())
            .bind(get_config().economy.initial_coins)
            .execute(&db.pool)
            .await?;

        sqlx::query_as::<_, User>("SELECT user_id, coins FROM users WHERE user_id = ?")
            .bind(user_id.get())
            .fetch_one(&db.pool)
            .await
    }

    pub async fn set_coins(db: &Db, user_id: UserId, coins: u32) -> Result<()> {
        sqlx::query("INSERT INTO users (user_id, coins) VALUES (?, ?) ON DUPLICATE KEY UPDATE coins = ?")
            .bind(user_id.get())
            .bind(coins)
            .bind(coins)
            .execute(&db.pool)
            .await
            .map(|_| ())
    }

    pub async fn add_coins(db: &Db, user_id: UserId, coins: u32) -> Result<()> {
        sqlx::query("UPDATE users SET coins = LEAST(4294967295, coins + ?) WHERE user_id = ?")
            .bind(coins)
            .bind(user_id.get())
            .execute(&db.pool)
            .await
            .map(|_| ())
    }

    pub async fn sub_coins(db: &Db, user_id: UserId, coins: u32) -> Result<()> {
        sqlx::query("UPDATE users SET coins = GREATEST(0, coins - ?) WHERE user_id = ?")
            .bind(coins)
            .bind(user_id.get())
            .execute(&db.pool)
            .await
            .map(|_| ())
    }

    pub async fn transaction_update(db: &Db, users: Vec<Self>) -> Result<()> {
        let mut tx = db.get_tx().await?;

        for user in users {
            sqlx::query("UPDATE users SET coins = ? WHERE user_id = ?")
                .bind(user.coins)
                .bind(user.user_id.get())
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;
        Ok(())
    }

    pub async fn update(&self, db: &Db) -> Result<()> {
        sqlx::query("UPDATE users SET coins = ? WHERE user_id = ?")
            .bind(self.coins)
            .bind(self.user_id.get())
            .execute(&db.pool)
            .await
            .map(|_| ())
    }

    pub async fn send_coins(&mut self, db: &Db, receiver: UserId, qnt: u32) -> Result<()> {
        let mut tx = db.get_tx().await?;

        let new_coins = self.coins.saturating_sub(qnt);
        sqlx::query("UPDATE users SET coins = ? WHERE user_id = ?")
            .bind(new_coins)
            .bind(self.user_id.get())
            .execute(&mut *tx)
            .await?;

        sqlx::query(
            "
            INSERT INTO users (user_id, coins) VALUES (?, ?)
            ON DUPLICATE KEY UPDATE coins = coins + ?
            ",
        )
        .bind(receiver.get())
        .bind(get_config().economy.initial_coins + qnt)
        .bind(qnt)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        self.coins = new_coins;

        Ok(())
    }
}
