use std::any::type_name_of_val;

use askama::Template as AskamaTemplate;
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

pub trait TemplateView {
    fn get_name(&self) -> String {
        let mut snake_case = String::new();

        let name: &str = type_name_of_val(self).split("::").last().unwrap();
        // Iterate over each character in the input string
        for (i, c) in name.chars().enumerate() {
            // If it's uppercase and not the first character, add underscore
            if c.is_uppercase() && i != 0 {
                snake_case.push('_');
            }
            // Convert character to lowercase and add to result
            snake_case.push(c.to_lowercase().next().unwrap());
        }

        for (_, c) in ".html".chars().enumerate() {
            snake_case.push(c.to_lowercase().next().unwrap());
        }

        snake_case
    }
}

pub struct Template {}

impl Template {
    pub fn shared() -> &'static Tera {
        &TEMPLATES
    }

    #[cfg(feature = "tera_templates")]
    pub fn render<T>(template: &T) -> String
    where
        T: TemplateView + Serialize,
    {
        return TEMPLATES
            .render(
                template.get_name().as_str(),
                &Context::from_serialize(&template).unwrap(),
            )
            .unwrap();
    }

    #[cfg(feature = "askama_templates")]
    pub fn render<T>(template: &T) -> String
    where
        T: TemplateView + AskamaTemplate + Serialize,
    {
        template.render().unwrap()
    }
}

#[macro_export]
macro_rules! view {
    ($template:expr) => {
        rust_queue::models::template::render::<_>($template)
    };
}

#[cfg(feature = "tera_templates")]
pub fn render<T>(template: &T) -> Html<String>
where
    T: TemplateView + Serialize,
{
    return Html(Template::render(template));
}

#[cfg(feature = "askama_templates")]
pub fn render<T>(template: &T) -> Html<String>
where
    T: TemplateView + AskamaTemplate + Serialize,
{
    return Html(Template::render(template));
}
