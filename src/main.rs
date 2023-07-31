use std::{
    io,
    io::{BufRead, Read, Error, Write},
    fs::{OpenOptions, write}
};

#[derive(Debug)]
struct Todo {
    task: String,
    done: bool
}

impl Todo {
    fn create(task: String, done: bool) -> Todo {
        Todo { task, done }
    }
    #[warn(unused_must_use)]
    fn save(&self) -> Result<(), Error> {   
        let task = format!("{}{}{}", self.task, ":", self.done);
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("todos.txt")
            .expect("unable to open todos.txt");
        writeln!(file, "{}", task).expect("something went wrong");

        Ok(())
    }

}

fn main() {
    let mut todo: Vec<Todo> = all_todos().expect("Unable to read task list");
    println!("Task {:?}", todo);
    loop {
        let stdin = io::stdin();

        println!("Enter an action: ");
        let action = stdin.lock().lines().next().unwrap().unwrap();

        if action == "break" {
            break;
        }

        println!("Enter a new task: ");
        let task = stdin.lock().lines().next().unwrap().unwrap();

        match action.as_ref() {
            "show" => show_todos(&mut todo, task),
            "create" => create_todo(&mut todo, task),
            "complete" => complete_todo(&mut todo, task),
            "delete" => delete_todo(&mut todo, task),
            _ => println!("invalid action")
        }
    }
}

fn all_todos() -> Result<Vec<Todo>, Error> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("todos.txt")
        .expect("Unable to open file");
    let mut body = String::new();
    file.read_to_string(&mut body).expect("Unable to read file");
    let mut list: Vec<Todo> = Vec::new();

    for line in body.lines() {
        // task:false
        let task = line.split(':').collect::<Vec<&str>>();
        list.push(
            Todo::create(task[0].to_string(), task[1].parse().unwrap())
        )
    }
    Ok(list)

}

fn show_todos(todo: &mut Vec<Todo>, status_task: String) {
    println!("Task to complet:\n");
    
    for task in todo {
        let status = if !task.done { "Not done" } else { "Done" };

        if &status_task == "all" {
            println!("{} - {}", task.task, status);
        } else if status_task == "completed" && task.done {
            println!("{} - {}", task.task, status);
        } else if status_task == "not" &&!task.done {
            println!("{} - {}", task.task, status);
        }
    }
}

fn create_todo(todo:&mut Vec<Todo>, task : String) {
    let todo_instance = Todo::create(task, false);
    match todo_instance.save() {
        Ok(_) => {
            todo.push(todo_instance);
            println!("Task saved")
        }
        Err(_) => println!("Task not saved")
    }
    
}

fn complete_todo(todo: &mut Vec<Todo>, arg_task: String) {
    let mut body = String::new();

    for task in todo {
        let task_done = if task.task == arg_task { true } else { task.done };
        let current_task = format!(
            "{}:{}\n", 
            task.task,
            task_done
        );

        task.done = task_done;
        body.push_str(&current_task)
    }

    match write("todos.txt", body) {
        Ok(_) => println!("Task completed"),
        Err(error) => println!("Task not completed: {}", error)
    }

}

fn delete_todo(todo: &mut Vec<Todo>, arg_task: String) {
    let mut body = String::new();

    for task in &mut todo.into_iter() {
        let current_task = format!(
            "{}:{}\n", 
            task.task,
            task.done
        );
        if task.task != arg_task {
            body.push_str(&current_task);
        }
    }

    todo.retain(|value| value.task!= arg_task);

    match write("todos.txt", body) {
        Ok(_) => println!("Task deleted"),
        Err(error) => println!("Task not deleted: {}", error)
    }
}