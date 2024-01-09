use minijinja::Environment;
use std::cell::OnceCell;
use std::sync::{Arc, Mutex, OnceLock};

#[cfg(debug_assertions)]
pub use dynja_derive::Template;

#[cfg(not(debug_assertions))]
pub use askama::Template;

pub use minijinja;

pub trait TemplateFile {
    const PATH: &'static str; // NOTE: this is not the actual path of the template.
                              // It's just the name of the variable taken in the #[template] macro,
                              // just like with 'askama'.
}

pub fn templates() -> &'static Environment<'static> {
    static ENV: OnceLock<Environment> = OnceLock::new();
    ENV.get_or_init(|| {
        let mut env = Environment::new();
        env.set_loader(minijinja::path_loader("templates"));
        env
    })
}
