use rusqlite::{ Connection, Result };
use rusqlite::NO_PARAMS;
use rocket::request::FromForm;

#[derive(FromForm)]
pub struct CardSets {
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
    pub name: String,
    // Rocket FromFormValue Option<T> always return
    // successfully if validation succeeds.
    // Returns None ONLY if the value could not be
    // validated.
    pub card_set: i32,
    pub card_number: i32,
    pub color: String,
    pub cmc: i32,
}

pub fn create_schema() -> Result<()> {
    let conn = Connection::open("mtg.db")?;
    // SQLite has only the following types
    // NULL, INTEGER, REAL, TEXT, and BLOB
    // 6 types of INT though.
    // No primitive "date" type but SQLite
    // includes a date() function to provide
    // a date/time stamp.

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
        "CREATE TABLE IF NOT EXISTS card_sets (
            name        TEXT NOT NULL,
            release    TEXT NOT NULL
        )",
        NO_PARAMS,
    )?;

    // Consider UNIQUE on card_set and card_number
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cards (
            name        TEXT NOT NULL,
            card_set    INT NOT NULL,
            card_number INT NOT NULL,
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

/*
// Consider accepting array for values
pub fn sql_insert(table: &str, name: &str) -> Result<()> {
    let conn = Connection::open("mtg.db")?;
    /*
    let mut stmt = conn.prepare(
        "INSERT INTO users (email) VALUES ('ecnavda@gmail.com')"
    )?;
    stmt.execute(&[])?;
    */
    Ok(())
}
*/

pub fn insert_user(user: &Users) -> Result<()> {
    let conn = Connection::open("mtg.db")?;
    let mut stmt1 = conn.prepare(
        "INSERT INTO users (email) VALUES (?)"
    )?;
    let mut stmt2 = conn.prepare(
        "INSERT INTO users (email, name) VALUES (?1, ?2)"
    )?;
    match &user.name {
        Some(x) => {
            stmt2.execute(&[user.email.as_str(), x.as_str()])?
        },
        None => {
            stmt1.execute(&[user.email.as_str()])?
        },
    };
    Ok(())
}

pub fn insert_card_set(card_set: &CardSets) -> Result<()> {
    let conn = Connection::open("mtg.db")?;

    let mut stmt1 = conn.prepare(
        "INSERT INTO card_sets (name) VALUES (?)"
    )?;
    let mut stmt2 = conn.prepare(
        "INSERT INTO card_sets (name, release) VALUES (?1, ?2)"
    )?;

    match &card_set.release {
        Some(x) => {
            println!("Entered stmt2 block");
            stmt2.execute(&[card_set.name.as_str(), x.as_str()])?
        },
        None => {
            stmt1.execute(&[card_set.name.as_str()])?
        },
    };

    Ok(())
}
// TODO - Either use Cards struct or create a new struct
// to pass values into statement.execute()
pub fn insert_card(card: &Cards) -> Result<()> {
    

    let conn = Connection::open("mtg.db")?;
    let mut stmt = conn.prepare(
        "INSERT INTO cards (name, card_set) VALUES (?1, ?2)"
    )?;
    
    // Statement.execute() takes an iterator
    

    Ok(())
}

pub fn select_card_sets() -> Result<Vec<(i32, String)>> {
    let conn = Connection::open("mtg.db")?;
    let mut stmt = conn.prepare(
        "SELECT rowid,name FROM card_sets"
    )?;
    let mut rows = stmt.query(NO_PARAMS)?;

    let mut names = Vec::new();
    while let Some(row) = rows.next()? {
        names.push((row.get(0)?, row.get(1)?));
    }
    Ok(names)
}

pub fn select_users() -> Result<Vec<String>> {
    let conn = Connection::open("mtg.db")?;
    let mut stmt = conn.prepare(
        "SELECT email FROM users"
    )?;
    let mut rows = stmt.query(NO_PARAMS)?;

    let mut emails = Vec::new();
    while let Some(row) =rows.next()? {
        emails.push(row.get(0)?);
    }
    Ok(emails)
}