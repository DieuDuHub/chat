use tokio_postgres::{NoTls, Error as PgError, Client, Connection, Socket};
use std::env;
use std::fs;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;
use chrono::{DateTime, Utc, NaiveDate};
// Supprimer rust_decimal car pas supporté directement
// use rust_decimal::Decimal;

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
    pub create_table_person_data: String,
    pub insert_person: String,
    pub insert_person_data: String,
    pub select_all_persons: String,
    pub select_all_person_data: String,
    pub select_person_data_by_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PersonData {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub email: String,
    pub phone: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub gender: Option<String>,
    pub street_address: Option<String>,
    pub city: Option<String>,
    pub state_province: Option<String>,
    pub postal_code: Option<String>,
    pub country: Option<String>,
    pub nationality: Option<String>,
    pub occupation: Option<String>,
    pub company: Option<String>,
    pub salary: Option<f64>, // Utiliser f64 au lieu de Decimal
    pub marital_status: Option<String>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub is_active: Option<bool>,
}

#[derive(Deserialize)]
pub struct Data {
    pub default_name: String,
}

pub fn load_sql_config() -> Result<SqlConfig, Box<dyn std::error::Error>> {
    let config_content = fs::read_to_string("conf/pgbo_sql.toml")?;
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

// Method to read all person data from the person_data table
pub async fn read_all_person_data() -> Result<Vec<PersonData>, Box<dyn std::error::Error>> {
    let client = create_client().await?;
    let config = load_sql_config()?;
    
    read_all_person_data_with_client(&client, &config).await
}

// Method to read all person data with an existing client
pub async fn read_all_person_data_with_client(client: &Client, config: &SqlConfig) -> Result<Vec<PersonData>, Box<dyn std::error::Error>> {
    let rows = client.query(&config.queries.select_all_person_data, &[]).await?;
    
    let mut persons = Vec::new();
    
    for row in rows {
        let person = PersonData {
            id: row.get("id"),
            first_name: row.get("first_name"),
            last_name: row.get("last_name"),
            email: row.get("email"),
            phone: row.get("phone"),
            birth_date: row.get("birth_date"),
            gender: row.get("gender"),
            street_address: row.get("street_address"),
            city: row.get("city"),
            state_province: row.get("state_province"),
            postal_code: row.get("postal_code"),
            country: row.get("country"),
            nationality: row.get("nationality"),
            occupation: row.get("occupation"),
            company: row.get("company"),
            salary: row.get("salary"),
            marital_status: row.get("marital_status"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
            is_active: row.get("is_active"),
        };
        persons.push(person);
    }
    
    Ok(persons)
}

// Method to read a single person by ID
pub async fn read_person_data_by_id(id: i32) -> Result<Option<PersonData>, Box<dyn std::error::Error>> {
    let client = create_client().await?;
    let config = load_sql_config()?;
    
    read_person_data_by_id_with_client(&client, &config, id).await
}

// Method to read a single person by ID with an existing client
pub async fn read_person_data_by_id_with_client(client: &Client, config: &SqlConfig, id: i32) -> Result<Option<PersonData>, Box<dyn std::error::Error>> {
    let rows = client.query(&config.queries.select_person_data_by_id, &[&id]).await?;
    
    if rows.is_empty() {
        return Ok(None);
    }
    
    let row = &rows[0];
    let person = PersonData {
        id: row.get("id"),
        first_name: row.get("first_name"),
        last_name: row.get("last_name"),
        email: row.get("email"),
        phone: row.get("phone"),
        birth_date: row.get("birth_date"),
        gender: row.get("gender"),
        street_address: row.get("street_address"),
        city: row.get("city"),
        state_province: row.get("state_province"),
        postal_code: row.get("postal_code"),
        country: row.get("country"),
        nationality: row.get("nationality"),
        occupation: row.get("occupation"),
        company: row.get("company"),
        salary: row.get("salary"),
        marital_status: row.get("marital_status"),
        created_at: row.get("created_at"),
        updated_at: row.get("updated_at"),
        is_active: row.get("is_active"),
    };
    
    Ok(Some(person))
}

// Legacy function for backward compatibility and tests
pub async fn db_connectivity_test() -> Result<(), Box<dyn std::error::Error>> {
    db_connectivity_test_with_new_client().await
}
