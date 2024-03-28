use super::application::Application;

pub trait ServiceProvider: Send {
    #[allow(unused_variables)]
    fn register(&self, app: &Application) {}
    #[allow(unused_variables)]
    fn boot(&self, app: &Application) {}
}
