use async_trait::async_trait;
use axum::{
    body::Body,
    extract::Request as AxumRequest,
    middleware::Next,
    response::{IntoResponse, Response},
    routing::{delete, get, head, options, patch, post, put, trace, MethodRouter},
};
use std::{future::Future, pin::Pin};

use super::{application::Application, request::Request};

#[async_trait]
pub trait Controller: Default + Send {
    type ReturnType: Send + IntoResponse;

    async fn execute(&self, request: Request) -> Self::ReturnType;
}

#[async_trait]

pub trait Middleware: Send {
    async fn execute(&self, request: AxumRequest, next: Next) -> Response;
}

pub trait RouterRegister {
    fn register_routes();
}

pub struct Router {
    pub path: String,
    pub method: MethodRouter,
    pub middleware: Option<Box<dyn Middleware>>,
}

impl Router {
    pub fn new(path: String, method_router: MethodRouter) -> Self {
        Router {
            path: path,
            middleware: None,
            method: method_router,
        }
    }

    pub fn set_middleware<M>(&mut self)
    where
        M: Middleware + Default + 'static,
    {
        self.middleware = Some(Box::new(M::default()));
    }
}

pub struct Route;
impl Route {
    pub fn get<C>(path: &str)
    where
        C: Controller + 'static,
    {
        let router: Router = Router::new(path.to_owned(), get(execute::<C>));
        Application::shared().add_router(router);
    }

    pub fn post<C>(path: &str)
    where
        C: Controller + 'static,
    {
        let router: Router = Router::new(path.to_owned(), post(execute::<C>));
        Application::shared().add_router(router);
    }
    pub fn put<C>(path: &str)
    where
        C: Controller + 'static,
    {
        let router: Router = Router::new(path.to_owned(), put(execute::<C>));
        Application::shared().add_router(router);
    }
    pub fn options<C>(path: &str)
    where
        C: Controller + 'static,
    {
        let router: Router = Router::new(path.to_owned(), options(execute::<C>));
        Application::shared().add_router(router);
    }
    pub fn patch<C>(path: &str)
    where
        C: Controller + 'static,
    {
        let router: Router = Router::new(path.to_owned(), patch(execute::<C>));
        Application::shared().add_router(router);
    }
    pub fn head<C>(path: &str)
    where
        C: Controller + 'static,
    {
        let router: Router = Router::new(path.to_owned(), head(execute::<C>));
        Application::shared().add_router(router);
    }
    pub fn delete<C>(path: &str)
    where
        C: Controller + 'static,
    {
        let router: Router = Router::new(path.to_owned(), delete(execute::<C>));
        Application::shared().add_router(router);
    }
    pub fn trace<C>(path: &str)
    where
        C: Controller + 'static,
    {
        let router: Router = Router::new(path.to_owned(), trace(execute::<C>));
        Application::shared().add_router(router);
    }

    pub fn group(path: &str, func: impl Fn()) {
        Application::shared().is_grouping_route(path.to_owned());
        func();
        Application::shared().reset_is_grouping_route();
    }
}

fn execute<C>(request: AxumRequest<Body>) -> Pin<Box<dyn Future<Output = C::ReturnType> + Send>>
where
    C: Controller + 'static,
{
    // Create a future that resolves to a JSON value representing the data
    let future = async {
        let controller = C::default();
        controller.execute(Request(request)).await
    };

    Box::pin(future)
}
