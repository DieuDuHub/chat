use tokio_postgres::{NoTls, Error as PgError};
use std::env;
use std::fs;
use serde::Deserialize;

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

pub async fn db_connectivity_test() -> Result<(), Box<dyn std::error::Error>> {
    let config = load_sql_config()?;
    
    let db_url = env::var("PGBO_DB")
        .unwrap_or_else(|_| "host=localhost dbname=md".to_string());
    
    let (client, connection) =
        tokio_postgres::connect(&db_url, NoTls).await?;

    // Spawn the connection task to handle communication with the database
    let connection_handle = tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

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
    
    // Close the connection gracefully
    connection_handle.abort();
    
    Ok(())
}
