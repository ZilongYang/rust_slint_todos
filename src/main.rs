use std::borrow::Borrow;
use std::env;
use std::rc::Rc;
use std::sync::{Mutex, RwLock};
use std::{cell::RefCell, sync::Arc};
use todo::{TodoItem, TodoList};

mod todo;

impl TodoList  {
    fn to_slint(&self) -> Vec<TodoItemSlint> {
        let mut todos_slint: Vec<TodoItemSlint> = Vec::new();
        for todo in self {
            let todo_s = TodoItemSlint {
                id: todo.id.to_string().into(),
                content: todo.content.clone().into(),
                done: todo.done,
            };
            todos_slint.push(todo_s);
        }
        todos_slint
    }
}

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let todos = TodoList::load_from_yaml_file(); //加载数据
    // let todos = Rc::new(RefCell::new(todos));

    let todos_slint = todos.to_slint();
    // let mut todos_slint: Vec<TodoItemSlint> = Vec::new();
    // for todo in todos.clone() {
    //     let todo_s = TodoItemSlint {
    //         id: todo.id.to_string().into(),
    //         content: todo.content.clone().into(),
    //         done: todo.done,
    //     };
    //     todos_slint.push(todo_s);
    // } //构架slint数据

    let ui = MyApp::new()?;
    let models = std::rc::Rc::new(slint::VecModel::from(todos_slint));
    ui.set_todos(models.clone().into());
    // let ui_weak = ui.as_weak();
    let todos = Arc::new(RwLock::new(todos));
    let models = Arc::new(RwLock::new(models));

    ui.global::<Logic>().on_add_todo({
        let todos = Arc::clone(&todos);
        let models = Arc::clone(&models);
        move |content| {
            // let todo = TodoItem {
            //     id: uuid::Uuid::new_v4(),
            //     content: content.clone().into(),
            //     done: false,
            // };
            // let todo_s = todo.clone();

            // let todos = todos_add.clone();
            let mut todos = todos.write().unwrap();
            let id = todos.add_from_content(content.clone().into());
            let models = models.write().unwrap();
            // save_todos(todos.clone());
            todos.save_to_ymal_file();

            models.insert(
                0,
                TodoItemSlint {
                    id: id.to_string().into(),
                    content: content.clone(),
                    done: false,
                },
            );
        }
    });

    ui.global::<Logic>().on_del_todo({
        let todos = Arc::clone(&todos);
        let models = Arc::clone(&models);
        move |id| {
            let id = uuid::Uuid::parse_str(&id).unwrap_or_else(|err| {
                eprintln!("Failed to parse UUID: {}", err);
                uuid::Uuid::nil()
            });
            // let todos = todos_del.clone();
            let mut todos = todos.write().unwrap();
            let models = models.write().unwrap();
            match todos.index_of(id) {
                Some(index) => {
                    todos.del_by_id(id);
                    models.remove(index);
                    // save_todos(todos.borrow().clone());
                    todos.save_to_ymal_file();
                }
                None => {
                    eprintln!("Failed to find todo with ID: {}", id);
                }
            }
        }
    });

    ui.global::<Logic>().on_set_todo_done({
        let todos: Arc<RwLock<TodoList>> = Arc::clone(&todos);
        let models = Arc::clone(&models);
        move |id, done| {
            let id = uuid::Uuid::parse_str(&id).unwrap_or_else(|err| {
                eprintln!("Failed to parse UUID: {}", err);
                uuid::Uuid::nil()
            });
            // let todos = todos_toggle.clone();
            let mut todos = todos.write().unwrap();
            let models = models.write().unwrap();
            match todos.index_of(id) {
                Some(index) => {
                    let todo = &mut todos.todos[index];
                    todo.done = done;
                    models.insert(index, TodoItemSlint {
                        id: id.to_string().into(),
                        content: todo.content.clone().into(),
                        done: todo.done,
                    });
                    models.remove(index + 1);
                    todos.save_to_ymal_file();
                }
                None => {
                    eprintln!("Failed to find todo with ID: {}", id);
                }
            }
        }
    });

    ui.global::<Logic>().on_set_todo_content({
        let todos = Arc::clone(&todos);
        let models = Arc::clone(&models);
        move |id, content| {
            let id = uuid::Uuid::parse_str(&id).unwrap_or_else(|err| {
                eprintln!("Failed to parse UUID: {}", err);
                uuid::Uuid::nil()
            });
            let mut todos = todos.write().unwrap();
            let models = models.write().unwrap();
            match todos.index_of(id) {
                Some(index) => {
                    let todo = &mut todos.todos[index];
                    todo.content = content.clone().into();
                    models.insert(index, TodoItemSlint {
                        id: id.to_string().into(),
                        content: todo.content.clone().into(),
                        done: todo.done,
                    });
                    models.remove(index + 1);
                    todos.save_to_ymal_file();
                }
                None => {
                    eprintln!("Failed to find todo with ID: {}", id);
                }
            }
        }
    });

    ui.run()?;
    Ok(())
}
