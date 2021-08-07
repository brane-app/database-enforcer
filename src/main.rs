#![feature(async_closure)]

mod schema;

use futures::join;
use schema::{load_schema, Table};
use std::time::Duration;
use tokio::time::sleep;

const FREQUENCY: Duration = Duration::from_millis(10_000);

async fn enforce_all(tables: Vec<Table>) {
    loop {
        for table in &tables {
            unimplemented!();
        }

        sleep(FREQUENCY).await;
    }
}

#[tokio::main]
async fn main() {
    join!(enforce_all(load_schema("./schema")));

    println!("Hello, async");
}
