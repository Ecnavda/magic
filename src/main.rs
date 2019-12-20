#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;
use rocket::request::{ FromForm, Form };
use rocket_contrib::templates::Template;
use rusqlite::{ Connection, Result };
use rusqlite::NO_PARAMS;
use serde::{ Serialize, Deserialize };
use std::collections::HashMap;

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

    match create_schema() {
        Ok(_) => println!("Success"),
        Err(e) => eprintln!("Error: {}", e),
    };

    start_webserver().launch();
}

fn start_webserver() -> rocket::Rocket {
    // Fairings (middleware) must be attached to rocket
    // before launching.
    rocket::ignite()
        .mount("/", routes![index])
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

    let context: HashMap<&str, &str> = [("result", "nothing")].iter().cloned().collect();
    Template::render("receive", &context)
}

fn create_schema() -> Result<()> {
    let conn = Connection::open("mtg.db")?;

    // SQLite has foreign keys off by default
    conn.execute(
        "PRAGMA foreign_keys = ON",
        NO_PARAMS,
    )?;

    // SQLite adds a rowid column as the primary key
    // by default. Setting email as the primary key
    // and disabling the rowid column for this table.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            email   TEXT NOT NULL,
            name    TEXT NOT NULL,
            PRIMARY KEY(email)
        ) WITHOUT ROWID",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS card_set (
            name        TEXT NOT NULL,
            released    TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    // Consider UNIQUE on card_set and card_number
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cards (
            card_set    TEXT NOT NULL,
            card_number INT NOT NULL,
            name        TEXT NOT NULL,
            color       TEXT NOT NULL,
            cmc         INT,
            FOREIGN KEY(card_set) REFERENCES card_sets(rowid)
        )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS inventory (
            email    TEXT NOT NULL,
            card    TEXT NOT NULL,
            FOREIGN KEY(email) REFERENCES users(email),
            FOREIGN KEY(card) REFERENCES cards(rowid)
        )",
        NO_PARAMS,
    )?;

    Ok(())
}