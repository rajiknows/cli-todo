mod cli;
mod db;
mod todo;
mod tests;

use clap::Parser;
use cli::{Cli, Commands};
use std::io::{self, Write};
use todo::{TodoList, User};

fn main() {
    let mut user = User::new("Raj".to_string(), "rajesh@example.com".to_string());

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args = input.trim().split_whitespace();
        let cli = Cli::try_parse_from(args);

        match cli {
            Ok(cli) => match &cli.command {
                Commands::Show {
                    all,
                    completed,
                    incomplete,
                    list_name,
                } => {
                    if *all {
                        user.show_lists();
                    } else if *completed {
                        user.show_completed_items(list_name.as_ref());
                    } else if *incomplete {
                        user.show_incomplete_items(list_name.as_ref());
                    } else if let Some(name) = list_name {
                        user.show_list_items(name);
                    } else {
                        println!("Invalid show command. Use --help for more information.");
                    }
                }
                Commands::Add { list_name, item } => {
                    if let Some(list) = user.get_todo_list(list_name) {
                        list.add(item.to_string());
                    } else {
                        user.add_todo_list(list_name.to_string());
                        if let Some(list) = user.get_todo_list(list_name) {
                            list.add(item.to_string());
                        }
                    }
                }
                Commands::Complete {
                    list_name,
                    item_number,
                } => {
                    if let Some(list) = user.get_todo_list(list_name) {
                        list.mark_complete(*item_number);
                    }
                }
                Commands::Incomplete {
                    list_name,
                    item_number,
                } => {
                    if let Some(list) = user.get_todo_list(list_name) {
                        list.mark_incomplete(*item_number);
                    }
                }
                Commands::Remove {
                    list_name,
                    item_number,
                } => match (list_name, item_number) {
                    (Some(name), Some(number)) => {
                        if let Some(list) = user.get_todo_list(name) {
                            list.remove_item(*number);
                        }
                    }
                    (Some(name), None) => {
                        user.todo_lists.remove(name);
                    }
                    (None, None) => {
                        user.todo_lists.clear();
                    }
                    _ => {
                        println!("Invalid remove command. Use --help for more information.");
                    }
                },
                Commands::Push => {
                    if let Err(e) = user.push_to_db() {
                        println!("Failed to push to db: {}", e);
                    }
                }
                Commands::Pull { user_name } => {
                    match User::pull_from_db(user_name) {
                        Ok(pulled_user) => user = pulled_user,
                        Err(e) => println!("Failed to pull from db: {}", e),
                    }
                }
                Commands::Exit => {
                    println!("Exiting...");
                    break;
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                println!("Please use a valid command or type 'exit' to quit.");
            }
        }
    }
}
