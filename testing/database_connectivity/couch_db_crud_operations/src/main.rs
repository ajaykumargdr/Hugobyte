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
use serde_json::Value;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TestData {
    name: String,
    data: i32,
}

// db getting connection
pub fn get_connection_client() -> Client {
    Client::new("http://ajaykumarm:db9488@localhost:5984/")
        .expect("cannot connect to the data base")
}

pub fn create_database_if_not_exist(client: &mut Client, db_name: &str) {
    let db = client.database(db_name).expect("cannot find the data base");

    if !db.exists().unwrap() {
        db.create().expect("not able to create database");
        println!("database successfully created");
    } else {
        println!("database already exists");
    }
}

fn is_doc_exists(
    client: &mut Client,
    db_name: &str,
    doc_id: &str,
) -> Option<GetResponse<TestData>> {
    let db = client
        .database(db_name)
        .expect("cannot connect to the given data base");

    let doc_get_response: Result<GetResponse<TestData>, _> = db.get(doc_id).send();

    if let Ok(response) = doc_get_response {
        return Some(response);
    }

    None
}

pub fn get_document(client: &mut Client, db_name: &str, doc_id: &str) -> Option<TestData> {
    let doc_get_response = is_doc_exists(client, db_name, doc_id);

    if let Some(response) = doc_get_response {
        let document: TestData = response
            .into_inner()
            .expect("not able to get the document from the response");

        return Some(document);
    }

    None
}

pub fn create_document(client: &mut Client, db_name: &str, data: TestData, new_doc_id: &str) {
    let doc_exist = is_doc_exists(client, db_name, new_doc_id);

    if let Some(_) = doc_exist {
        println!("doc already exists!");
    } else {
        client
            .database(db_name)
            .expect("cannot connect to the given data base")
            .insert(&data, String::from(new_doc_id))
            .send()
            .expect("not able to create document");

        println!("new doc created, Id: {new_doc_id}");
    }
}

pub fn update_document(client: &mut Client, db_name: &str, doc_id: &str, updated_data: TestData) {
    let mut _query_doc = is_doc_exists(client, db_name, doc_id);

    if let Some(response) = _query_doc {
        client
            .database(db_name)
            .expect("cannot connect to the given database")
            .update(
                &updated_data,
                String::from(doc_id),
                response.meta_data()._rev.clone(),
            )
            .send()
            .expect("not able to create document");

        println!("document updated!");
    } else {
        println!("doc id {doc_id} not exist");
    }
}

pub fn delete_document(client: &mut Client, db_name: &str, doc_id: &str) {
    let _query_doc = is_doc_exists(client, db_name, doc_id);

    if let Some(response) = _query_doc {
        client
            .database(db_name)
            .expect("cannot connect to the given database")
            .delete(doc_id, response.meta_data()._rev.clone())
            .send()
            .expect("not able to delete document");

        println!("document {doc_id} deleted successfully");
    } else {
        println!("document not exist");
    }
}

fn main() {
    let mut client = get_connection_client();
    
    // Inserting data
    ///////////////////////////////////////////////////////////////////////
    // let db = client
    //             .database("event_registration_db")
    //             .unwrap()
    //             .create()  
                // .expect("not able to create test database");
 
    let json_value = r#"{"name": "icon-eth-notification", "trigger": "0a36fd24-84ac-420e-9187-912929c782ea"}"#;

    let data:Value = serde_json::from_str(json_value).unwrap();

    client
        .database("event_registration_db")
        .unwrap()
        .insert(&data, String::from("0a36fd24-84ac-420e-9187-912929c782ea"))
        .send()
        .expect("not able to create document");

    ////////////////////////////////////////////////////////////////////////////

    ////////////////////////////////////////////////////////////////////////////////

    let db = client
        .database("event_registration_db").unwrap();

    let doc_get_response: Result<GetResponse<Value>, _> = db
            .get("0a36fd24-84ac-420e-9187-912929c782ea")
            .send();

    let data:Value = doc_get_response.unwrap()
                            .into_inner()
                            .unwrap()
                            .get("name")
                            .unwrap()
                            .clone();

    let action_name:String =  serde_json::from_value(data).unwrap();


    println!("{:?}", action_name);

    delete_document(&mut client, "event_registration_db", "0a36fd24-84ac-420e-9187-912929c782ea");


























    ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
    // const DB_NAME: &'static str = "test_db";

    // /////////////////////////// CREATE database //////////////////////////////

    // create_database_if_not_exist(&mut client, DB_NAME);

    // //////////////////////////// CREATE document //////////////////////////////

    // let data = TestData {
    //     name: String::from("Rustian"),
    //     data: 1,
    // };

    // const DOC_ID: &'static str = "my-couch-db-id";

    // create_document(&mut client, DB_NAME, data, DOC_ID);

    // ///////////////////////////// READ document //////////////////////////////////

    // let doc = get_document(&mut client, DB_NAME, DOC_ID);
    // println!("{:?}", doc);

    // ///////////////////////////// UPDATE document ////////////////////////////////

    // let updated_data = TestData {
    //     name: String::from("Rustian 2.0"),
    //     data: 2,
    // };

    // update_document(&mut client, DB_NAME, DOC_ID, updated_data);

    // ////////////////////////////// DELETE document ////////////////////////////////

    // delete_document(&mut client, DB_NAME, DOC_ID);
}
