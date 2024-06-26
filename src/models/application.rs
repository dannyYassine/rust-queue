use std::{
    env,
    future::Future,
    pin::Pin,
    sync::{Arc, Mutex},
};

use axum::{
    body::Body, extract::Request, middleware::Next, response::Response, routing::MethodRouter,
    Router as AxumRouter,
};
use dotenvy::dotenv;
use lazy_static::lazy_static;

use super::{
    app_state::AppStateManager,
    data_connection::DatabaseConnection,
    router::{Middleware, Router, RouterRegister},
    service_providers::ServiceProvider,
};

#[derive(Clone)]
pub struct Application {
    service_providers: Arc<Mutex<Vec<Box<dyn ServiceProvider>>>>,
    router: Arc<Mutex<AxumRouter>>,
    routers: Arc<Mutex<Vec<Router>>>,
    grouped_route_path: Arc<Mutex<String>>,
}

impl Application {
    pub fn new() -> Self {
        Application {
            service_providers: Arc::new(Mutex::new(vec![])),
            router: Arc::new(Mutex::new(AxumRouter::new())),
            routers: Arc::new(Mutex::new(vec![])),
            grouped_route_path: Arc::new(Mutex::new(String::new())),
        }
    }
    pub async fn initialize(&self) -> &Self {
        Self::load_env_vars().await;
        Self::set_up_database_connection().await;

        return self;
    }
    pub async fn register_root_service_provider<S>(&self) -> &Self
    where
        S: ServiceProvider + Default,
    {
        let root = S::default();

        {
            root.register(self);
            let providers = self.service_providers.lock().unwrap();
            for provider in providers.iter() {
                provider.register(self);
            }
        }
        {
            root.boot(self);
            let providers = self.service_providers.lock().unwrap();
            for provider in providers.iter() {
                provider.boot(self);
            }
        }

        return self;
    }
    pub fn register_service_provider<S>(&self)
    where
        S: ServiceProvider + Default + 'static,
    {
        let mut providers = self.service_providers.lock().unwrap();
        providers.push(Box::new(S::default()));
    }
    pub fn is_grouping_route(&self, path: String) {
        let mut router = self.grouped_route_path.lock().unwrap();
        *router = path;
    }
    pub fn reset_is_grouping_route(&self) {
        let mut router = self.grouped_route_path.lock().unwrap();
        *router = String::from("");
    }
    pub fn add_router(&self, router: Router) -> &Self {
        {
            let mut routers = self.routers.lock().unwrap();
            routers.push(router);
        }

        return self;
    }
    pub fn add_route(&self, path: &str, method_router: MethodRouter) -> &Self {
        let grouped_route_path = self.grouped_route_path.lock().unwrap();
        let new_path = format!("{}{}", grouped_route_path, path);
        let mut router = self.router.lock().unwrap();
        *router = router.clone().route(&new_path, method_router);

        return self;
    }
    fn add_routes_to_router(&self) {
        for router in self.routers.lock().unwrap().iter() {
            let grouped_route_path = self.grouped_route_path.lock().unwrap();
            let new_path = format!("{}{}", grouped_route_path, router.path);
            let mut axum_router = self.router.lock().unwrap();
            let mut r = axum_router
                .clone()
                .route(&new_path, router.method.to_owned());

            if let Some(middleware) = &router.middleware {
                // let func = get_middleware(middleware);
                // r = r.layer(middleware::from_fn(func));
            }

            *axum_router = axum_router
                .clone()
                .route(&new_path, router.method.to_owned());
        }
    }
    pub fn add_middleware_to_router(&self, router: &Router, middleware: Box<dyn Middleware>) {
        let mut routers = self.routers.lock().unwrap();

        if let Some(r) = routers.iter_mut().find(|r| r.path == router.path) {
            r.middleware = Some(middleware);
        }
    }
    pub async fn serve(&self) {
        self.add_routes_to_router();

        let host: String = env::var("HTTP_SERVER_HOST").unwrap();
        let port: String = env::var("HTTP_SERVER_PORT").unwrap();

        println!("Service application at: {}:{}", &host, &port);
        let app = self.router.lock().unwrap().clone();
        let listener = tokio::net::TcpListener::bind(format!("{}:{}", host, port).as_str())
            .await
            .unwrap();
        axum::serve(listener, app).await.unwrap();
    }
    pub async fn register_routes<R>(&self) -> &Self
    where
        R: RouterRegister,
    {
        R::register_routes();

        return self;
    }
    pub async fn bootstrap() {
        Self::load_env_vars().await;
        Self::set_up_database_connection().await;
    }

    async fn load_env_vars() {
        dotenv().ok();
    }

    async fn set_up_database_connection() {
        let connection = DatabaseConnection::create();
        AppStateManager::get_instance().set_connection(connection);
    }
}

lazy_static! {
    static ref APPLICATION: Application = Application::new();
}

impl Application {
    pub fn shared() -> &'static Self {
        &APPLICATION
    }
}

async fn get_middleware(
    middleware: &'static (dyn Middleware + 'static),
) -> impl Fn(Request, Next) -> Pin<Box<dyn Future<Output = Response<Body>> + Send>> {
    return move |request: Request,
                 next: Next|
          -> Pin<Box<dyn Future<Output = Response<Body>> + Send + 'static>> {
        middleware.execute(request, next)
    };
}

async fn my_middleware(request: Request, next: Next) -> Response {
    // do something with `request`...

    let response = next.run(request).await;

    // do something with `response`...

    response
}
