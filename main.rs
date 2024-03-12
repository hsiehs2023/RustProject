//! # Task Manager
//!
//! A console-based task management application.
//!
//! ## Concepts
//!
//! This module provides a simple task management system. Tasks are represented by the `Task` struct,
//! and the application allows users to add, remove, list, search, and update tasks using a command-line interface.
//!
//! ## Important Functions
//!
//! - `save_tasks(tasks: &[Task]) -> Result<(), Box<dyn Error>>`: Saves a vector of tasks to a JSON file.
//! - `load_tasks() -> Result<Vec<Task>, Box<dyn Error>>`: Loads tasks from a JSON file.
//! - `update_task(matches: &ArgMatches, tasks: &mut Vec<Task>) -> Result<(), &'static str>`: Updates a task based on command-line arguments.
//! - `list_tasks_by_project(tasks: &[Task], project_name: &str)`: Lists all tasks with the same project name.
//! - `list_tasks_by_status(tasks: &[Task], status: &str)`: Lists all tasks with the same status.
//! - `list_tasks_by_priority(tasks: &[Task], priority: u8)`: Lists all tasks with the same priority number.
//! - `main()`: The entry point of the application, which handles command-line arguments and performs corresponding actions on tasks.
//!
//! ## Data Types
//!
//! - `Task`: Represents a task with title, description, priority, status, and project fields.
//!
//! ## Traits
//!
//! - `Deserialize`, `Serialize`, `PartialEq`: Implemented for the `Task` struct to enable serialization, deserialization, and equality comparisons.
//!
//!  ## Dependencies
//!
//! - `clap`: Used for parsing command-line arguments.
//! - `serde`: Used for JSON serialization and deserialization.

use clap::ArgMatches;
use clap::{App, Arg, SubCommand};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Debug, Deserialize, Serialize, PartialEq)]

/// `Task`: Represents a task with title, description, priority, status, and project fields.
struct Task {
    title: String,
    description: String,
    priority: u8,
    status: String,
    project: String,
}

/// Saves a vector of tasks to a JSON file.
fn save_tasks(tasks: &[Task]) -> Result<(), Box<dyn Error>> {
    let serialized = serde_json::to_string_pretty(tasks)?;
    fs::write("tasks.json", serialized)?;
    Ok(())
}

/// Loads tasks from a JSON file.
fn load_tasks() -> Result<Vec<Task>, Box<dyn Error>> {
    let contents = fs::read_to_string("tasks.json").unwrap_or_default();
    let tasks: Vec<Task> = serde_json::from_str(&contents)?;
    Ok(tasks)
}

/// Lists all tasks with the same project name.
fn list_tasks_by_project(tasks: &[Task], project_name: &str) {
    let filtered_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|task| task.project == project_name)
        .collect();

    for (index, task) in filtered_tasks.iter().enumerate() {
        println!("Task {}: {:#?}", index + 1, task);
    }
}

/// Lists all tasks with the same status.
fn list_tasks_by_status(tasks: &[Task], status: &str) {
    let filtered_tasks: Vec<&Task> = tasks.iter().filter(|task| task.status == status).collect();

    for (index, task) in filtered_tasks.iter().enumerate() {
        println!("Task {}: {:#?}", index + 1, task);
    }
}

/// Lists all tasks with the same priority number.
fn list_tasks_by_priority(tasks: &[Task], priority: u8) {
    let filtered_tasks: Vec<&Task> = tasks
        .iter()
        .filter(|task| task.priority == priority)
        .collect();

    for (index, task) in filtered_tasks.iter().enumerate() {
        println!("Task {}: {:#?}", index + 1, task);
    }
}

/// Updates a task based on command-line arguments.
fn update_task(matches: &ArgMatches, tasks: &mut Vec<Task>) -> Result<(), &'static str> {
    let title = matches.value_of("title").unwrap();

    if let Some(task) = tasks.iter_mut().find(|t| t.title == title) {
        if let Some(new_description) = matches.value_of("description") {
            task.description = new_description.to_string();
        }
        if let Some(new_priority) = matches.value_of("priority") {
            task.priority = new_priority.parse::<u8>().map_err(|_| "Invalid priority")?;
        }
        if let Some(new_status) = matches.value_of("status") {
            task.status = new_status.to_string();
        }
        if let Some(new_project) = matches.value_of("project") {
            task.project = new_project.to_string();
        }
        save_tasks(tasks).map_err(|_| "Failed to save tasks")?;
        Ok(())
    } else {
        Err("Task not found")
    }
}

/// The entry point of the application, which handles command-line arguments and performs corresponding actions on tasks.
fn main() {
    let matches = App::new("Task Manager")
        .version("1.0")
        .author("Me")
        .about("A console-based task management application")
        .subcommand(
            SubCommand::with_name("add")
                .about("Add a new task")
                .arg(
                    Arg::with_name("title")
                        .index(1)
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("description")
                        .index(2)
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("priority")
                        .index(3)
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("status")
                        .index(4)
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("project")
                        .index(5)
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("remove").about("Remove a task").arg(
                Arg::with_name("title")
                    .index(1)
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(SubCommand::with_name("list").about("List all tasks"))
        .subcommand(
            SubCommand::with_name("search")
                .about("Search for tasks by title or description")
                .arg(
                    Arg::with_name("query")
                        .index(1)
                        .required(true)
                        .takes_value(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list-by-project")
                .about("List tasks by project")
                .arg(
                    Arg::with_name("project")
                        .long("project")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list-by-status")
                .about("List tasks by status")
                .arg(
                    Arg::with_name("status")
                        .long("status")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("list-by-priority")
                .about("List tasks by priority")
                .arg(
                    Arg::with_name("priority")
                        .long("priority")
                        .takes_value(true)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("update")
                .about("Update a task")
                .arg(
                    Arg::with_name("title")
                        .index(1)
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("description")
                        .long("description")
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("priority")
                        .long("priority")
                        .takes_value(true),
                )
                .arg(Arg::with_name("status").long("status").takes_value(true))
                .arg(Arg::with_name("project").long("project").takes_value(true)),
        )
        .get_matches();

    let mut tasks = load_tasks().unwrap_or_else(|_| vec![]);

    match matches.subcommand() {
        ("add", Some(sub_m)) => {
            let title = sub_m.value_of("title").unwrap();
            let description = sub_m.value_of("description").unwrap();
            let priority = sub_m.value_of("priority").unwrap().parse::<u8>().unwrap();
            let status = sub_m.value_of("status").unwrap();
            let project = sub_m.value_of("project").unwrap();

            let new_task = Task {
                title: title.to_string(),
                description: description.to_string(),
                priority,
                status: status.to_string(),
                project: project.to_string(),
            };

            tasks.push(new_task);
            save_tasks(&tasks).unwrap();
            println!("Task added successfully!");
        }
        ("remove", Some(sub_m)) => {
            let title = sub_m.value_of("title").unwrap();
            tasks.retain(|task| task.title != title);
            save_tasks(&tasks).unwrap();
            println!("Task removed successfully!");
        }
        ("list", _) => {
            for (index, task) in tasks.iter().enumerate() {
                println!("Task {}: {:#?}", index + 1, task);
            }
        }
        ("list-by-project", Some(sub_m)) => {
            if let Some(project_name) = sub_m.value_of("project") {
                list_tasks_by_project(&tasks, project_name);
            } else {
                println!("Please provide a project name with the --project option");
            }
        }
        ("list-by-status", Some(sub_m)) => {
            if let Some(status) = sub_m.value_of("status") {
                list_tasks_by_status(&tasks, status);
            } else {
                println!("Please provide a status with the --status option");
            }
        }
        ("list-by-priority", Some(sub_m)) => {
            if let Some(priority) = sub_m.value_of("priority") {
                if let Ok(priority) = priority.parse::<u8>() {
                    list_tasks_by_priority(&tasks, priority);
                } else {
                    println!(
                        "Invalid priority value. Please provide a valid integer for priority."
                    );
                }
            } else {
                println!("Please provide a priority with the --priority option");
            }
        }
        ("search", Some(sub_m)) => {
            let query = sub_m.value_of("query").unwrap().to_lowercase();
            let filtered_tasks: Vec<&Task> = tasks
                .iter()
                .filter(|task| {
                    task.title.to_lowercase().contains(&query)
                        || task.description.to_lowercase().contains(&query)
                })
                .collect();

            for (index, task) in filtered_tasks.iter().enumerate() {
                println!("Task {}: {:#?}", index + 1, task);
            }
        }

        ("update", Some(sub_m)) => {
            if let Err(err) = update_task(sub_m, &mut tasks) {
                println!("Error: {}", err);
            } else {
                println!("Task updated successfully!");
            }
        }
        _ => println!("Invalid command"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_save_and_load_tasks() {
        let tasks = vec![
            Task {
                title: String::from("Task 1"),
                description: String::from("Description 1"),
                priority: 1,
                status: String::from("Todo"),
                project: String::from("Project"),
            },
            Task {
                title: String::from("Task 2"),
                description: String::from("Description 2"),
                priority: 2,
                status: String::from("In Progress"),
                project: String::from("Project"),
            },
        ];

        // Save tasks
        save_tasks(&tasks).unwrap();

        // Load tasks
        let loaded_tasks = load_tasks().unwrap();

        // Check if loaded tasks match the original tasks
        assert_eq!(tasks, loaded_tasks);

        // Clean up: delete the test file
        fs::remove_file("tasks.json").unwrap();
    }

    #[test]
    fn test_update_task() {
        let mut tasks = vec![
            Task {
                title: String::from("Task 1"),
                description: String::from("Description 1"),
                priority: 1,
                status: String::from("Todo"),
                project: String::from("Project"),
            },
            Task {
                title: String::from("Task 2"),
                description: String::from("Description 2"),
                priority: 2,
                status: String::from("In Progress"),
                project: String::from("Project"),
            },
        ];

        // Create ArgMatches for the update command
        let update_matches = App::new("Test Update Command")
            .subcommand(
                SubCommand::with_name("update")
                    .arg(
                        Arg::with_name("title")
                            .index(1)
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("description")
                            .long("description")
                            .takes_value(true),
                    )
                    .arg(Arg::with_name("project").long("project").takes_value(true)),
            )
            .get_matches_from(vec![
                "",
                "update",
                "Task 1",
                "--description",
                "Updated Description",
            ]);

        // Perform the update
        update_task(
            &update_matches.subcommand_matches("update").unwrap(),
            &mut tasks,
        )
        .unwrap();

        // Check if the task was updated successfully
        let updated_task = tasks.iter().find(|t| t.title == "Task 1").unwrap();
        assert_eq!(updated_task.description, "Updated Description");
    }

    #[test]
    fn test_add_task() {
        let mut tasks = vec![Task {
            title: String::from("Task 1"),
            description: String::from("Description 1"),
            priority: 1,
            status: String::from("Todo"),
            project: String::from("Project"),
        }];

        // Create ArgMatches for the add command
        let add_matches = App::new("Test Add Command")
            .subcommand(
                SubCommand::with_name("add")
                    .arg(
                        Arg::with_name("title")
                            .index(1)
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("description")
                            .index(2)
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("priority")
                            .index(3)
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("status")
                            .index(4)
                            .required(true)
                            .takes_value(true),
                    )
                    .arg(
                        Arg::with_name("project")
                            .index(5)
                            .required(true)
                            .takes_value(true),
                    ),
            )
            .get_matches_from(vec![
                "",
                "add",
                "New Task",
                "New Description",
                "3",
                "In Progress",
                "Project",
            ]);

        // Perform the add
        match add_matches.subcommand() {
            ("add", Some(sub_m)) => {
                let title = sub_m.value_of("title").unwrap();
                let description = sub_m.value_of("description").unwrap();
                let priority = sub_m.value_of("priority").unwrap().parse::<u8>().unwrap();
                let status = sub_m.value_of("status").unwrap();
                let project = sub_m.value_of("project").unwrap();

                let new_task = Task {
                    title: title.to_string(),
                    description: description.to_string(),
                    priority,
                    status: status.to_string(),
                    project: project.to_string(),
                };

                tasks.push(new_task);
                save_tasks(&tasks).unwrap();
            }
            _ => unreachable!(),
        }

        // Check if the task was added successfully
        let added_task = tasks.iter().find(|t| t.title == "New Task").unwrap();
        assert_eq!(added_task.description, "New Description");
        assert_eq!(added_task.priority, 3);
        assert_eq!(added_task.status, "In Progress");
        assert_eq!(added_task.project, "Project");
    }

    #[test]
    fn test_remove_task() {
        let mut tasks = vec![
            Task {
                title: String::from("Task 1"),
                description: String::from("Description 1"),
                priority: 1,
                status: String::from("Todo"),
                project: String::from("Project"),
            },
            Task {
                title: String::from("Task 2"),
                description: String::from("Description 2"),
                priority: 2,
                status: String::from("In Progress"),
                project: String::from("Project"),
            },
        ];

        // Create ArgMatches for the remove command
        let remove_matches = App::new("Test Remove Command")
            .subcommand(
                SubCommand::with_name("remove").arg(
                    Arg::with_name("title")
                        .index(1)
                        .required(true)
                        .takes_value(true),
                ),
            )
            .get_matches_from(vec!["", "remove", "Task 1"]);

        // Perform the remove
        match remove_matches.subcommand() {
            ("remove", Some(sub_m)) => {
                let title = sub_m.value_of("title").unwrap();
                tasks.retain(|task| task.title != title);
                save_tasks(&tasks).unwrap();
            }
            _ => unreachable!(),
        }

        // Check if the task was removed successfully
        assert!(!tasks.iter().any(|t| t.title == "Task 1"));
    }
}
