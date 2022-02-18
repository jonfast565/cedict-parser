use crate::parser::CeResult;
use tera::{Context, Tera};

lazy_static! {
    pub static ref STATIC_TEMPLATES: Tera = {
        let tera = match Tera::new("templates/**/*") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                ::std::process::exit(1);
            }
        };
        tera
    };
}

pub struct TemplateBuilder {}

impl TemplateBuilder {
    pub fn compile_latex(words: &CeResult) -> String {
        match STATIC_TEMPLATES.render("latex.tera", &Context::from_serialize(&words).unwrap()) {
            Ok(compiled) => return compiled,
            Err(err) => panic!("{}", err),
        };
    }
}