#[macro_use] extern crate rocket;
use rocket::serde::json::Json;

mod database;

#[cfg(test)]
mod tests;

use database::{db_connectivity_test_with_new_client, get_db_url, read_all_person_data, read_person_data_by_id, PersonData};

#[get("/test")]
async fn ping() -> &'static str {
    match db_connectivity_test_with_new_client().await {
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

#[get("/person_data")]
async fn get_all_person_data() -> Json<Result<Vec<PersonData>, String>> {
    match read_all_person_data().await {
        Ok(persons) => Json(Ok(persons)),
        Err(e) => {
            eprintln!("Failed to read person data: {}", e);
            Json(Err(format!("Failed to read person data: {}", e)))
        }
    }
}

#[get("/person_data/<id>")]
async fn get_person_data_by_id(id: i32) -> Json<Result<Option<PersonData>, String>> {
    match read_person_data_by_id(id).await {
        Ok(person) => Json(Ok(person)),
        Err(e) => {
            eprintln!("Failed to read person data for ID {}: {}", id, e);
            Json(Err(format!("Failed to read person data: {}", e)))
        }
    }
}

#[launch]
async fn rocket() -> _ {
    // Test database connectivity at startup
    let db_url = get_db_url();
    println!("🔗 Testing database connection: {}", db_url);
    
    match tokio_postgres::connect(&db_url, tokio_postgres::NoTls).await {
        Ok((_, connection)) => {
            println!("✅ Database connection test successful");
            // Spawn the connection task to handle communication
            let _connection_handle = tokio::spawn(async move {
                if let Err(e) = connection.await {
                    eprintln!("Connection error: {}", e);
                }
            });
        },
        Err(e) => {
            eprintln!("❌ Failed to connect to database: {}", e);
            panic!("Cannot start server without database connectivity");
        }
    };

    rocket::build()
        .mount("/", routes![index, ping, get_all_person_data, get_person_data_by_id])
}

// Helper function for tests
#[cfg(test)]
pub fn rocket_for_tests() -> rocket::Rocket<rocket::Build> {
    use database::db_connectivity_test;
    
    // For tests, use the legacy endpoint that doesn't require state
    #[get("/test")]
    async fn test_ping() -> &'static str {
        match db_connectivity_test().await {
            Ok(_) => {
                println!("Database connectivity test passed successfully");
                "Database connectivity test passed"
            },
            Err(e) => {
                eprintln!("Database connectivity test failed: {}", e);
                "Database connectivity test failed"
            }
        }
    }

    rocket::build()
        .mount("/", routes![index, test_ping, get_all_person_data, get_person_data_by_id])
}   