use rust_queue::models::{application::Application, service_providers::ServiceProvider};

#[derive(Default)]
struct RootServiceProvider;
impl ServiceProvider for RootServiceProvider {
    fn register(&self, app: &Application) {
        app.register_service_provider::<ChildServiceProvider>();
    }
}

#[derive(Default)]
struct ChildServiceProvider;
impl ServiceProvider for ChildServiceProvider {
    #[allow(unused_variables)]
    fn register(&self, app: &Application) {
        println!("ChildServiceProvider registered");
    }
    #[allow(unused_variables)]
    fn boot(&self, app: &Application) {
        println!("ChildServiceProvider booted");
    }
}

#[tokio::main]
async fn main() {
    let application = Application::new();
    let _ = application
        .register_root_service_provider::<RootServiceProvider>()
        .await;
}
