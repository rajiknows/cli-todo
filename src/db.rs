use rusqlite::{Connection, Result};

pub fn establish_connection() -> Result<Connection> {
    let conn = Connection::open("todo.db")?;
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
