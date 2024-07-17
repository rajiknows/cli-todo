mod cli;
mod db;
mod tests;
mod todo;

use clap::Parser;
use cli::{Cli, Commands};
use std::io::{self, Write};
use todo::User;

fn main() {
    let mut user: Option<User> = None;

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let args = input.trim().split_whitespace();
        let cli = Cli::try_parse_from(args);

        match cli {
            Ok(cli) => match &cli.command {
                Commands::Login { user_name, email } => {
                    user = Some(User::new(user_name.to_string(), email.to_string()));
                    println!("Logged in as {}.", user_name);
                }
                Commands::Logout => {
                    user = None;
                    println!("Logged out.");
                }
                Commands::Exit => {
                    println!("Exiting...");
                    break;
                }
                Commands::Show {
                    all,
                    completed,
                    incomplete,
                    list_name,
                } => {
                    if let Some(user) = &user {
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
                    } else {
                        println!("Please log in first.");
                    }
                }
                Commands::Add { list_name, item } => {
                    if let Some(user) = &mut user {
                        if let Some(list) = user.get_todo_list(list_name) {
                            list.add(item.to_string());
                        } else {
                            user.add_todo_list(list_name.to_string());
                            if let Some(list) = user.get_todo_list(list_name) {
                                list.add(item.to_string());
                            }
                        }
                    } else {
                        println!("Please log in first.");
                    }
                }
                Commands::Complete {
                    list_name,
                    item_number,
                } => {
                    if let Some(user) = &mut user {
                        if let Some(list) = user.get_todo_list(list_name) {
                            list.mark_complete(*item_number);
                        }
                    } else {
                        println!("Please log in first.");
                    }
                }
                Commands::Incomplete {
                    list_name,
                    item_number,
                } => {
                    if let Some(user) = &mut user {
                        if let Some(list) = user.get_todo_list(list_name) {
                            list.mark_incomplete(*item_number);
                        }
                    } else {
                        println!("Please log in first.");
                    }
                }
                Commands::Remove {
                    list_name,
                    item_number,
                } => {
                    if let Some(user) = &mut user {
                        match (list_name, item_number) {
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
                                println!(
                                    "Invalid remove command. Use --help for more information."
                                );
                            }
                        }
                    } else {
                        println!("Please log in first.");
                    }
                }
                Commands::Push => {
                    if let Some(user) = &user {
                        if let Err(e) = user.push_to_db() {
                            println!("Failed to push to db: {}", e);
                        }
                    } else {
                        println!("Please log in first.");
                    }
                }
                Commands::Pull { user_name } => {
                    if let Some(user) = &mut user {
                        match User::pull_from_db(user_name) {
                            Ok(pulled_user) => *user = pulled_user,
                            Err(e) => println!("Failed to pull from db: {}", e),
                        }
                    } else {
                        println!("Please log in first.");
                    }
                }
                _ => {
                    println!("Invalid command. Use --help for more information.");
                }
            },
            Err(err) => {
                println!("Error: {}", err);
                println!("Please use a valid command or type 'exit' to quit.");
            }
        }
    }
}
