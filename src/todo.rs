use serde::{Deserialize, Serialize};
use std::env;
use std::path::{Path, PathBuf};
use std::{fs, path};
use uuid::Uuid;

pub fn load_todos() -> Vec<TodoItem> {
    let todos = vec![
        TodoItem::new("Buy milk".to_string()),
        TodoItem::new("Feed the cat".to_string()),
        TodoItem::new("Prepare for the meeting".to_string()),
    ];

    // 从 YAML 文件加载配置
    // let mut yaml_content: String = String::new();
    // match fs::read_to_string("todos.yml") {
    //     Ok(str) => yaml_content = str,
    //     Err(_) => {
    //         println!("Error reading file");
    //     }
    // };

    // 获取当前程序的运行目录
    let current_dir = env::current_dir().unwrap();
    // 拼接出"todos.yml"的完整路径
    let file_path = Path::new(&current_dir).join("todos.yml");

    let yaml_content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file: {}", err);
        String::new()
    });
    let todos: Vec<TodoItem> = serde_yaml::from_str(&yaml_content).unwrap_or_else(|err| {
        eprintln!("Error parsing YAML: {}", err);
        vec![]
    });
    todos
}

pub fn save_todos(todos: Vec<TodoItem>) {
    let yaml_content = serde_yaml::to_string(&todos).unwrap_or_else(|err| {
        eprintln!("Error serializing YAML: {}", err);
        String::new()
    });

    let current_dir = env::current_dir().unwrap();
    let file_path = Path::new(&current_dir).join("todos.yml");

    fs::write(file_path, yaml_content).unwrap_or_else(|err| {
        eprintln!("Error writing file: {}", err);
    });
}

#[derive(Debug, Clone)]
pub struct TodoList {
    pub todos: Vec<TodoItem>,
}

impl TodoList {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }

    pub fn load_from_yaml_file() -> Self {
        let todos = load_todos();
        Self { todos }
    }

    pub fn save_to_ymal_file(&self) {
        save_todos(self.todos.clone());
    }

    pub fn add(&mut self, todo: TodoItem) {
        self.todos.push(todo);
    }

    pub fn add_from_content(&mut self, content: String) -> Uuid {
        let todo = TodoItem::new(content);
        let id = todo.id;
        self.todos.insert(0, todo);
        id
    }

    pub fn index_of(&self, id: Uuid) -> Option<usize> {
        self.todos.iter().position(|todo| todo.id == id)
    }

    pub fn del_by_id(&mut self, id: Uuid) {
        self.todos.retain(|todo| todo.id != id);
    }

    // 其他方法...
}

impl IntoIterator for TodoList {
    type Item = TodoItem;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.todos.into_iter()
    }
}

impl<'a> IntoIterator for &'a TodoList {
    type Item = &'a TodoItem;
    type IntoIter = std::slice::Iter<'a, TodoItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.todos.iter()
    }
}

impl<'a> IntoIterator for &'a mut TodoList {
    type Item = &'a mut TodoItem;
    type IntoIter = std::slice::IterMut<'a, TodoItem>;

    fn into_iter(self) -> Self::IntoIter {
        self.todos.iter_mut()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: Uuid,
    pub content: String,
    pub done: bool,
}

impl TodoItem {
    pub fn new(content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            content,
            done: false,
        }
    }

    pub fn toggle(&mut self) {
        self.done = !self.done;
    }

    pub fn update_content(&mut self, content: String) {
        self.content = content;
    }
}
