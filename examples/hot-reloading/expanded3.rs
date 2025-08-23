#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2021::*;
#[macro_use]
extern crate std;
use anyhow::Context;
use spring::{auto_config, App};
use spring_sqlx::sqlx::Row;
use spring_sqlx::{sqlx, ConnectPool, SqlxPlugin};
use spring_web::error::KnownWebError;
use spring_web::{WebPlugin, WebConfigurator};
use spring_web::{
    axum::response::IntoResponse, error::Result, extractor::Component, get, Router,
    handler::TypeRouter,
};
use subsecond;
async fn __internal_main() {
    {
        App::new()
            .add_router(::spring_web::handler::auto_router())
            .add_plugin(WebPlugin)
            .add_plugin(SqlxPlugin)
            .run()
            .await
    }
}
fn main() {
    ::tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Failed building the Runtime")
        .block_on(dioxus_devtools::serve_subsecond(__internal_main));
}
#[allow(non_camel_case_types, missing_docs)]
struct hello_world_0198d876ecb57280bf895f2ef7f7105c;
impl ::spring_web::handler::TypedHandlerRegistrar
for hello_world_0198d876ecb57280bf895f2ef7f7105c {
    fn install_route(&self, mut __router: ::spring_web::Router) -> ::spring_web::Router {
        async fn hello_world() -> impl IntoResponse {
            "🎉 #[hot] MACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
        }
        let __method_router = ::spring_web::MethodRouter::new();
        let __method_router = ::spring_web::MethodRouter::on(
            __method_router,
            ::spring_web::MethodFilter::GET,
            hello_world_0198d876ecb57280bf895f2ef7f7105c,
        );
        __router = ::spring_web::Router::route(__router, "/", __method_router);
        __router
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            &hello_world_0198d876ecb57280bf895f2ef7f7105c
                as &dyn ::spring_web::handler::TypedHandlerRegistrar
        },
        next: ::inventory::core::cell::UnsafeCell::new(
            ::inventory::core::option::Option::None,
        ),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};
#[allow(non_camel_case_types, missing_docs)]
struct goodbye_0198d876ecb57280bf895f35530c4941;
impl ::spring_web::handler::TypedHandlerRegistrar
for goodbye_0198d876ecb57280bf895f35530c4941 {
    fn install_route(&self, mut __router: ::spring_web::Router) -> ::spring_web::Router {
        async fn goodbye() -> impl IntoResponse {
            "🎉 #[hot] aasMsssssACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
        }
        let __method_router = ::spring_web::MethodRouter::new();
        let __method_router = ::spring_web::MethodRouter::on(
            __method_router,
            ::spring_web::MethodFilter::GET,
            goodbye_0198d876ecb57280bf895f35530c4941,
        );
        __router = ::spring_web::Router::route(__router, "/goodbye", __method_router);
        __router
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            &goodbye_0198d876ecb57280bf895f35530c4941
                as &dyn ::spring_web::handler::TypedHandlerRegistrar
        },
        next: ::inventory::core::cell::UnsafeCell::new(
            ::inventory::core::option::Option::None,
        ),
    };
    #[link_section = ".text.startup"]
    unsafe extern "C" fn __ctor() {
        unsafe { ::inventory::ErasedNode::submit(__INVENTORY.value, &__INVENTORY) }
    }
    #[used]
    #[link_section = ".init_array"]
    static __CTOR: unsafe extern "C" fn() = __ctor;
};
