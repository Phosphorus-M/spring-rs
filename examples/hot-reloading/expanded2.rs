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
struct __TypedHandlerRegistrar_hello_world_0198d872b51074f2af5e6507f2f07536;
impl ::spring_web::handler::TypedHandlerRegistrar
for __TypedHandlerRegistrar_hello_world_0198d872b51074f2af5e6507f2f07536 {
    fn install_route(&self, mut __router: ::spring_web::Router) -> ::spring_web::Router {
        async fn hello_world() -> impl IntoResponse {
            "🎉 #[hot] MACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
        }
        let __method_router = ::spring_web::MethodRouter::new();
        let __method_router = ::spring_web::MethodRouter::on(
            __method_router,
            ::spring_web::MethodFilter::GET,
            __TypedHandlerRegistrar_hello_world_0198d872b51074f2af5e6507f2f07536,
        );
        __router = ::spring_web::Router::route(__router, "/", __method_router);
        __router
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            &__TypedHandlerRegistrar_hello_world_0198d872b51074f2af5e6507f2f07536
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
struct __TypedHandlerRegistrar_goodbye_0198d872b51176819f1fcb0c61f3d0e0;
impl ::spring_web::handler::TypedHandlerRegistrar
for __TypedHandlerRegistrar_goodbye_0198d872b51176819f1fcb0c61f3d0e0 {
    fn install_route(&self, mut __router: ::spring_web::Router) -> ::spring_web::Router {
        async fn goodbye() -> impl IntoResponse {
            "🎉 #[hot] aasMsssssACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
        }
        let __method_router = ::spring_web::MethodRouter::new();
        let __method_router = ::spring_web::MethodRouter::on(
            __method_router,
            ::spring_web::MethodFilter::GET,
            __TypedHandlerRegistrar_goodbye_0198d872b51176819f1fcb0c61f3d0e0,
        );
        __router = ::spring_web::Router::route(__router, "/goodbye", __method_router);
        __router
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            &__TypedHandlerRegistrar_goodbye_0198d872b51176819f1fcb0c61f3d0e0
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
