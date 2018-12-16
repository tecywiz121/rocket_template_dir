#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use rocket_contrib::templates::Template;

#[get("/default")]
fn default_template() -> Template {
    Template::render("foo", ())
}

#[get("/extended")]
fn extended_template() -> Template {
    Template::render("bar", ())
}

fn main() {
    rocket::ignite()
        .attach(Template::custom(|engines| {
            let extended_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/extended");
            engines
                .handlebars
                .register_templates_directory(".html.hbs", extended_dir)
                .unwrap();

            println!("Has Bar: {:?}", engines.handlebars.has_template("bar"));
        }))
        .mount("/", routes![default_template, extended_template])
        .launch();
}
