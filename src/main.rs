use std::cell::RefCell;
use std::env;
use std::rc::Rc;
use todo::{TodoItem, *};

mod todo;

slint::include_modules!();

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let todos: Vec<TodoItem> = load_todos(); //加载数据
    let todos = Rc::new(RefCell::new(todos));

    let mut todos_slint: Vec<TodoItemSlint> = Vec::new();
    for todo in todos.borrow().iter() {
        let todo_s = TodoItemSlint {
            id: todo.id.to_string().into(),
            content: todo.content.clone().into(),
            done: todo.done,
        };
        todos_slint.push(todo_s);
    } //构架slint数据

    let ui = MyApp::new()?;
    let models = std::rc::Rc::new(slint::VecModel::from(todos_slint));
    ui.set_todos(models.clone().into());
    // let ui_weak = ui.as_weak();

    ui.on_add_todo({
        let todos = todos.clone();
        let models = models.clone();
        move |content| {
            let todo = TodoItem {
                id: uuid::Uuid::new_v4(),
                content: content.into(),
                done: false,
            };
            let todo_s = todo.clone();

            // let todos = todos_add.clone();
            todos.borrow_mut().insert(0, todo);
            save_todos(todos.borrow().clone());

            models.insert(
                0,
                TodoItemSlint {
                    id: todo_s.id.to_string().into(),
                    content: todo_s.content.into(),
                    done: todo_s.done,
                },
            );
        }
    });

    ui.on_del_todo({
        let todos = todos.clone();
        let models = models.clone();
        move |id| {
            let id = uuid::Uuid::parse_str(&id).unwrap();
            // let todos = todos_del.clone();
            let index = todos.borrow().iter().position(|x| x.id == id).unwrap();
            todos.borrow_mut().remove(index);
            save_todos(todos.borrow().clone());
            models.remove(index);
        }
    });

    ui.run()?;
    Ok(())
}
