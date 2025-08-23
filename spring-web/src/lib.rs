//! [![spring-rs](https://img.shields.io/github/stars/spring-rs/spring-rs)](https://spring-rs.github.io/docs/plugins/spring-web)
#![doc = include_str!("../README.md")]
#![doc(html_favicon_url = "https://spring-rs.github.io/favicon.ico")]
#![doc(html_logo_url = "https://spring-rs.github.io/logo.svg")]

/// spring-web config
pub mod config;
/// spring-web defined error
pub mod error;
/// axum extract
pub mod extractor;
/// axum route handler
pub mod handler;
pub mod middleware;

pub use axum;
pub use spring::async_trait;
/////////////////web-macros/////////////////////
/// To use these Procedural Macros, you need to add `spring-web` dependency
pub use spring_macros::delete;
pub use spring_macros::get;
pub use spring_macros::head;
pub use spring_macros::middlewares;
pub use spring_macros::nest;
pub use spring_macros::options;
pub use spring_macros::patch;
pub use spring_macros::post;
pub use spring_macros::put;
pub use spring_macros::route;
pub use spring_macros::routes;
pub use spring_macros::trace;

/// axum::routing::MethodFilter re-export
pub use axum::routing::MethodFilter;
/// MethodRouter with AppState
pub use axum::routing::MethodRouter;
/// Router with AppState
pub use axum::Router;

// trait RemoveRoute {
//     fn remove_route(self, path: &str, method: MethodFilter) -> Option<()>;
// }

// impl RemoveRoute for Router {
//     fn remove_route(self, path: &str, method: MethodFilter) -> Option<()> {
//         self.into_inner
//     }
// }



use anyhow::Context;
use axum::Extension;
use config::ServerConfig;
use config::WebConfig;
use spring::plugin::component::ComponentRef;
use spring::plugin::ComponentRegistry;
use spring::plugin::MutableComponentRegistry;
use spring::{
    app::{App, AppBuilder},
    config::ConfigRegistry,
    error::Result,
    plugin::Plugin,
};
use std::{collections::HashMap, net::SocketAddr, ops::Deref, sync::{Arc, LazyLock, Mutex}};

use crate::handler::TypedHandlerRegistrar;

static HOT_RELOAD_PORTS: LazyLock<Mutex<HashMap<SocketAddr, bool>>> = 
    LazyLock::new(|| Mutex::new(HashMap::new()));

async fn get_or_create_listener(addr: SocketAddr) -> Result<tokio::net::TcpListener> {
    let socket = if addr.is_ipv4() {
        tokio::net::TcpSocket::new_v4()
    } else {
        tokio::net::TcpSocket::new_v6()
    }.context("Failed to create TCP socket")?;
    
    socket.set_reuseaddr(true).context("Failed to set SO_REUSEADDR")?;
    
    #[cfg(unix)]
    {
        use std::os::unix::io::AsRawFd;
        let fd = socket.as_raw_fd();
        
        unsafe {
            let enable: libc::c_int = 1;
            let result = libc::setsockopt(
                fd,
                libc::SOL_SOCKET,
                libc::SO_REUSEPORT,
                &enable as *const _ as *const libc::c_void,
                std::mem::size_of_val(&enable) as libc::socklen_t,
            );
            
            if result != 0 {
                tracing::warn!("Failed to set SO_REUSEPORT, continuing without it");
            } else {
                tracing::debug!("SO_REUSEPORT enabled for hot reload compatibility");
            }
        }
    }
    
    match socket.bind(addr) {
        Ok(()) => {
            let listener = socket.listen(1024).context("Failed to listen on socket")?;
            
            let mut ports = HOT_RELOAD_PORTS.lock().unwrap();
            ports.insert(addr, true);
            
            tracing::info!("🆕 Created TCP listener for hot reload: {addr}");
            Ok(listener)
        }
        Err(e) if e.kind() == std::io::ErrorKind::AddrInUse => {
            let is_tracked = {
                let ports = HOT_RELOAD_PORTS.lock().unwrap();
                ports.contains_key(&addr)
            };
                
            if is_tracked {
                tracing::info!("♻️ Port {addr} tracked for hot reload, attempting forceful bind");
                
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
                
                let socket = if addr.is_ipv4() {
                    tokio::net::TcpSocket::new_v4()
                } else {
                    tokio::net::TcpSocket::new_v6()
                }.context("Failed to create TCP socket")?;
                
                socket.set_reuseaddr(true).context("Failed to set SO_REUSEADDR")?;
                
                #[cfg(unix)]
                {
                    use std::os::unix::io::AsRawFd;
                    let fd = socket.as_raw_fd();
                    
                    unsafe {
                        let enable: libc::c_int = 1;
                        libc::setsockopt(
                            fd,
                            libc::SOL_SOCKET,
                            libc::SO_REUSEPORT,
                            &enable as *const _ as *const libc::c_void,
                            std::mem::size_of_val(&enable) as libc::socklen_t,
                        );
                    }
                }
                
                match socket.bind(addr) {
                    Ok(()) => {
                        let listener = socket.listen(1024).context("Failed to listen on socket")?;
                        tracing::info!("♻️ Successfully rebound TCP listener for hot reload: {addr}");
                        Ok(listener)
                    }
                    Err(e) => {
                        tracing::error!("Failed to rebind for hot reload: {e}");
                        Err(anyhow::Error::new(e).context(format!("bind tcp listener failed:{addr}")).into())
                    }
                }
            } else {
                tracing::error!("Port {addr} already in use and not tracked for hot reload");
                Err(anyhow::Error::new(e).context(format!("bind tcp listener failed:{addr}")).into())
            }
        }
        Err(e) => {
            tracing::error!("Failed to bind TCP listener: {e}");
            Err(anyhow::Error::new(e).context(format!("bind tcp listener failed:{addr}")).into())
        }
    }
}

/// Routers collection
pub type Routers = Vec<Router>;

/// Web Configurator
pub trait WebConfigurator {
    /// add route to app registry
    fn add_router(&mut self, router: Router) -> &mut Self;
}

impl WebConfigurator for AppBuilder {
    fn add_router(&mut self, router: Router) -> &mut Self {
        println!("Adding router to AppBuilder");
        tracing::info!("Adding router to AppBuilder");
        if let Some(routers) = self.get_component_ref::<Routers>() {
            unsafe {
                let raw_ptr = ComponentRef::into_raw(routers);
                let routers = &mut *(raw_ptr as *mut Routers);
                routers.push(router);
            }
            self
        } else {
            self.add_component(vec![router])
        }
    }
}

/// State of App
#[derive(Clone)]
pub struct AppState {
    /// App Registry Ref
    pub app: Arc<App>,
}

/// Web Plugin Definition
pub struct WebPlugin;

#[async_trait]
impl Plugin for WebPlugin {
    async fn build(&self, app: &mut AppBuilder) {
        let config = app
            .get_config::<WebConfig>()
            .expect("web plugin config load failed");

        // 1. collect router
        let routers = app.get_component_ref::<Routers>();
        let mut router: Router = match routers {
            Some(rs) => {
                let mut router = Router::new();
                tracing::info!("Collected {} routers", rs.deref().len());
                for r in rs.deref().iter() {
                    router = router.merge(r.to_owned());
                }
                router
            }
            None => Router::new(),
        };
        if let Some(middlewares) = config.middlewares {
            router = crate::middleware::apply_middleware(router, middlewares);
        }

        app.add_component(router);

        let server_conf = config.server;

        app.add_scheduler(move |app: Arc<App>| Box::new(Self::schedule(app, server_conf)));
    }
}

impl WebPlugin {
    async fn schedule(app: Arc<App>, config: ServerConfig) -> Result<String> {
        let router = app.get_expect_component::<Router>();

        // 2. bind tcp listener (with hot reload support)
        let addr = SocketAddr::from((config.binding, config.port));
        let listener = get_or_create_listener(addr).await?;

        // 3. axum server
        let router = router.layer(Extension(AppState { app }));

        tracing::info!("axum server started");
        
        if config.connect_info {
            // with client connect info
            let service = router.into_make_service_with_connect_info::<SocketAddr>();
            let server = axum::serve(listener, service);
            if config.graceful {
                server.with_graceful_shutdown(shutdown_signal()).await
            } else {
                server.await
            }
        } else {
            let service = router.into_make_service();
            let server = axum::serve(listener, service);
            if config.graceful {
                server.with_graceful_shutdown(shutdown_signal()).await
            } else {
                server.await
            }
        }
        .context("start axum server failed")?;

        Ok("axum schedule finished".to_string())
    }
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C signal, waiting for web server shutdown")
        },
        _ = terminate => {
            tracing::info!("Received kill signal, waiting for web server shutdown")
        },
    }
}
