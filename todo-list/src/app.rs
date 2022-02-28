use std::env; 
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::prelude::*;

static TODO_FILE_PATH: &str = "data/data.json";

struct TodoItem {
  is_checked: bool, 
  text: String
}

struct TodoList {
  todos: Vec<TodoItem>
}

impl TodoList {
  fn new(todos: Vec<TodoItem>) -> TodoList {
    return TodoList {
      todos: todos
    };
  }
  fn _todolist_from_file(path_name: &str) -> TodoList {
    match File::open(path_name) {
      Ok(file) => {
        let todo_list = TodoList::new(vec![]);
        let data = BufReader::new(file);
        for line in data.lines() {
          println!("{}", line.unwrap());
        }
        return todo_list; 
      }, 
      Err(err) => {
        println!("Error when get todo list from path: {} ", err);
        return TodoList::new(vec![]);
      }
    }
  }
}

pub fn main() {
  println!("{}", std::env::current_dir().unwrap().display());
  let todo_list = TodoList::_todolist_from_file(TODO_FILE_PATH);
  println!("Length of list {}", todo_list.todos.len())
}