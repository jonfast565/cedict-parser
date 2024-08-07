extern crate rusqlite;

use rusqlite::{params, Connection, Result, backup};
use crate::parser::CeLine;

fn insert_into_db(line: &CeLine, conn: &Connection) -> Result<()> {
    conn.execute(
        "INSERT INTO ce_line (simplified, traditional) VALUES (?1, ?2)",
        params![line.simplified, line.traditional],
    )?;

    let ce_line_id = conn.last_insert_rowid();
    let mut counter = 1;
    for py in &line.pinyin {
        let pinyin_id = insert_or_get_pinyin(conn, py)?;
        conn.execute(
            "INSERT INTO ce_line_pinyin (ordering, ce_line_id, pinyin_id) VALUES (?1, ?2, ?3)",
            params![counter, ce_line_id, pinyin_id],
        )?;
        counter += 1;
    }

    counter = 1;
    for def in &line.definitions {
        let definition_id = insert_or_get_definition(conn, def)?;
        conn.execute(
            "INSERT INTO ce_line_definitions (ordering, ce_line_id, definition_id) VALUES (?1, ?2, ?3)",
            params![counter, ce_line_id, definition_id],
        )?;
        counter += 1;
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

pub fn insert_lines(lines: Vec<&CeLine>) -> Result<(), rusqlite::Error> {
    let conn = Connection::open_in_memory()?;

    create_line_table(&conn)?;
    create_pinyin_table(&conn)?;
    create_definitions_table(&conn)?;
    create_line_characters(&conn)?;
    create_line_pinyin_table(&conn)?;
    create_line_definitions_table(&conn)?;

    let len = lines.len();
    let mut counter: usize = 0;
    for line in lines {
        insert_into_db(line, &conn)?;
        println!("{}/{} rows inserted in database, {:.2}% completed", counter, len, (counter as f32 / len as f32) * 100.0);
        counter += 1;
    }
    //conn.execute("COMMIT", [])?;
    conn.cache_flush()?;

    let mut dst = Connection::open(DATABASE_NAME)?;
    let backup = backup::Backup::new(&conn, &mut dst)?;
    backup.run_to_completion(100, std::time::Duration::from_millis(25), None)?;

    Ok(())
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
            ordering INTEGER NOT NULL,
            ce_line_id INTEGER NOT NULL,
            pinyin_id INTEGER NOT NULL,
            FOREIGN KEY (ce_line_id) REFERENCES ce_line(id),
            FOREIGN KEY (pinyin_id) REFERENCES pinyin(id),
            PRIMARY KEY (ordering, ce_line_id, pinyin_id)
        )",
        [],
    )?;
    Ok(())
}

fn create_line_definitions_table(conn: &Connection) -> Result<(), rusqlite::Error> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS ce_line_definitions (
            ordering INTEGER NOT NULL,
            ce_line_id INTEGER NOT NULL,
            definition_id INTEGER NOT NULL,
            FOREIGN KEY (ce_line_id) REFERENCES ce_line(id),
            FOREIGN KEY (definition_id) REFERENCES definitions(id),
            PRIMARY KEY (ordering, ce_line_id, definition_id)
        )",
        [],
    )?;
    Ok(())
}
