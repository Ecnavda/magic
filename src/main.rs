#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use] extern crate rocket;

extern crate rusqlite;
use rusqlite::{ Connection, Result };
use rusqlite::NO_PARAMS;

fn main() {
    println!("Initializing database...");

    match create_schema() {
        Ok(_) => println!("Success"),
        Err(e) => eprintln!("Error: {}", e),
    };

    start_webserver();
}

fn start_webserver() {
    rocket::ignite()
        .mount("/", routes![index])
        .launch();
}

#[get("/")]
fn index() -> &'static str {
    "Magic the Gathering card inventory"
}

fn create_schema() -> Result<()> {
    let conn = Connection::open("mtg.db")?;

    conn.execute(
        "PRAGMA foreign_keys = ON",
        NO_PARAMS,
    )?;

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
            user    TEXT NOT NULL,
            card    TEXT NOT NULL,
            FOREIGN KEY(user) REFERENCES users(rowid),
            FOREIGN KEY(card) REFERENCES cards(rowid)
        )",
        NO_PARAMS,
    )?;

    Ok(())
}