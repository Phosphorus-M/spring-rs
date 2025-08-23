use anyhow::Context;
use spring::{auto_config, App};
use spring_sqlx::sqlx::Row;
use spring_sqlx::{sqlx, ConnectPool, SqlxPlugin};
use spring_web::error::KnownWebError;
use spring_web::{WebPlugin, WebConfigurator};
use spring_web::{
    axum::response::IntoResponse,
    error::Result,
    extractor::Component,
    get, Router, handler::TypeRouter,
};
use subsecond;

#[auto_config(WebConfigurator)]
#[spring::main]
async fn main() {
    App::new()
        .add_plugin(WebPlugin)
        .add_plugin(SqlxPlugin)
        .run()
        .await
}

#[get("/")]
async fn hello_world() -> impl IntoResponse {
    "🎉 #[hot] MACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
}


#[get("/goodbye")]
async fn goodbye() -> impl IntoResponse {
    "🎉 #[hot] aasMsssssACssssssssssssssRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
}
