#[cfg(test)]
mod tests {

    use crate::todo::User;
    use chrono::Local;

    use rusqlite::{params, Connection, Result};

    fn setup_db() -> Result<Connection> {
        let conn = Connection::open_in_memory()?;
        conn.execute(
            "CREATE TABLE users (
                id INTEGER PRIMARY KEY,
                user_name TEXT NOT NULL UNIQUE,
                email TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE todos (
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

    #[test]
    fn test_add_todo_list() {
        let mut user = User::new(
            "test_user".to_string(),
            "test_email@example.com".to_string(),
        );
        user.add_todo_list("Work".to_string());

        assert!(user.todo_lists.contains_key("Work"));
    }

    #[test]
    fn test_add_todo_item() {
        let mut user = User::new(
            "test_user".to_string(),
            "test_email@example.com".to_string(),
        );
        user.add_todo_list("Work".to_string());
        let todo_list = user.get_todo_list(&"Work".to_string()).unwrap();

        todo_list.add("Complete report".to_string());

        assert_eq!(todo_list.todos.len(), 1);
        assert_eq!(todo_list.todos[0].title, "Complete report");
    }

    #[test]
    fn test_mark_complete() {
        let mut user = User::new(
            "test_user".to_string(),
            "test_email@example.com".to_string(),
        );
        user.add_todo_list("Work".to_string());
        let todo_list = user.get_todo_list(&"Work".to_string()).unwrap();

        todo_list.add("Complete report".to_string());
        todo_list.mark_complete(1);

        assert!(todo_list.todos[0].is_completed);
    }

    #[test]
    fn test_mark_incomplete() {
        let mut user = User::new(
            "test_user".to_string(),
            "test_email@example.com".to_string(),
        );
        user.add_todo_list("Work".to_string());
        let todo_list = user.get_todo_list(&"Work".to_string()).unwrap();

        todo_list.add("Complete report".to_string());
        todo_list.mark_complete(1);
        todo_list.mark_incomplete(1);

        assert!(!todo_list.todos[0].is_completed);
    }

    #[test]
    fn test_remove_item() {
        let mut user = User::new(
            "test_user".to_string(),
            "test_email@example.com".to_string(),
        );
        user.add_todo_list("Work".to_string());
        let todo_list = user.get_todo_list(&"Work".to_string()).unwrap();

        todo_list.add("Complete report".to_string());
        todo_list.remove_item(1);

        assert!(todo_list.todos[0].is_deleted);
    }

    #[test]
    fn test_push_to_db() {
        let conn = setup_db().unwrap();
        let mut user = User::new(
            "test_user".to_string(),
            "test_email@example.com".to_string(),
        );
        user.add_todo_list("Work".to_string());
        let todo_list = user.get_todo_list(&"Work".to_string()).unwrap();

        todo_list.add("Complete report".to_string());
        user.push_to_db().unwrap();

        let mut stmt = conn
            .prepare("SELECT user_name, email FROM users WHERE user_name = ?1")
            .unwrap();
        let mut rows = stmt.query(params!["test_user"]).unwrap();
        assert!(rows.next().unwrap().is_some());

        let mut stmt = conn.prepare("SELECT title FROM todos WHERE user_id = (SELECT id FROM users WHERE user_name = ?1)").unwrap();
        let mut rows = stmt.query(params!["test_user"]).unwrap();
        assert!(rows.next().unwrap().is_some());
    }

    #[test]
    fn test_pull_from_db() {
        let conn = setup_db().unwrap();
        let user_name = "test_user".to_string();
        let email = "test_email@example.com".to_string();
        conn.execute(
            "INSERT INTO users (user_name, email) VALUES (?1, ?2)",
            params![user_name, email],
        )
        .unwrap();

        let user_id: i64 = conn
            .query_row(
                "SELECT id FROM users WHERE user_name = ?1",
                params![user_name],
                |row| row.get(0),
            )
            .unwrap();

        conn.execute(
            "INSERT INTO todos (user_id, list_name, item_number, title, datetime, is_completed) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![user_id, "Work", 1, "Complete report", get_current_date_time(), 0],
        ).unwrap();

        let mut user = User::pull_from_db(&user_name).unwrap();
        assert_eq!(user.user_name, "test_user");
        assert_eq!(user.email, "test_email@example.com");
        assert!(user.todo_lists.contains_key("Work"));

        let todo_list = user.get_todo_list(&"Work".to_string()).unwrap();
        assert_eq!(todo_list.todos.len(), 1);
        assert_eq!(todo_list.todos[0].title, "Complete report");
    }

    fn get_current_date_time() -> String {
        let local_now = Local::now();
        local_now.format("%Y-%m-%d %H:%M:%S").to_string()
    }
}
