#[macro_use] extern crate rocket;

mod database;

#[cfg(test)]
mod tests;

use database::db_connectivity_test;

#[get("/test")]
async fn ping() -> &'static str {
    match db_connectivity_test().await {
        Ok(_) => {
            println!("Database connectivity test passed successfully");
            "Database connectivity test passed"
        },
        Err(e) => {
            eprintln!("Database connectivity test failed: {}", e);
            // Log specific error types for debugging
            if let Some(pg_error) = e.downcast_ref::<tokio_postgres::Error>() {
                eprintln!("PostgreSQL error: {}", pg_error);
            } else if let Some(toml_error) = e.downcast_ref::<toml::de::Error>() {
                eprintln!("TOML parsing error: {}", toml_error);
            } else if let Some(io_error) = e.downcast_ref::<std::io::Error>() {
                eprintln!("IO error: {}", io_error);
            }
            "Database connectivity test failed"
        }
    }
}

#[get("/ping")]
async fn index() -> &'static str {
    "alive"
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index,ping])
}   