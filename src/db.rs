use rusqlite::{Connection, Result};
use dotenv::dotenv;
use std::env;


pub fn establish_connection() -> Result<Connection> {
    dotenv().ok();
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = Connection::open(url)?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            user_name TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL
        )",
        [],
    )?;
    conn.execute(
        "CREATE TABLE IF NOT EXISTS todos (
            id INTEGER PRIMARY KEY,
            user_id INTEGER NOT NULL,
            list_name TEXT NOT NULL,
            item_number INTEGER NOT NULL,
            title TEXT NOT NULL,
            datetime TEXT NOT NULL,
            is_completed INTEGER NOT NULL,
            FOREIGN KEY(user_id) REFERENCES users(id)
        )",
        [],
    )?;
    Ok(conn)
}
