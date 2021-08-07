#![feature(async_closure)]

mod schema;

use futures::join;
use schema::{load_schema, Table};
use std::time::Duration;
use tokio::time::sleep;

const FREQUENCY: Duration = Duration::from_millis(10_000);

async fn enforce_all(tables: Vec<Table>) {
    unimplemented!();
}

async fn loop_enforce_all() {
    loop {
        enforce_all(load_schema("./schema")).await
    }

    sleep(FREQUENCY).await;
}

#[tokio::main]
async fn main() {
    join!(loop_enforce_all());

    println!("Hello, async");
}
