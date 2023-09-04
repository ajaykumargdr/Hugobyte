/* 
use reqwest;
// use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let client = reqwest::Client::new();
    let db_name = "my_database";

    // Client::connect("postgresql://postgres:pgdb9488@localhost:5432/akdb", NoTls)

    // Create a new database
    let create_db_url = format!("http://ajaykumarm:db9488@localhost:5984/{}", db_name);

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
} */

use chesterfield::sync::Client;
use chesterfield::GetResponse;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestData{
    name: String,
    data: i32,
}

// db getting connection
pub fn get_connection_client() -> Client{

    Client::new("http://ajaykumarm:db9488@localhost:5984/")
            .expect("cannot connect to the data base")

}

pub fn create_database_if_not_exist(client: &mut Client, db_name: &str){

   let db =  client.database(db_name).expect("cannot find the data base");

   if !db.exists().unwrap(){
        db.create().expect("not able to create database");
   }

   println!("database successfully created");
}  

pub fn create_document(client: &mut Client, db_name:&str, data: TestData) -> String{

    let db = client.database(db_name).expect("cannot connect to the given data base");

    db
        .insert(&data, None)
        .send()
        .expect("not able to create document")
        .id
}

pub fn get_document(client: &mut Client, db_name: &str, doc_id: &str) -> TestData{

    let db = client.database(db_name).expect("cannot connect to the given data base");

    println!("---- 1 ------");

    // this method call prints some unnecessary data

    let doc_get_response:GetResponse<TestData> = db
        .get(doc_id)
        .send()
        .expect("cannot retrieve document from the database");

        
    println!("---- 2 ------");

    let document:TestData = doc_get_response
        .into_inner()
        .expect("not able to get the document from the response");

        
    println!("---- 3 ------");
    
    document


}

fn main(){

    let mut client = get_connection_client();

    let db_name = String::from("test_db");

    // Create db
    create_database_if_not_exist(&mut client, &db_name);
    
    // create document
    
    let data = TestData{
        name: String::from("Rustian"),
        data: 1
    };

    let doc_id = create_document(&mut client, &db_name, data);

    // get document

    let doc = get_document(&mut client, &db_name, &doc_id );

    println!("{:?}", doc);

}