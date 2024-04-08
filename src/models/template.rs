use askama::Template as AskamaTenplate;
use axum::response::Html;
use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera}; // bring trait in scope

pub type HtmlResource = Html<String>;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let mut tera = match Tera::new("src/bin/html/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera.autoescape_on(vec![".html", ".sql"]);
        // tera.register_filter("do_nothing", do_nothing_filter);
        tera
    };
}

pub trait TemplateView {}

pub struct Template {}

impl Template {
    pub fn shared() -> &'static Tera {
        &TEMPLATES
    }
    pub fn render<T>(template: &str, data: T) -> String
    where
        T: Serialize,
    {
        TEMPLATES
            .render(template, &Context::from_serialize(&data).unwrap())
            .unwrap()
    }

    pub fn render_view<T>(template: &T) -> String
    where
        T: TemplateView + AskamaTenplate,
    {
        template.render().unwrap()
    }
}

#[macro_export]
macro_rules! view {
    ($template:expr, $data:expr) => {
        rust_queue::models::template::render_view::<_>($template, $data)
    };
}

pub fn render_view<T>(template: &str, data: T) -> Html<String>
where
    T: Serialize,
{
    return Html(
        Template::shared()
            .render(template, &Context::from_serialize(&data).unwrap())
            .unwrap(),
    );
}
