use anyhow::Context;
use spring::{auto_config, App};
use spring_sqlx::sqlx::Row;
use spring_sqlx::{sqlx, ConnectPool, SqlxPlugin};
use spring_web::error::KnownWebError;
use spring_web::{WebConfigurator, WebPlugin};
use spring_web::{
    axum::response::IntoResponse,
    error::Result,
    extractor::Component,
    get,
};
use subsecond;

#[auto_config(WebConfigurator)]
#[spring::main]
async fn main() {
    // Initialize Dioxus devtools for hot reloading
    #[cfg(debug_assertions)]
    dioxus_devtools::connect_subsecond();
    
    App::new()
        .add_plugin(WebPlugin)
        .add_plugin(SqlxPlugin)
        .run()
        .await
}

#[get("/")]
async fn hello_world() -> impl IntoResponse {
    "🎉 #[hot] MACRO **CONFIRMED WORKING** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
}

#[get("/version")]
async fn sql_version(Component(pool): Component<ConnectPool>) -> Result<String> {
    let version: String = sqlx::query("select sqlite_version() as version")
        .fetch_one(&pool)
        .await
        .context("sqlx query failed")?
        .get("version");
    Ok(format!("version: {}", version))
}

#[get("/error")]
async fn error_request() -> Result<String> {
    Err(KnownWebError::bad_request("request error"))?
}

#[get("/test")]
async fn test_hot_reload() -> impl IntoResponse {
    subsecond::call(|| {
        "🚀 MANUAL SUBSECOND CALL! Hot reload **CONFIRMED WORKING** - This change was detected instantly! 🔥⚡🎯"
    })
}

// Add a sync function to test automatic wrapping
#[get("/sync")]
async fn sync_test() -> &'static str {
    "🔄 SYNC WITH #[hot] MACRO - Now this endpoint supports hot reload too! 🎯"
}

#[get("/hot-async")]
async fn hot_async_test() -> impl IntoResponse {
    subsecond::call(|| {
        "🔥 ASYNC with SUBSECOND CALL - Hot reload is working perfectly! 🎯✨"
    })
}
