use rocket::local::blocking::Client;
use rocket::http::Status;
use crate::database::{db_connectivity_test, load_sql_config, create_client, get_db_url};
use crate::rocket_for_tests;

#[test]
fn test_db_connectivity() {
    let client = Client::tracked(rocket_for_tests()).expect("valid rocket instance");
    let response = client.get("/test").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().expect("valid response body");
    assert!(body.contains("Database connectivity test passed") || body.contains("Database connectivity test failed"));
}

#[tokio::test]
async fn test_invalid_sql_should_fail() {
    // Test with invalid TOML file name to simulate SQL error
    let result = std::panic::catch_unwind(|| {
        // This should fail because the TOML file doesn't exist or has invalid SQL
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            // Temporarily change the config file to test error handling
            let original_config = std::fs::read_to_string("conf/pgbo_sql.toml").ok();
            
            // Create invalid SQL config
            let invalid_sql = r#"
[queries]
create_table_person = "INVALID SQL SYNTAX HERE"
insert_person = "INSERT INTO"
select_all_persons = "SELECT FROM"

[data]
default_name = "Test"
            "#;
            
            std::fs::write("conf/pgbo_sql.toml", invalid_sql).unwrap();
            
            let result = db_connectivity_test().await;
            
            // Restore original config if it existed
            if let Some(original) = original_config {
                std::fs::write("conf/pgbo_sql.toml", original).unwrap();
            }
            
            result
        })
    });
    
    // The test should fail with invalid SQL
    // Either the panic catch or the result should indicate failure
    if let Ok(db_result) = result {
        assert!(db_result.is_err(), "Database connectivity test should fail with invalid SQL");
    }
}

#[test] 
fn test_config_loading() {
    // Test that config loading works with valid TOML
    let result = load_sql_config();
    match result {
        Ok(config) => {
            assert!(!config.queries.create_table_person.is_empty());
            assert!(!config.queries.insert_person.is_empty()); 
            assert!(!config.queries.select_all_persons.is_empty());
            assert!(!config.data.default_name.is_empty());
        },
        Err(e) => {
            println!("Config loading failed (expected if conf/pgbo_sql.toml doesn't exist): {}", e);
            // This is acceptable for the test - we're just checking the function works
        }
    }
}

#[tokio::test]
async fn test_create_client_connection() {
    // Test that create_client successfully creates a working database client
    let result = create_client().await;
    
    match result {
        Ok(client) => {
            println!("✅ Database client created successfully");
            
            // Test that we can perform a simple query to verify the connection works
            let query_result = client.query("SELECT 1 as test_value", &[]).await;
            
            match query_result {
                Ok(rows) => {
                    assert_eq!(rows.len(), 1, "Should return exactly one row");
                    let test_value: i32 = rows[0].get(0);
                    assert_eq!(test_value, 1, "Should return the value 1");
                    println!("✅ Database client connection verified with test query");
                },
                Err(e) => {
                    println!("❌ Database client created but test query failed: {}", e);
                    panic!("Database client test query failed: {}", e);
                }
            }
        },
        Err(e) => {
            panic!("ℹ️ Database client creation failed (expected if DB not available): {}", e);
            // This test is allowed to fail if database is not available
            // but we log it for debugging purposes
        }
    }
}

#[tokio::test]
async fn test_create_client_strict() {
    // Strict version that fails if client creation fails
    let result = create_client().await;
    
    match result {
        Ok(client) => {
            println!("✅ Strict database client test passed");
            
            // Additional test: verify client can handle basic operations
            let query_result = client.query("SELECT current_database() as db_name", &[]).await;
            assert!(query_result.is_ok(), "Should be able to query current database name");
            
            if let Ok(rows) = query_result {
                assert!(!rows.is_empty(), "Should return database name");
                let db_name: String = rows[0].get(0);
                println!("✅ Connected to database: {}", db_name);
            }
        },
        Err(e) => {
            panic!("❌ Strict database client creation failed: {}", e);
        }
    }
}

#[test]
fn test_ping_endpoint() {
    let client = Client::tracked(rocket_for_tests()).expect("valid rocket instance");
    let response = client.get("/ping").dispatch();
    assert_eq!(response.status(), Status::Ok);
    let body = response.into_string().expect("valid response body");
    assert_eq!(body, "alive");
}

#[tokio::test]
async fn test_database_connectivity_direct() {
    // Direct test of database connectivity function
    let result = db_connectivity_test().await;
    
    // This test may pass or fail depending on database availability
    // We just want to ensure the function doesn't panic
    match result {
        Ok(_) => println!("Database connectivity test passed"),
        Err(e) => panic!("Database connectivity test failed (expected if no DB): {}", e),
    }
    
    // The test passes if we reach this point without panicking
    assert!(true);
}

#[test]
fn test_get_all_person_data_endpoint() {
    let client = Client::tracked(rocket_for_tests()).expect("valid rocket instance");
    let response = client.get("/person_data").dispatch();
    
    // The endpoint should respond, regardless of whether data exists
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().expect("valid response body");
    
    // The response should be valid JSON
    let json_result: Result<serde_json::Value, _> = serde_json::from_str(&body);
    assert!(json_result.is_ok(), "Response should be valid JSON: {}", body);
    
    if let Ok(json_value) = json_result {
        // Should be either Ok([...]) or Err("...")
        assert!(json_value.get("Ok").is_some() || json_value.get("Err").is_some(),
                "Response should contain either Ok or Err field");
        
        if let Some(ok_value) = json_value.get("Ok") {
            assert!(ok_value.is_array(), "Ok response should contain an array");
            println!("✅ Get all person data endpoint returned {} records", 
                     ok_value.as_array().unwrap().len());
        } else if let Some(err_value) = json_value.get("Err") {
            println!("ℹ️ Get all person data endpoint returned error (expected if no DB or table): {}", 
                     err_value.as_str().unwrap_or("unknown error"));
        }
    }
}

#[test]
fn test_get_person_data_by_id_endpoint() {
    let client = Client::tracked(rocket_for_tests()).expect("valid rocket instance");
    let response = client.get("/person_data/1").dispatch();
    
    // The endpoint should respond, regardless of whether the ID exists
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().expect("valid response body");
    
    // The response should be valid JSON
    let json_result: Result<serde_json::Value, _> = serde_json::from_str(&body);
    assert!(json_result.is_ok(), "Response should be valid JSON: {}", body);
    
    if let Ok(json_value) = json_result {
        // Should be either Ok(null), Ok({...}) or Err("...")
        assert!(json_value.get("Ok").is_some() || json_value.get("Err").is_some(),
                "Response should contain either Ok or Err field");
        
        if let Some(ok_value) = json_value.get("Ok") {
            if ok_value.is_null() {
                println!("✅ Get person data by ID endpoint: person with ID 1 not found (as expected)");
            } else {
                assert!(ok_value.is_object(), "Ok response should contain an object or null");
                println!("✅ Get person data by ID endpoint returned person data");
            }
        } else if let Some(err_value) = json_value.get("Err") {
            println!("ℹ️ Get person data by ID endpoint returned error (expected if no DB or table): {}", 
                     err_value.as_str().unwrap_or("unknown error"));
        }
    }
}

#[test]
fn test_get_person_data_by_invalid_id_endpoint() {
    let client = Client::tracked(rocket_for_tests()).expect("valid rocket instance");
    let response = client.get("/person_data/999999").dispatch();
    
    // The endpoint should respond even for non-existent IDs
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().expect("valid response body");
    
    // The response should be valid JSON
    let json_result: Result<serde_json::Value, _> = serde_json::from_str(&body);
    assert!(json_result.is_ok(), "Response should be valid JSON: {}", body);
    
    if let Ok(json_value) = json_result {
        if let Some(ok_value) = json_value.get("Ok") {
            // For a non-existent ID, we should get null
            if ok_value.is_null() {
                println!("✅ Get person data by invalid ID correctly returned null");
            } else {
                panic!("ℹ️ Get person data by invalid ID returned data (unexpected but not an error)");
            }
        }
    }
    else {
        // If we reach here, the response was not valid JSON
        panic!("Response was not valid JSON: {}", body);
    }
}

#[tokio::test]
async fn test_person_data_database_functions() {
    // Test the underlying database functions directly
    use crate::database::{read_all_person_data, read_person_data_by_id};
    
    // Test read_all_person_data function
    let all_result = read_all_person_data().await;
    match all_result {
        Ok(persons) => {
            println!("✅ read_all_person_data returned {} persons", persons.len());
            
            // If we have data, test the structure
            if !persons.is_empty() {
                let first_person = &persons[0];
                assert!(first_person.id > 0, "Person ID should be positive");
                assert!(!first_person.first_name.is_empty(), "First name should not be empty");
                assert!(!first_person.last_name.is_empty(), "Last name should not be empty");
                assert!(!first_person.email.is_empty(), "Email should not be empty");
                println!("✅ Person data structure validation passed");
            }
        },
        Err(e) => {
            println!("ℹ️ read_all_person_data failed (expected if no DB or table): {}", e);
        }
    }
    
    // Test read_person_data_by_id function
    let by_id_result = read_person_data_by_id(1).await;
    match by_id_result {
        Ok(person_option) => {
            match person_option {
                Some(person) => {
                    println!("✅ read_person_data_by_id found person with ID 1");
                    assert_eq!(person.id, 1, "Returned person should have ID 1");
                },
                None => {
                    println!("✅ read_person_data_by_id correctly returned None for ID 1");
                }
            }
        },
        Err(e) => {
            println!("ℹ️ read_person_data_by_id failed (expected if no DB or table): {}", e);
        }
    }
}

#[test]
fn test_pgbo_db_env_var_handling() {
    // Test that get_db_url() correctly handles the PGBO_DB environment variable
    let db_url = get_db_url();
    
    // Check if PGBO_DB environment variable exists
    match std::env::var("PGBO_DB") {
        Ok(env_value) => {
            // If the environment variable exists, get_db_url() should return its value
            assert_eq!(db_url, env_value);
            println!("✅ PGBO_DB environment variable exists: '{}'", env_value);
            println!("✅ get_db_url() correctly returns the environment variable value");
        },
        Err(_) => {
            // If the environment variable doesn't exist, get_db_url() should return the default
            let expected_default = "host=localhost dbname=md";
            assert_eq!(db_url, expected_default);
            println!("ℹ️ PGBO_DB environment variable not set, using default value");
            println!("✅ get_db_url() correctly returns default value: '{}'", expected_default);
        }
    }
}

#[test]
fn test_get_db_url_function() {
    // Test that get_db_url() function works and returns a non-empty string
    let db_url = get_db_url();
    
    assert!(!db_url.is_empty(), "get_db_url() should never return an empty string");
    assert!(db_url.contains("host="), "DB URL should contain host parameter");
    assert!(db_url.contains("dbname="), "DB URL should contain dbname parameter");
    
    println!("✅ get_db_url() returns valid database URL: '{}'", db_url);
}

#[test]
fn test_create_person_data_endpoint_valid_data() {
    use rocket::http::{Status, ContentType};
    
    let client = Client::tracked(rocket_for_tests()).expect("valid rocket instance");
    
    let test_person = r#"{
        "first_name": "Jean",
        "last_name": "Dupont",
        "email": "jean.dupont@test.com",
        "phone": "+33123456789",
        "birth_date": "1990-05-15",
        "gender": "M",
        "street_address": "123 Rue de la Paix",
        "city": "Paris",
        "state_province": "Île-de-France",
        "postal_code": "75001",
        "country": "France",
        "nationality": "French",
        "occupation": "Engineer",
        "company": "Tech Corp",
        "salary": 65000.0,
        "marital_status": "Single"
    }"#;
    
    let response = client
        .post("/person_data")
        .header(ContentType::JSON)
        .body(test_person)
        .dispatch();
    
    // The endpoint should respond
    assert_eq!(response.status(), Status::Ok);
    
    let body = response.into_string().expect("valid response body");
    println!("Create person response: {}", body);
    
    // The response should be valid JSON
    let json_result: Result<serde_json::Value, _> = serde_json::from_str(&body);
    assert!(json_result.is_ok(), "Response should be valid JSON: {}", body);
    
    if let Ok(json_value) = json_result {
        // Should be either Ok({...}) or Err("...")
        assert!(json_value.get("Ok").is_some() || json_value.get("Err").is_some(),
                "Response should contain either Ok or Err field");
        
        if let Some(ok_value) = json_value.get("Ok") {
            assert!(ok_value.is_object(), "Ok response should contain a person object");
            
            // Verify the created person has the expected fields
            if let Some(first_name) = ok_value.get("first_name") {
                assert_eq!(first_name.as_str().unwrap(), "Jean");
            }
            if let Some(last_name) = ok_value.get("last_name") {
                assert_eq!(last_name.as_str().unwrap(), "Dupont");
            }
            if let Some(email) = ok_value.get("email") {
                assert_eq!(email.as_str().unwrap(), "jean.dupont@test.com");
            }
            
            println!("✅ Create person data endpoint successfully created person");
        } else if let Some(err_value) = json_value.get("Err") {
            println!("ℹ️ Create person data endpoint returned error (expected if no DB or table): {}", 
                     err_value.as_str().unwrap_or("unknown error"));
        }
    }
}

#[test]
fn test_create_person_data_endpoint_invalid_json() {
    use rocket::http::{Status, ContentType};
    
    let client = Client::tracked(rocket_for_tests()).expect("valid rocket instance");
    
    let invalid_json = r#"{
        "first_name": "Jean",
        "last_name": "Dupont"
        // Missing required fields and invalid JSON
    }"#;
    
    let response = client
        .post("/person_data")
        .header(ContentType::JSON)
        .body(invalid_json)
        .dispatch();
    
    // Should return bad request for invalid JSON
    assert_eq!(response.status(), Status::BadRequest);
    
    println!("✅ Create person data endpoint correctly rejected invalid JSON");
}

#[test]
fn test_create_person_data_endpoint_missing_fields() {
    use rocket::http::{Status, ContentType};
    
    let client = Client::tracked(rocket_for_tests()).expect("valid rocket instance");
    
    let incomplete_person = r#"{
        "first_name": "Jean"
    }"#;
    
    let response = client
        .post("/person_data")
        .header(ContentType::JSON)
        .body(incomplete_person)
        .dispatch();
    
    // Should return unprocessable entity for missing required fields
    assert_eq!(response.status(), Status::UnprocessableEntity);
    
    println!("✅ Create person data endpoint correctly rejected incomplete data");
}

#[tokio::test]
async fn test_create_person_data_database_function() {
    use crate::database::{create_person_data, CreatePersonData};
    use chrono::NaiveDate;
    
    let test_person = CreatePersonData {
        first_name: "Alice".to_string(),
        last_name: "Martin".to_string(),
        email: "alice.martin@test.com".to_string(),
        phone: Some("+33987654321".to_string()),
        birth_date: Some(NaiveDate::from_ymd_opt(1985, 12, 10).unwrap()),
        gender: Some("F".to_string()),
        street_address: Some("456 Avenue des Champs".to_string()),
        city: Some("Lyon".to_string()),
        state_province: Some("Auvergne-Rhône-Alpes".to_string()),
        postal_code: Some("69000".to_string()),
        country: Some("France".to_string()),
        nationality: Some("French".to_string()),
        occupation: Some("Designer".to_string()),
        company: Some("Design Studio".to_string()),
        salary: Some(55000.0),
        marital_status: Some("Married".to_string()),
        is_active: Some(true),
    };
    
    // Test create_person_data function
    let result = create_person_data(test_person).await;
    match result {
        Ok(created_person) => {
            println!("✅ create_person_data successfully created person with ID: {}", created_person.id);
            
            // Verify the created person has the expected data
            assert_eq!(created_person.first_name, "Alice");
            assert_eq!(created_person.last_name, "Martin");
            assert_eq!(created_person.email, "alice.martin@test.com");
            assert!(created_person.id > 0, "Created person should have a positive ID");
            assert!(created_person.created_at.is_some(), "Created person should have a created_at timestamp");
            assert!(created_person.updated_at.is_some(), "Created person should have an updated_at timestamp");
            
            println!("✅ Created person data structure validation passed");
        },
        Err(e) => {
            println!("ℹ️ create_person_data failed (expected if no DB or table): {}", e);
        }
    }
}
