use rusqlite::{ Connection, Result };
use rusqlite::NO_PARAMS;
use rocket::request::FromForm;

#[derive(FromForm)]
pub struct CardSet {
    pub name: String,
    pub release: Option<String>,
}

#[derive(FromForm)]
pub struct Users {
    pub email: String,
    pub name: Option<String>,
}

#[derive(FromForm)]
pub struct Cards {
    pub card_set: String,
    pub card_number: i32,
    pub name: String,
    pub color: String,
    pub cmc: i32,
}

pub fn create_schema() -> Result<()> {
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
            name    TEXT,
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

// Consider accepting array for values
pub fn sql_insert(table: &str, name: &str) -> Result<()> {
    let conn = Connection::open("mtg.db")?;

    let mut stmt = conn.prepare(
        "INSERT INTO users (email) VALUES ('ecnavda@gmail.com')"
    )?;
    //stmt.execute(&[])?;
    
    Ok(())
}

pub fn insert_user(user: Users) -> Result<()> {
    let conn = Connection::open("mtg.db")?;
    let stmt1 = conn.prepare(
        "INSERT INTO users (email) VALUES (?)"
    )?;
    let stmt2 = conn.prepare(
        "INSERT INTO users (email, name) VALUES (?1, ?2)"
    )?;
    match user.name {
        Some(x) => {
            stmt2.execute(&[user.email.as_str(), x.as_str()])?
        },
        None => {
            stmt1.execute(&[user.email.as_str()])?
        },
    };
    Ok(())
}