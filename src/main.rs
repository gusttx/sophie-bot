mod utils;
mod commands;
mod config;
mod database;
mod env;
mod types;
mod framework;

use config::get_config;
use database::Database;
use env::get_env;
use framework::create_framework;
use poise::serenity_prelude::{ClientBuilder, GatewayIntents};
use types::Data;
use utils::redis::Redis;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file");
    env_logger::init();

    let data = create_data().await;
    let framework = create_framework(data);
    let token = &get_env().discord_token;

    ClientBuilder::new(token, GatewayIntents::all())
        .framework(framework)
        .await
        .expect("Failed to create client")
        .start()
        .await
        .expect("Failed to start client");
}

async fn create_data() -> Data {
    let db = Database::new(get_env().mysql.connection_string())
        .await
        .expect("Failed to create connection pool")
        .init()
        .await
        .expect("Failed to initialize database");

    let redis = Redis::new(get_env().redis.connection_string()).expect("Failed to initialize Redis");

    Data::new(db, redis)
}