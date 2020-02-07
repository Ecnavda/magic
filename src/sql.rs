use rusqlite::{ Connection, Result };
use rusqlite::NO_PARAMS;
use rusqlite::types::Value as SQLValue;
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

#[derive(FromForm, Debug)]
pub struct Cards {
    pub name: String,
    // Option<T> could be used to detect the presence
    // of a value received from a form.
    pub card_set: i64,
    pub card_number: Option<i64>,
    pub rarity: String,
    pub red: bool,
    pub blue: bool,
    pub black: bool,
    pub green: bool,
    pub white: bool,
    pub colorless: bool,
    pub cmc: Option<i64>,
}

impl Cards {
    fn sql_output(&self) -> (Vec<&str>, Vec<SQLValue>) {
        let mut keys: Vec<&str> = vec!["name", "card_set", "rarity"];
        let mut values: Vec<SQLValue> = vec![SQLValue::Text(self.name.clone()), SQLValue::Integer(self.card_set), SQLValue::Text(self.rarity.clone())];

        if let Some(num) = self.card_number {
            keys.push("card_number");
            values.push(SQLValue::Integer(num));
        }

        let mut color_string = String::new();
        if self.red {
            color_string.push_str("red ");
        }
        if self.blue {
            color_string.push_str("blue ");
        }
        if self.black {
            color_string.push_str("black ");
        }
        if self.green {
            color_string.push_str("green ");
        }
        if self.white {
            color_string.push_str("white ");
        }
        if self.colorless {
            color_string.push_str("colorless ")
        }
        if !(color_string.is_empty()) {
            color_string.pop();
            keys.push("color");
            values.push(SQLValue::Text(color_string));
        }
        if let Some(cmc) = self.cmc {
            keys.push("cmc");
            values.push(SQLValue::Integer(cmc));
        }

        (keys, values)
    }

    pub fn color_output(&mut self) {
        
    }
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
            user_id     TEXT    NOT NULL,
            name        TEXT,
            PRIMARY KEY(user_id)
        ) WITHOUT ROWID",
        NO_PARAMS,
    )?;

    // Implicit rowid created by SQLite cannot be used
    // as a foreign key. Creating 'id' row for that purpose.
    // INTEGER Primary key gets AUTOINCREMENT by default.
    conn.execute(
        "CREATE TABLE IF NOT EXISTS card_sets (
            card_set_id INTEGER NOT NULL, 
            name        TEXT    NOT NULL,
            release     TEXT    NOT NULL,
            PRIMARY KEY(card_set_id)
        )",
        NO_PARAMS,
    )?;

    // Consider UNIQUE on card_set and card_number
    conn.execute(
        "CREATE TABLE IF NOT EXISTS cards (
            card_id     INT     NOT NULL,
            name        TEXT    NOT NULL,
            card_set_id INT     NOT NULL,
            rarity      TEXT    NOT NULL,
            card_number INT,
            color       TEXT,
            cmc         INT,
            PRIMARY KEY(card_id),
            FOREIGN KEY(card_set_id) REFERENCES card_sets(card_set_id)
        )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS inventory (
            inv_id  INT     NOT NULL,
            user_id TEXT    NOT NULL,
            card_id INTEGER NOT NULL,
            PRIMARY KEY(inv_id),
            FOREIGN KEY(user_id) REFERENCES users(user_id),
            FOREIGN KEY(card_id) REFERENCES cards(card_id)
        )",
        NO_PARAMS,
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS deck_sets (
            deck_set_id INT     NOT NULL,
            name        TEXT    NOT NULL,
            user_id     TEXT    NOT NULL,
            PRIMARY KEY(deck_set_id)
        )",
        NO_PARAMS
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS decks (
            deck_id     INT     NOT NULL,
            deck_set_id INT     NOT NULL,
            card_id     INT     NOT NULL,
            PRIMARY KEY(deck_id),
            FOREIGN KEY(deck_set_id) REFERENCES deck_sets(deck_set_id),
            FOREIGN KEY(card_id) REFERENCES cards(card_id)
        )",
        NO_PARAMS
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
        "INSERT INTO users (user_id) VALUES (?)"
    )?;
    let mut stmt2 = conn.prepare(
        "INSERT INTO users (user_id, name) VALUES (?1, ?2)"
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
    
    // This pattern is used to create a Vec that holds one type of multiple types
    let (names, values) = card.sql_output();
    let mut statement = String::from("INSERT INTO cards (");
    for name in names {
        statement.push_str(name);
        statement.push_str(",")
    }
    statement.pop();
    statement.push_str(") ");
    
    let mut prep1 = statement.clone();
    prep1.push_str("VALUES (?1, ?2, ?3)");
    let mut prep2 = statement.clone();
    prep2.push_str("VALUES (?1, ?2, ?3, ?4)");
    let mut prep3 = statement.clone();
    prep3.push_str("VALUES (?1, ?2, ?3, ?4, ?5)");
    let mut prep4 = statement.clone();
    prep4.push_str("VALUES (?1, ?2, ?3, ?4, ?5, ?6)");

    let conn = Connection::open("mtg.db")?;

    // Connection.prepare() accepts &str, build with column names first
    // prepare() also checks for number of fields and values to be the same.
    match values.len() {
        3 => {
            let mut stmt1 = conn.prepare(prep1.as_str())?;
            stmt1.execute(&values)?;
        },
        4 => {
            let mut stmt2 = conn.prepare(prep2.as_str())?;
            stmt2.execute(&values)?;
        },
        5 => {
            let mut stmt3 = conn.prepare(prep3.as_str())?;
            stmt3.execute(&values)?;
        },
        6 => {
            let mut stmt4 = conn.prepare(prep4.as_str())?;
            stmt4.execute(&values)?;
        },
        _ => (),
        };

    Ok(())
}

pub fn select_cards() -> Result<Vec<(String, Vec<String>, String)>> {
    let conn = Connection::open("mtg.db")?;
    let mut stmt = conn.prepare(
        "SELECT name,color,rarity FROM cards"
    )?;
    let mut rows = stmt.query(NO_PARAMS)?;

    let mut info: Vec<(String, Vec<String>, String)> = Vec::new();
    while let Some(row) = rows.next()? {
        // name, colors
        let color: String = row.get(1)?;
        let colors = color.split_whitespace()
                                        .map(|slice| slice.to_string())
                                        .collect();
        info.push((row.get(0)?, colors, row.get(2)?));
    }

    Ok(info)
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
        "SELECT user_id FROM users"
    )?;
    let mut rows = stmt.query(NO_PARAMS)?;

    let mut emails = Vec::new();
    while let Some(row) =rows.next()? {
        emails.push(row.get(0)?);
    }
    Ok(emails)
}