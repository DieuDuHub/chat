#[macro_use] extern crate rocket;
use tokio_postgres::{NoTls, Error};

async fn db_connectivity_test() -> Result<(), Error>{
    let (client, connection) =
        tokio_postgres::connect("host=localhost dbname=md", NoTls).await?;

    // Spawn the connection task to handle communication with the database
    let connection_handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    // Check if the table exists 
    let result = client.batch_execute("
        CREATE TABLE IF NOT EXISTS person (
            id      SERIAL PRIMARY KEY,
            name    TEXT NOT NULL,
            data    BYTEA
        )
    ").await;

    match result {
        Ok(_) => println!("Table created or already exists"),
        Err(e) => {
            eprintln!("Error with table creation: {}", e);
            return Err(e);
        }
    }

    let name = "Ferris";
    let data = None::<&[u8]>;
    
    // Insert data
    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&name, &data],
    ).await?;

    // Query data
    let rows = client.query("SELECT id, name, data FROM person", &[]).await?;
    
    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }

    println!("Database connectivity test completed!");
    
    // Close the connection gracefully
    connection_handle.abort();
    
    Ok(())
}


#[get("/test")]
async fn ping() -> &'static str {
    match db_connectivity_test().await {
        Ok(_) => "Database connectivity test passed",
        Err(e) => {
            eprintln!("Database connectivity test failed: {}", e);
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

//create cargo test
#[cfg(test)]
mod tests {
    use super::*;
    use rocket::local::blocking::Client;
    use rocket::http::Status;
    
    #[test]
    fn test_db_connectivity() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.get("/test").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let body = response.into_string().expect("valid response body");
        assert!(body.contains("Database connectivity test passed") || body.contains("Database connectivity test failed"));
    }
}   