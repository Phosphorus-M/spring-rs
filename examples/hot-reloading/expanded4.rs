Using handler function as-is: Ident { ident: "hello_world_0198d88f07557001b451a5dad1f0d9b9", span: #17 bytes(546..557) }
Generating struct: TokenStream [Ident { ident: "async", span: #0 bytes(558..563) }, Ident { ident: "fn", span: #0 bytes(564..566) }, Ident { ident: "hello_world_0198d88f07557001b451a5dad1f0d9b9", span: #17 bytes(546..557) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(578..580) }, Punct { ch: '-', spacing: Joint, span: #0 bytes(581..582) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(582..583) }, Ident { ident: "impl", span: #0 bytes(584..588) }, Ident { ident: "IntoResponse", span: #0 bytes(589..601) }, Group { delimiter: Brace, stream: TokenStream [Literal { kind: Str, symbol: "🎉 #[hot] MACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨", suffix: None, span: #0 bytes(608..705) }], span: #0 bytes(602..707) }]
Using handler function as-is: Ident { ident: "goodbye_0198d88f0756777083e615673c13b73c", span: #25 bytes(710..728) }
Generating struct: TokenStream [Ident { ident: "async", span: #0 bytes(729..734) }, Ident { ident: "fn", span: #0 bytes(735..737) }, Ident { ident: "goodbye_0198d88f0756777083e615673c13b73c", span: #25 bytes(710..728) }, Group { delimiter: Parenthesis, stream: TokenStream [], span: #0 bytes(745..747) }, Punct { ch: '-', spacing: Joint, span: #0 bytes(748..749) }, Punct { ch: '>', spacing: Alone, span: #0 bytes(749..750) }, Ident { ident: "impl", span: #0 bytes(751..755) }, Ident { ident: "IntoResponse", span: #0 bytes(756..768) }, Group { delimiter: Brace, stream: TokenStream [Literal { kind: Str, symbol: "🎉 #[hot] aasMsssssACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨", suffix: None, span: #0 bytes(775..880) }], span: #0 bytes(769..882) }]
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
struct hello_world_0198d88f07557001b451a5dad1f0d9b9;
impl ::spring_web::handler::TypedHandlerRegistrar
for hello_world_0198d88f07557001b451a5dad1f0d9b9 {
    fn install_route(&self, mut __router: ::spring_web::Router) -> ::spring_web::Router {
        async fn hello_world_0198d88f07557001b451a5dad1f0d9b9() -> impl IntoResponse {
            "🎉 #[hot] MACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
        }
        let __method_router = ::spring_web::MethodRouter::new();
        let __method_router = ::spring_web::MethodRouter::on(
            __method_router,
            ::spring_web::MethodFilter::GET,
            hello_world_0198d88f07557001b451a5dad1f0d9b9,
        );
        __router = ::spring_web::Router::route(__router, "/", __method_router);
        __router
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            &hello_world_0198d88f07557001b451a5dad1f0d9b9
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
struct goodbye_0198d88f0756777083e615673c13b73c;
impl ::spring_web::handler::TypedHandlerRegistrar
for goodbye_0198d88f0756777083e615673c13b73c {
    fn install_route(&self, mut __router: ::spring_web::Router) -> ::spring_web::Router {
        async fn goodbye_0198d88f0756777083e615673c13b73c() -> impl IntoResponse {
            "🎉 #[hot] aasMsssssACRO **ISs** - The macro-based hot reload is functioning perfectly! 🚀🔥⚡✨"
        }
        let __method_router = ::spring_web::MethodRouter::new();
        let __method_router = ::spring_web::MethodRouter::on(
            __method_router,
            ::spring_web::MethodFilter::GET,
            goodbye_0198d88f0756777083e615673c13b73c,
        );
        __router = ::spring_web::Router::route(__router, "/goodbye", __method_router);
        __router
    }
}
#[allow(non_upper_case_globals)]
const _: () = {
    static __INVENTORY: ::inventory::Node = ::inventory::Node {
        value: &{
            &goodbye_0198d88f0756777083e615673c13b73c
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
