// use axum::{extract::Path, routing::get, Router};
// use serde::Serialize;

// #[derive(Serialize)]
// struct Todo {
//     id: u32,
//     title: String,
//     done: bool,
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new()
//         .route("/hello/{name}", get(hello_name))
//         .route("/add/{a}/{b}", get(add));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     println!("Listening on http://127.0.0.1:3000");
//     axum::serve(listener, app).await.unwrap();
// }

// // async fn hello() -> &'static str {
// //     "Hello, world!"
// // }

// // async fn example_todo() -> Json<Todo> {
// //     let todo = Todo {
// //         id: 1,
// //         title: String::from("Learn axum"),
// //         done: false,
// //     };
// //     Json(todo)
// // }

// async fn hello_name(Path(name): Path<String>) -> String {
//     format!("Hello, {name}!")
// }

// async fn add(Path((a, b)): Path<(i32, i32)>) -> String {
//     format!("{a} + {b} = {}", a + b)
// }

// ----------------------------------------------------

// use axum::{routing::post, Json, Router};
// use serde::{Deserialize, Serialize};

// #[derive(Serialize)]
// struct Todo {
//     id: u32,
//     title: String,
//     done: bool,
// }

// #[derive(Deserialize)]
// struct NewTodo {
//     title:String,
// }

// #[tokio::main]
// async fn main() {
//     let app = Router::new().route("/echo",post(echo_todo));

//     let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
//     println!("Listening on http://127.0.0.1:3000");
//     axum::serve(listener, app).await.unwrap();
// }

// async fn echo_todo(Json(input): Json<NewTodo>) ->Json<Todo> {
//     let todo = Todo {
//         id: 1,
//         title: input.title,
//         done: false,
//     };
//     Json(todo)
// }

// --------------------------------------------------


use std::{
    fs,
    io::ErrorKind,
    sync::{Arc, Mutex},
};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
struct Todo {
    id: u32,
    title: String,
    done: bool,
}

#[derive(Deserialize)]
struct CreateTodoInput {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodoInput {
    title: String,
    done: bool,
}

type Db = Arc<Mutex<Vec<Todo>>>;

fn load_todos() -> Result<Vec<Todo>, Box<dyn std::error::Error>> {
    match fs::read_to_string("todos.json") {
        Ok(contents) => {
            let todos: Vec<Todo> = serde_json::from_str(&contents)?;
            Ok(todos)
        }
        Err(error) if error.kind() == ErrorKind::NotFound => {
            fs::write("todos.json", "[]")?;
            Ok(Vec::new())
        }
        Err(error) => Err(error.into()),
    }
}

fn save_todos(todos: &[Todo]) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_string(todos)?;
    fs::write("todos.json", json)?;
    Ok(())
}

#[tokio::main]
async fn main() {
    let todos = load_todos().expect("Failed to load todos.json");

    let db: Db = Arc::new(Mutex::new(todos));

    let app = Router::new()
        .route("/todos", get(list_todos).post(create_todo))
        .route("/todos/{id}", get(get_todo).delete(delete_todo).put(update_todo))
        .with_state(db);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();

    println!("Listening on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn list_todos(State(db): State<Db>) -> Json<Vec<Todo>> {
    let todos = db.lock().unwrap();
    Json(todos.clone())
}

async fn get_todo(State(db): State<Db>, Path(id): Path<u32>) -> Result<Json<Todo>, StatusCode> {
    let todos = db.lock().unwrap();

    match todos.iter().find(|todo| todo.id == id) {
        Some(todo) => Ok(Json(todo.clone())),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn create_todo(State(db): State<Db>, Json(input): Json<CreateTodoInput>,) -> Result<(StatusCode, Json<Todo>), StatusCode> {
    let mut todos = db.lock().unwrap();

    let id = todos.iter().map(|todo| todo.id).max().unwrap_or(0) + 1;

    let todo = Todo {
        id,
        title: input.title,
        done: false,
    };

    todos.push(todo.clone());

    save_todos(&todos)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok((StatusCode::CREATED, Json(todo)))
}

async fn update_todo(State(db): State<Db>, Path(id): Path<u32>, Json(input): Json<UpdateTodoInput>) -> Result<Json<Todo>, StatusCode> {
    let mut todos = db.lock().unwrap();

    let todo = match todos.iter_mut().find(|todo| todo.id == id) {
        Some(todo) => todo,
        None => return Err(StatusCode::NOT_FOUND),
    };

    todo.title = input.title;
    todo.done = input.done;

    let updated_todo = todo.clone();

    save_todos(&todos)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(updated_todo))
}

async fn delete_todo(State(db): State<Db>, Path(id): Path<u32>) -> StatusCode {
    let mut todos = db.lock().unwrap();

    let before = todos.len();

    todos.retain(|todo| todo.id != id);

    if todos.len() == before {
        return StatusCode::NOT_FOUND;
    }

    match save_todos(&todos) {
        Ok(()) => StatusCode::NO_CONTENT,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}