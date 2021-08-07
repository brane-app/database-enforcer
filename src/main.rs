#![feature(async_closure)]

mod schema;

use futures::join;
use mysql::prelude::Queryable;
use mysql::{Opts, Pool};
use schema::{load_schema, Table};
use std::{env::var, time::Duration};
use tokio::time::sleep;

const FREQUENCY: Duration = Duration::from_millis(10_000);
static SCHEMA_ROOT: &str = "./schema";

async fn enforce(pool: &Pool, table: &Table) -> mysql::Result<()> {
    let fields_join = table.field_schemas.join(", ");
    let statement = format!(
        "CREATE TABLE IF NOT EXISTS {name} ({fields})",
        name = table.name,
        fields = fields_join
    );

    pool.get_conn()?.query_drop(statement)
}

async fn loop_enforce_all(pool: &Pool) {
    loop {
        for table in load_schema(SCHEMA_ROOT).iter() {
            if let Err(why) = enforce(pool, table).await {
                println!("Failed to enforce {}: \n{}", table.name, why);
            };
        }

        sleep(FREQUENCY).await;
    }
}

#[tokio::main]
async fn main() {
    let address = var("DATABASE_CONNECTION").expect("DATABASE_CONNECTION missing!");
    let pool = Pool::new(Opts::from_url(&address).expect("DATABASE_CONNECTION isn't valid!"))
        .unwrap_or_else(|err| panic!("{}", err));

    join!(loop_enforce_all(&pool));

    println!("Hello, async");
}
