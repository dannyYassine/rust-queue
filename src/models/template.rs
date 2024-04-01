use lazy_static::lazy_static;
use serde::Serialize;
use tera::{Context, Tera};

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
}

fn render_view<T>(template: &str, data: T) -> String
where
    T: Serialize,
{
    return Template::shared()
        .render(template, &Context::from_serialize(&data).unwrap())
        .unwrap();
}

#[macro_export]
macro_rules! view {
    ($template:expr, $data:expr) => {
        render_view::<_>($template, $data)
    };
}
