use rusqlite::{params, Connection, Result};

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

pub fn clear_user_data(conn: &Connection, user_name: &str) -> Result<()> {
    conn.execute(
        "DELETE FROM todos WHERE user_id = (SELECT id FROM users WHERE user_name = ?1)",
        params![user_name],
    )?;
    conn.execute("DELETE FROM users WHERE user_name = ?1", params![user_name])?;
    Ok(())
}
