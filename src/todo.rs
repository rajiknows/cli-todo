use chrono::prelude::*;
use std::collections::HashMap;

pub struct TodoItem {
    pub item_number: usize,
    pub title: String,
    pub datetime: String,
    pub is_completed: bool,
}

impl TodoItem {
    pub fn new(item_number: usize, title: String) -> Self {
        TodoItem {
            item_number,
            title,
            datetime: get_current_date_time(),
            is_completed: false,
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
        }
    }

    pub fn mark_incomplete(&mut self, item_number: usize) {
        if let Some(todo) = self
            .todos
            .iter_mut()
            .find(|todo| todo.item_number == item_number)
        {
            todo.is_completed = false;
        }
    }

    pub fn remove_item(&mut self, item_number: usize) {
        self.todos.retain(|todo| todo.item_number != item_number);
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
}

fn get_current_date_time() -> String {
    let local_now = Local::now();
    local_now.format("%Y-%m-%d %H:%M:%S").to_string()
}
