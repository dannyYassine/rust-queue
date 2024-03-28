use axum::{
    routing::{delete, get, post, put},
    Json,
};
use serde_json::Value;

use super::application::Application;

pub trait Controller: Default {
    fn execute(&self) -> Json<Value>;
}

pub trait Router {
    fn register_routes();
}

pub struct Route;
impl Route {
    pub fn get<C>(path: &str)
    where
        C: Controller,
    {
        Application::shared().add_route(path, get(move || async { execute::<C>() }));
    }
    pub fn post<C>(path: &str)
    where
        C: Controller,
    {
        Application::shared().add_route(path, post(move || async { execute::<C>() }));
    }
    pub fn put<C>(path: &str)
    where
        C: Controller,
    {
        Application::shared().add_route(path, put(move || async { execute::<C>() }));
    }
    pub fn delete<C>(path: &str)
    where
        C: Controller,
    {
        Application::shared().add_route(path, delete(move || async { execute::<C>() }));
    }
}

fn execute<C>() -> Json<Value>
where
    C: Controller,
{
    let controller = C::default();

    controller.execute()
}
