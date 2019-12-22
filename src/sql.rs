use rusqlite::{ Connection, Result };
use rusqlite::NO_PARAMS;

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

// Consider accepting array for values
pub fn sql_insert(table: &str, name: &str) -> Result<()> {
    let conn = Connection::open("mtg.db")?;
    let mut stmt = conn.prepare(
        "INSERT INTO ?1 (email) VALUES (?2)"
    )?;
    stmt.execute(&[table, name])?;

    Ok(())
}