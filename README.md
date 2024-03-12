[![Review Assignment Due Date](https://classroom.github.com/assets/deadline-readme-button-24ddc0f5d75046c5622901739e7c5dd533143b0c8e959d652212380cedb1ea36.svg)](https://classroom.github.com/a/RQfdh2iK)
# Task Tracker Application

## Description

A console-based task management application. Users can add, remove, list, and update tasks. Users can also search for tasks by title or description. Each task will have a title, description, priority, status and category. The application will be implemented in such a way to persist tasks so that they are not lost when the program is closed and reopened and will provide error handling for invalid user inputs. This program will make use of the clap and serde third party crates. 

## Installation

Build the project using ` cargo build ` . You can run tests with ` cargo test `. To run a single test, use ` cargo test <testname> `.

## How to use

After compiling, installing, and setting up the project, users can interact with it via the command line. Here are some examples:

### Adding a Task

` cargo run  -- add "Task Name" "Task Description" 1 "Status" "Category" ` In this example, the number "1" correspondes to the priority number of the task.

### Removing a Task

` cargo run -- remove "Task Name" `

### Listing All Tasks

` cargo run -- list `

#### Listing Tasks by Category

` cargo run -- list-by-project --project "Work" `

#### Listing Tasks by Status

` cargo run -- list-by-status --status "In Progress" `

#### Listing Tasks by Priority

` cargo run -- list-by-priority --priority 2 `

### Searching for a Task

` cargo run -- search "Task Name" ``` or ```cargo run -- search "Task Description" `

### Updating a Task

To update an existing task you can run ` cargo run -- update "Task Name" --description "Updated Description" --priority 2 --status "Updated Status" --project "Updated project name" `
Additionally, you can also update just one field: ` cargo run -- update "Task Name" --project "Updated project name" `

