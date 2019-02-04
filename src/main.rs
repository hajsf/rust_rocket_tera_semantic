#![feature(proc_macro_hygiene, decl_macro, custom_attribute)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

use rocket_contrib::serve::StaticFiles;
use rocket_contrib::templates::Template;

extern crate serde_json;

use std::collections::HashMap;

use rocket::Request;
use rocket::response::Redirect;

use std::env;

#[derive(Serialize)]
struct TemplateContext {
    name: String,
    items: Vec<&'static str>
}

#[get("/")]
fn index() -> Template { // Redirect {
   // Redirect::to(uri!(get: name = "Unknown"))
   let context: HashMap<&str, i32> = HashMap::new();
   Template::render("index", &context)
}

#[get("/hello/<name>")]
fn get(name: String) -> Template {
   // let name = String::from("Hasan");
    let context = TemplateContext { name, items: vec!["One", "Two", "Three"] };
    Template::render("hello", &context)
}

#[get("/customer/create")]
fn create_customer() -> Template {
    let context: HashMap<&str, i32> = HashMap::new();
    Template::render("customer/create_customer", context)
}

#[catch(404)]
fn not_found(req: &Request) -> Template {
    let mut map = HashMap::new();
    map.insert("path", req.uri().path());
    Template::render("error/404", &map)
}


fn main() {
    let x = 3;
    let executable = env::current_exe().unwrap();
    let exe_dir = match executable.parent() {
        Some(parent) => parent,
        _ => panic!()
    };
   // let static_path = exe_dir.join("static");
    rocket::ignite()
     //   .mount("/", StaticFiles::from(static_path))
        .mount("/static", StaticFiles::from(concat!(env!("CARGO_MANIFEST_DIR"), "/static")))
        .mount("/", routes![index, get, create_customer])
        .attach(Template::fairing())
        .register(catchers![not_found])
        .launch();
}
