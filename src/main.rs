#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::response::Redirect;
use rocket::request::{ FromForm, Form };
use rocket::http::{ Cookie, Cookies };
use rocket_contrib::templates::Template;
use rocket_contrib::serve::StaticFiles;
use serde::Serialize;
// use std::collections::HashMap;

mod sql;

#[derive(Serialize)]
struct Info {
    profile: String,
    users: Vec<String>,
    card_sets: Vec<(i32, String)>,
    cards: Vec<(String, Vec<String>, String)>,
    result: String,
}

impl Info {
    fn new() -> Self {
        Info {
            profile: String::new(),
            users: Vec::new(),
            card_sets: Vec::new(),
            cards: Vec::new(),
            result: String::new(),
        }
    }

    fn insert_profile(&mut self, profile: String) {
        self.profile = profile;
    }


}

#[derive(FromForm)]
struct Profile {
    profile: String
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
                set_profile, logout, database,
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

#[get("/logout")]
fn logout(mut cookies: Cookies) -> Redirect {
    cookies.remove(Cookie::named("profile"));
    Redirect::to("/")
}

#[get("/database")]
fn database(cookies: Cookies) -> Template {
    let mut info = Info::new();
    if let Some(c) = cookies.get("profile") {
        info.insert_profile(c.value().to_string());
    }

    /*
    if let Ok(cards) = sql::select_cards() {
        // Card name and colors
        println!("{:?}", cards);
        info.cards = cards;
    }
    */

    match sql::select_cards() {
        Ok(cards) => {
            info.cards = cards;
        },
        Err(e) => eprintln!("{}", e),
    }
    
    Template::render("database", &info)
}

#[get("/user")]
fn user(cookies: Cookies) -> Template {
    let cookie = cookies.get("profile");
    let mut info = Info::new();

    if let Some(c) = cookie {
        info.insert_profile(c.value().to_string());
    }

    Template::render("user", &info)
}

#[get("/input")]
fn input(cookies: Cookies) -> Template {
    // Values in Rust are stored in the stack by default
    // Placing things in a Box<T> stores them on the heap
    // instead. The box being a pointer to the value(s)
    // Vec also places values on the heap.
    // The compiler doesn't need the length/capacity of these
    // at compile time.
    /*
    let context: HashMap<&str, Vec<String>> = match sql::select_card_sets() {
        Ok(names) => {
            [("card_sets", names)].iter().cloned().collect()
        },
        Err(_) => {
            [("card_sets", vec![String::from("Not Real 2020")])].iter().cloned().collect()
        }
    };
    */
    let cookie = cookies.get("profile");
    let mut info = Info::new();

    if let Some(c) = cookie {
        info.insert_profile(c.value().to_string());
    }
    /*
    if let Ok(names) = sql::select_card_sets() {
        info.card_sets = names;
    }
    */
    match sql::select_card_sets() {
        Ok(names) => info.card_sets = names,
        Err(e) => eprintln!("ERROR! -- {}", e),
    };
    
    Template::render("input", &info)
}

#[post("/receive_user", data = "<user>")]
fn receive_user(user: Form<sql::Users>, cookies: Cookies) -> Template {
    let cookie = cookies.get("profile");
    let mut info = Info::new();
    if let Some(c) = cookie {
        info.insert_profile(c.value().to_string());
    }

    match sql::insert_user(&user) {
        Ok(_) => info.result = String::from("Card set inserted into database."),
        Err(e) => {
            eprintln!("{}", e);
            info.result = String::from("Could not insert to database.");
        },
    };


    Template::render("receive", &info)
}

#[post("/receive_card_set", data = "<card_set>")]
fn receive_card_set(card_set: Form<sql::CardSets>, cookies: Cookies) -> Redirect {
    let cookie = cookies.get("profile");
    let mut info = Info::new();
    if let Some(c) = cookie {
        info.insert_profile(c.value().to_string());
    }

    match sql::insert_card_set(&card_set) {
        Ok(_) => info.result = String::from("Card set inserted into database."),
        Err(e) => {
            eprintln!("{}", e);
            info.result = String::from("Could not insert to database.");
        },
    };

    Redirect::to("/input")
}

#[post("/receive_card", data = "<card>")]
fn receive_card(card: Form<sql::Cards>, cookies: Cookies) -> Template {
    let cookie = cookies.get("profile");
    let mut info = Info::new();
    if let Some(c) = cookie {
        info.insert_profile(c.value().to_string());
    }
    
    match sql::insert_card(&card) {
        Ok(_) => info.result = String::from("Card inserted into database."),
        Err(e) => {
            eprintln!("{}", e);
            info.result = String::from("Could not insert. Error occurred.");
        }
    }

    Template::render("receive", &info)
}

#[post("/set_profile", data = "<profile>")]
fn set_profile(profile: Form<Profile>, mut cookies: Cookies) -> Redirect {
    cookies.add(Cookie::new("profile", profile.profile.clone()));
    Redirect::to("/")
}