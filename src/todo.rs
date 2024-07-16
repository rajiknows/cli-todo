use crate::db::{establish_connection};
use chrono::prelude::*;
use rusqlite::{params, Result};
use std::collections::HashMap;

pub struct TodoItem {
    pub item_number: usize,
    pub title: String,
    pub datetime: String,
    pub is_completed: bool,
    pub is_deleted: bool,
    pub is_modified: bool,
}

impl TodoItem {
    pub fn new(item_number: usize, title: String) -> Self {
        TodoItem {
            item_number,
            title,
            datetime: get_current_date_time(),
            is_completed: false,
            is_deleted: false,
            is_modified: false,
        }
    }
}

pub struct TodoList {
    pub title: String,
    pub todos: Vec<TodoItem>,
}

impl TodoList {
    pub fn new(title: String) -> Self {
        TodoList {
            title,
            todos: Vec::new(),
        }
    }

    pub fn add(&mut self, title: String) {
        let item_number = self.todos.len() + 1;
        let todo = TodoItem::new(item_number, title);
        self.todos.push(todo);
    }

    pub fn mark_complete(&mut self, item_number: usize) {
        if let Some(todo) = self
            .todos
            .iter_mut()
            .find(|todo| todo.item_number == item_number)
        {
            todo.is_completed = true;
            todo.is_modified = true;
        }
    }

    pub fn mark_incomplete(&mut self, item_number: usize) {
        if let Some(todo) = self
            .todos
            .iter_mut()
            .find(|todo| todo.item_number == item_number)
        {
            todo.is_completed = false;
            todo.is_modified = true;
        }
    }

    pub fn remove_item(&mut self, item_number: usize) {
        if let Some(todo) = self
            .todos
            .iter_mut()
            .find(|todo| todo.item_number == item_number)
        {
            todo.is_deleted = true;
        }
    }
}

pub struct User {
    pub user_name: String,
    pub email: String,
    pub todo_lists: HashMap<String, TodoList>,
}

impl User {
    pub fn new(user_name: String, email: String) -> Self {
        User {
            user_name,
            email,
            todo_lists: HashMap::new(),
        }
    }

    pub fn add_todo_list(&mut self, list_name: String) {
        self.todo_lists
            .insert(list_name.clone(), TodoList::new(list_name));
    }

    pub fn get_todo_list(&mut self, list_name: &String) -> Option<&mut TodoList> {
        self.todo_lists.get_mut(list_name)
    }

    pub fn show_lists(&self) {
        for (list_name, list) in &self.todo_lists {
            println!("List: {}", list_name);
            for todo in &list.todos {
                println!(
                    "{}. {} ({} - Completed: {})",
                    todo.item_number, todo.title, todo.datetime, todo.is_completed
                );
            }
        }
    }

    pub fn show_list_items(&self, list_name: &String) {
        if let Some(list) = self.todo_lists.get(list_name) {
            for todo in &list.todos {
                println!(
                    "{}. {} ({} - Completed: {})",
                    todo.item_number, todo.title, todo.datetime, todo.is_completed
                );
            }
        }
    }

    pub fn show_completed_items(&self, list_name: Option<&String>) {
        match list_name {
            Some(name) => {
                if let Some(list) = self.todo_lists.get(name) {
                    for todo in list.todos.iter().filter(|todo| todo.is_completed) {
                        println!(
                            "{}. {} ({} - Completed: {})",
                            todo.item_number, todo.title, todo.datetime, todo.is_completed
                        );
                    }
                }
            }
            None => {
                for (list_name, list) in &self.todo_lists {
                    println!("List: {}", list_name);
                    for todo in list.todos.iter().filter(|todo| todo.is_completed) {
                        println!(
                            "{}. {} ({} - Completed: {})",
                            todo.item_number, todo.title, todo.datetime, todo.is_completed
                        );
                    }
                }
            }
        }
    }

    pub fn show_incomplete_items(&self, list_name: Option<&String>) {
        match list_name {
            Some(name) => {
                if let Some(list) = self.todo_lists.get(name) {
                    for todo in list.todos.iter().filter(|todo| !todo.is_completed) {
                        println!(
                            "{}. {} ({} - Completed: {})",
                            todo.item_number, todo.title, todo.datetime, todo.is_completed
                        );
                    }
                }
            }
            None => {
                for (list_name, list) in &self.todo_lists {
                    println!("List: {}", list_name);
                    for todo in list.todos.iter().filter(|todo| !todo.is_completed) {
                        println!(
                            "{}. {} ({} - Completed: {})",
                            todo.item_number, todo.title, todo.datetime, todo.is_completed
                        );
                    }
                }
            }
        }
    }

    pub fn push_to_db(&self) -> Result<()> {
        let conn = establish_connection()?;
        
        // Insert user if not exists
        conn.execute(
            "INSERT OR IGNORE INTO users (user_name, email) VALUES (?1, ?2)",
            params![self.user_name, self.email],
        )?;
        
        // Get the user ID
        let user_id: i64 = conn.query_row(
            "SELECT id FROM users WHERE user_name = ?1",
            params![self.user_name],
            |row| row.get(0),
        )?;
        
        for (list_name, list) in &self.todo_lists {
            for todo in &list.todos {
                if todo.is_deleted {
                    conn.execute(
                        "DELETE FROM todos WHERE user_id = ?1 AND list_name = ?2 AND item_number = ?3",
                        params![user_id, list_name, todo.item_number],
                    )?;
                } else if todo.is_modified {
                    conn.execute(
                        "UPDATE todos SET title = ?1, datetime = ?2, is_completed = ?3 WHERE user_id = ?4 AND list_name = ?5 AND item_number = ?6",
                        params![todo.title, todo.datetime, todo.is_completed as i32, user_id, list_name, todo.item_number],
                    )?;
                } else {
                    conn.execute(
                        "INSERT OR IGNORE INTO todos (user_id, list_name, item_number, title, datetime, is_completed)
                        VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
                        params![
                            user_id,
                            list_name,
                            todo.item_number,
                            todo.title,
                            todo.datetime,
                            todo.is_completed as i32
                        ],
                    )?;
                }
            }
        }
        Ok(())
    }

    pub fn pull_from_db(user_name: &str) -> Result<Self> {
        let conn = establish_connection()?;
        let mut stmt = conn.prepare("SELECT id, email FROM users WHERE user_name = ?1")?;
        let user_row = stmt.query_row(params![user_name], |row| {
            Ok((row.get::<_, i64>(0)?, row.get::<_, String>(1)?))
        })?;
        let user_id = user_row.0;
        let email = user_row.1;

        let mut user = User {
            user_name: user_name.to_string(),
            email,
            todo_lists: HashMap::new(),
        };

        let mut stmt = conn.prepare(
            "SELECT list_name, item_number, title, datetime, is_completed FROM todos WHERE user_id = ?1",
        )?;
        let todo_iter = stmt.query_map(params![user_id], |row| {
            Ok((
                row.get::<_, String>(0)?,
                TodoItem {
                    item_number: row.get(1)?,
                    title: row.get(2)?,
                    datetime: row.get(3)?,
                    is_completed: row.get::<_, i32>(4)? != 0,
                    is_deleted: false,
                    is_modified: false,
                },
            ))
        })?;

        for todo in todo_iter {

            let (list_name, todo) = todo?;

            user.todo_lists
                .entry(list_name.clone())
                .or_insert_with(|| TodoList::new(list_name.clone()))
                .todos
                .push(todo);
        }

        Ok(user)
    }
}

fn get_current_date_time() -> String {
    let local_now = Local::now();
    local_now.format("%Y-%m-%d %H:%M:%S").to_string()
}
