use once_cell::sync::OnceCell;
use sqlx::{Executor, Pool, Sqlite, SqlitePool};
use tokio::time;

static CONNS : OnceCell<Pool<Sqlite>> = OnceCell::new();

#[tokio::main]
async fn main() {
    println!("Hello, world!");
    let poll = init().await;
    CONNS.set(poll).expect("Create sqlite error!");
    loop {
        let _ = time::sleep(time::Duration::from_micros(1000));
    }
}

async fn init() -> Pool<Sqlite> {
    SqlitePool::connect("egccri-storage.sqlite").await.unwrap()
}
