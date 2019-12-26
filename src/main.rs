#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::request::{ FromForm, Form };
use rocket::http::{ Cookie, Cookies };
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;

mod sql;

#[derive(Serialize)]
struct Info {
    profile: String,
    users: Vec<String>,
}

impl Info {
    fn new() -> Self {
        Info {profile: String::new(), users: Vec::new()}
    }

    fn insert_profile(&mut self, profile: String) {
        self.profile = profile;
    }

    fn insert_users(&mut self, users: Vec<String>) {

    }
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
        .mount(
            "/",
            routes![
                index, input, user, receive_card,
                receive_card_set, receive_user,
                ]
            )
        // This mount is for serving CSS, images, JS, etc.
        // everything that ISN'T a HTML template.
        .mount("/assets", StaticFiles::from("static"))
        .attach(Template::fairing())
}

#[get("/")]
fn index(cookies: Cookies) -> Template {
    let mut info = Info::new();
    let cookie = cookies.get("profile");
    // An empty context can be an empty HashMap
    // or a struct that derives Serialize from serde
    // let mut context: HashMap<&str, &str> = HashMap::new();
    
    if let Some(c) = cookie {
        info.insert_profile(c.value().to_string());
    }
    
    match sql::select_users() {
        Ok(users) => info.users = users,
        Err(e) => eprintln!("{}", e), 
    }

    Template::render("index", &info)
}

#[get("/user")]
fn user(cookies: Cookies) -> Template {
    let cookie = cookies.get("profile");
    let mut context: HashMap<&str, &str> = HashMap::new();

    if let Some(c) = cookie {
        context.insert("profile",c.value());
    }

    Template::render("user", &context)
}

#[get("/input")]
fn input() -> Template {
    // Values in Rust are stored in the stack by default
    // Placing things in a Box<T> stores them on the heap
    // instead. The box being a pointer to the value(s)
    // Vec also places values on the heap.
    // The compiler doesn't need the length/capacity of these
    // at compile time.
    let context: HashMap<&str, Vec<String>> = match sql::select_card_sets() {
        Ok(names) => {
            [("card_sets", names)].iter().cloned().collect()
        },
        Err(_) => {
            [("card_sets", vec![String::from("Not Real 2020")])].iter().cloned().collect()
        }
    };
    
    Template::render("input", &context)
}

#[post("/receive_user", data = "<user>")]
fn receive_user(user: Form<sql::Users>) -> Template {
    let context: HashMap<&str, &str> = match sql::insert_user(&user) {
        Ok(_) => [("result", "Successfully wrote to database.")].iter().cloned().collect(),
        Err(_) => [("result", "Something went wrong.")].iter().cloned().collect(),
    };

    Template::render("receive", &context)
}

#[post("/receive_card_set", data = "<card_set>")]
fn receive_card_set(card_set: Form<sql::CardSets>) -> Template {
    let context: HashMap<&str, &str> = match sql::insert_card_set(&card_set) {
        Ok(_) => [("result", "Card set inserted into database.")].iter().cloned().collect(),
        Err(e) => {
            eprintln!("{}", e);
            [("result", "Could not insert to database.")].iter().cloned().collect()
        },
    };

    Template::render("receive", &context)
}

#[post("/receive_card", data = "<card>")]
fn receive_card(card: Form<sql::Cards>) -> Template {
    let context: HashMap<&str, &str> = HashMap::new();
    Template::render("receive", &context)
}