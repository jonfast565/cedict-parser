extern crate rusqlite;

use std::error::Error;
use rusqlite::{params, Connection, Result};
use crate::parser::CeLine;

fn insert_into_db(line: &CeLine, conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO ce_line (simplified, traditional) VALUES (?1, ?2)",
        params![line.simplified, line.traditional],
    )?;

    let ce_line_id = conn.last_insert_rowid();
    for py in &line.pinyin {
        let pinyin_id = insert_or_get_pinyin(conn, py)?;
        conn.execute(
            "INSERT INTO ce_line_pinyin (ce_line_id, pinyin_id) VALUES (?1, ?2)",
            params![ce_line_id, pinyin_id],
        )?;
    }

    for def in &line.definitions {
        let definition_id = insert_or_get_definition(conn, def)?;
        conn.execute(
            "INSERT INTO ce_line_definitions (ce_line_id, definition_id) VALUES (?1, ?2)",
            params![ce_line_id, definition_id],
        )?;
    }

    Ok(())
}

fn insert_or_get_pinyin(conn: &Connection, pinyin: &str) -> Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO pinyin (pinyin) VALUES (?1)",
        params![pinyin],
    )?;

    let mut stmt = conn.prepare("SELECT id FROM pinyin WHERE pinyin = ?1")?;
    let pinyin_id: i64 = stmt.query_row(params![pinyin], |row| row.get(0))?;

    Ok(pinyin_id)
}

fn insert_or_get_definition(conn: &Connection, definition: &str) -> Result<i64> {
    conn.execute(
        "INSERT OR IGNORE INTO definitions (definition) VALUES (?1)",
        params![definition],
    )?;

    let mut stmt = conn.prepare("SELECT id FROM definitions WHERE definition = ?1")?;
    let definition_id: i64 = stmt.query_row(params![definition], |row| row.get(0))?;

    Ok(definition_id)
}

static DATABASE_NAME: &str =  "data/cedb.sqlite";

pub fn init_db() -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE_NAME)?;

    create_line_table(&conn)?;
    create_pinyin_table(&conn)?;
    create_definitions_table(&conn)?;
    create_line_pinyin_table(&conn)?;
    create_line_definitions_table(&conn)?;

    let close = conn.close();
    if close.is_err() {
        Err(close.err().unwrap().1)
    } else {
        Ok(())
    }
}

pub fn insert_lines(lines: Vec<&CeLine>) -> Result<(), rusqlite::Error> {
    let conn = Connection::open(DATABASE_NAME)?;
    for line in lines {
        let _ = insert_into_db(line, &conn);
    }
    let close = conn.close();
    if close.is_err() {
        Err(close.err().unwrap().1)
    } else {
        Ok(())
    }
}

fn create_line_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ce_line (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            simplified TEXT NOT NULL,
            traditional TEXT NOT NULL
        )",
        [],
    )?;
    Ok(())
}

fn create_pinyin_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS pinyin (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            pinyin TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    Ok(())
}

fn create_definitions_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS definitions (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            definition TEXT NOT NULL UNIQUE
        )",
        [],
    )?;
    Ok(())
}

fn create_line_pinyin_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ce_line_pinyin (
            ce_line_id INTEGER NOT NULL,
            pinyin_id INTEGER NOT NULL,
            FOREIGN KEY (ce_line_id) REFERENCES ce_line(id),
            FOREIGN KEY (pinyin_id) REFERENCES pinyin(id),
            PRIMARY KEY (ce_line_id, pinyin_id)
        )",
        [],
    )?;
    Ok(())
}

fn create_line_definitions_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ce_line_definitions (
            ce_line_id INTEGER NOT NULL,
            definition_id INTEGER NOT NULL,
            FOREIGN KEY (ce_line_id) REFERENCES ce_line(id),
            FOREIGN KEY (definition_id) REFERENCES definitions(id),
            PRIMARY KEY (ce_line_id, definition_id)
        )",
        [],
    )?;
    Ok(())
}
