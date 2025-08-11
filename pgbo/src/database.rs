use tokio_postgres::{NoTls, Error as PgError, Client, Connection, Socket};
use std::env;
use std::fs;
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::Mutex;

// Database connection state for Rocket - shared connection
pub type DbConnection = Arc<Mutex<Option<tokio::task::JoinHandle<()>>>>;

#[derive(Deserialize)]
pub struct SqlConfig {
    pub queries: Queries,
    pub data: Data,
}

#[derive(Deserialize)]
pub struct Queries {
    pub create_table_person: String,
    pub insert_person: String,
    pub select_all_persons: String,
}

#[derive(Deserialize)]
pub struct Data {
    pub default_name: String,
}

pub fn load_sql_config() -> Result<SqlConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("pgbo_sql.toml")?;
    let config: SqlConfig = toml::from_str(&config_content)?;
    Ok(config)
}

pub fn get_db_url() -> String {
    env::var("PGBO_DB")
        .unwrap_or_else(|_| "host=localhost dbname=md".to_string())
}

pub async fn create_client() -> Result<Client, Box<dyn std::error::Error>> {
    let db_url = get_db_url();
    let (client, connection) = tokio_postgres::connect(&db_url, NoTls).await?;
    
    // Spawn the connection task to handle communication with the database
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    Ok(client)
}

pub async fn db_connectivity_test_with_new_client() -> Result<(), Box<dyn std::error::Error>> {
    let client = create_client().await?;
    let config = load_sql_config()?;

    // Check if the table exists 
    let result = client.batch_execute(&config.queries.create_table_person).await;

    match result {
        Ok(_) => println!("Table created or already exists"),
        Err(e) => {
            eprintln!("Error with table creation: {}", e);
            return Err(e.into());
        }
    }

    let name = &config.data.default_name;
    let data = None::<&[u8]>;
    
    // Insert data
    client.execute(
        &config.queries.insert_person,
        &[&name, &data],
    ).await?;

    // Query data
    let rows = client.query(&config.queries.select_all_persons, &[]).await?;
    
    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }

    println!("Database connectivity test completed!");
    
    Ok(())
}

pub async fn db_connectivity_test_with_client(client: &Client) -> Result<(), Box<dyn std::error::Error>> {
    let config = load_sql_config()?;

    // Check if the table exists 
    let result = client.batch_execute(&config.queries.create_table_person).await;

    match result {
        Ok(_) => println!("Table created or already exists"),
        Err(e) => {
            eprintln!("Error with table creation: {}", e);
            return Err(e.into());
        }
    }

    let name = &config.data.default_name;
    let data = None::<&[u8]>;
    
    // Insert data
    client.execute(
        &config.queries.insert_person,
        &[&name, &data],
    ).await?;

    // Query data
    let rows = client.query(&config.queries.select_all_persons, &[]).await?;
    
    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: Option<&[u8]> = row.get(2);

        println!("found person: {} {} {:?}", id, name, data);
    }

    println!("Database connectivity test completed!");
    
    Ok(())
}

// Legacy function for backward compatibility and tests
pub async fn db_connectivity_test() -> Result<(), Box<dyn std::error::Error>> {
    db_connectivity_test_with_new_client().await
}
