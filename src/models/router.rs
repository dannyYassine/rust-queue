use std::{future::Future, pin::Pin};

use axum::{
    routing::{delete, get, head, options, patch, post, put, trace},
    Json,
};
use serde::Serialize;

use super::application::Application;

pub trait Controller: Default + Send {
    type ReturnType: Serialize + Send;

    fn execute(&self) -> Self::ReturnType;
}

pub trait Router {
    fn register_routes();
}

pub struct Route;
impl Route {
    pub fn get<C>(path: &str)
    where
        C: Controller + 'static,
    {
        Application::shared().add_route(path, get(execute::<C>));
    }
    pub fn post<C>(path: &str)
    where
        C: Controller + 'static,
    {
        Application::shared().add_route(path, post(execute::<C>));
    }
    pub fn put<C>(path: &str)
    where
        C: Controller + 'static,
    {
        Application::shared().add_route(path, put(execute::<C>));
    }
    pub fn options<C>(path: &str)
    where
        C: Controller + 'static,
    {
        Application::shared().add_route(path, options(execute::<C>));
    }
    pub fn patch<C>(path: &str)
    where
        C: Controller + 'static,
    {
        Application::shared().add_route(path, patch(execute::<C>));
    }
    pub fn head<C>(path: &str)
    where
        C: Controller + 'static,
    {
        Application::shared().add_route(path, head(execute::<C>));
    }
    pub fn delete<C>(path: &str)
    where
        C: Controller + 'static,
    {
        Application::shared().add_route(path, delete(execute::<C>));
    }
    pub fn trace<C>(path: &str)
    where
        C: Controller + 'static,
    {
        Application::shared().add_route(path, trace(execute::<C>));
    }

    pub fn group(path: &str, func: impl Fn()) {
        Application::shared().is_grouping_route(path.to_owned());
        func();
        Application::shared().reset_is_grouping_route();
    }
}

fn execute<C>() -> Pin<Box<dyn Future<Output = Json<C::ReturnType>> + Send>>
where
    C: Controller + 'static,
{
    // Create a future that resolves to a JSON value representing the data
    let future = async {
        let controller = C::default();
        Json(controller.execute())
    };

    Box::pin(future)
}
