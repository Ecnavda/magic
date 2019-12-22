#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::request::{ FromForm, Form };
use rocket_contrib::templates::Template;
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;

mod sql;

#[derive(FromForm)]
struct CardSet {
    name: String,
    release: String,
}

#[derive(FromForm)]
struct Users {
    email: String,
    name: String,
}

#[derive(FromForm)]
struct Cards {
    card_set: String,
    card_number: i32,
    name: String,
    color: String,
    cmc: i32,
}

fn main() {
    println!("Initializing database...");

    match sql::create_schema() {
        Ok(_) => println!("Success"),
        Err(e) => eprintln!("Error: {}", e),
    };

    start_webserver().launch();
}

fn start_webserver() -> rocket::Rocket {
    // Fairings (middleware) must be attached to rocket
    // before launching.
    rocket::ignite()
        .mount("/", routes![index, user, receive_user])
        .attach(Template::fairing())
}

#[get("/")]
fn index() -> Template {
    // An empty context can be an empty HashMap
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("index", &context)
}

#[get("/user")]
fn user() -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("user", &context)
}

#[post("/receive", data = "<user>")]
fn receive_user(user: Form<Users>) -> Template {
    let context: HashMap<&str, &str> = match sql::sql_insert("users", user.email.as_str()) {
        Ok(_) => [("result", "Successfully wrote to database.")].iter().cloned().collect(),
        Err(_) => [("result", "Something went wrong.")].iter().cloned().collect(),
    };

    Template::render("receive", &context)
}