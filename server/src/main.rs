mod routes;
mod handlers;
mod models;
mod db;

use db::{insert_projects, select_projects};
use routes::router;
//use rusqlite::{Connection, Result};
use sqlite::{Connection, OpenFlags, Result};
use tokio::task;


#[tokio::main]
async fn main() {
    
    let task_handle = task::spawn(async {
        if let Err(err) = insert_projects().await {
            eprintln!("Error creating database: {}", err);
        }
    }); 
    task_handle.await; //Ожидаем завершения асинхронной задачи по созданию базы данных

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:3000").await {
        Ok(listener) => listener,
        Err(err) => {
            eprintln!("Failed to bind to port 3000: {}", err);
            return;
        }
    };
    println!("Server is running at http://127.0.0.1:3000");

    let app = router();

    axum::serve(listener, app).await.unwrap();

    //let connection = sqlite::open(":memory:").unwrap();
    //let connection = Connection::open_in_memory().unwrap();
    // Создаем флаги для открытия базы данных
    //let flags = OpenFlags::new().with_create().with_read_write();

    // Создаем соединение с базой данных SQLite с указанными флагами
    //let connection = Connection::open_with_flags(":memory:", flags).unwrap();
    //insert_projects(&connection);
    //select_projects(&connection);

}
