use reqwest;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let db_name = "my_database";

    // Create a new database
    let create_db_url = format!("http://localhost:5984/{}", db_name);
    let response = client
        .put(&create_db_url)
        .send()
        .await?;

    if response.status().is_success() {
        println!("Database '{}' created successfully.", db_name);
    } else {
        println!("Failed to create the database. Status code: {:?}", response.status());
    }

    Ok(())
}

