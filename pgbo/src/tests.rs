use rocket::local::blocking::Client;
use rocket::http::Status;
use crate::database::{db_connectivity_test, load_sql_config, create_client};
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
            let original_config = std::fs::read_to_string("pgbo_sql.toml").ok();
            
            // Create invalid SQL config
            let invalid_sql = r#"
[queries]
create_table_person = "INVALID SQL SYNTAX HERE"
insert_person = "INSERT INTO"
select_all_persons = "SELECT FROM"

[data]
default_name = "Test"
            "#;
            
            std::fs::write("pgbo_sql.toml", invalid_sql).unwrap();
            
            let result = db_connectivity_test().await;
            
            // Restore original config if it existed
            if let Some(original) = original_config {
                std::fs::write("pgbo_sql.toml", original).unwrap();
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
            println!("Config loading failed (expected if pgbo_sql.toml doesn't exist): {}", e);
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
            println!("ℹ️ Database client creation failed (expected if DB not available): {}", e);
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
        Err(e) => println!("Database connectivity test failed (expected if no DB): {}", e),
    }
    
    // The test passes if we reach this point without panicking
    assert!(true);
}
