use postgres::{Client, Error, NoTls};
use std::{io, process::exit};

pub fn controller(menu_choice: String, client: &mut Client)-> Result<(), Error>{

    println!("{menu_choice}");

    match menu_choice.trim().parse::<i32>().unwrap(){
        1 => {
            insert(client)?;
            println!("\nRequest successfully inserted");
        }
        2 => {
            edit(client)?;
            println!("\nRow successfully update");
        }
        3 => {
            delete(client)?;
            println!("\nRow successfully deleted");
        }
        4 => {
            display(client)?;
        }
        5 => {
            query(client)?;
        }
        6 => {
            exit(0)
        }
        _ => println!("Option is wrong!"),
    }

    Ok(())
}

pub fn get_connection_client() -> Result<Client, Error> {
    Client::connect("postgresql://postgres:pgdb9488@localhost:5432/akdb", NoTls)
}

pub fn create_table_if_not_exist(client: &mut Client) -> Result<(), Error> {
    client.execute("CREATE TABLE IF NOT EXISTS person ( id SERIAL PRIMARY KEY, name VARCHAR(30), data VARCHAR(150));", &[])?;
    Ok(())
}

pub fn insert(client: &mut Client) -> Result<(), Error> {
    println!("");
    println!("*************");
    println!("Input Data");
    println!("*************");
    let mut username = String::new();
    println!("Enter the person name below :");

    io::stdin()
        .read_line(&mut username)
        .expect("failed to readline");

    let mut data = String::new();

    io::stdin()
        .read_line(&mut data)
        .expect("failed to readline");

    /////////////////////////////
    println!("|{data}|");

    client.execute(
        "INSERT INTO person (name, data) VALUES ($1, $2)",
        &[&username, &data],
    )?;

    Ok(())
}

pub fn edit(client: &mut Client) -> Result<(), Error> {

    println!("\n***********Edit Person Details***********\n");

    display(client)?;

    // Getting the Person Id
    println!("Enter person Id:");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("failed to readline");
    println!("");

    // getting new data
    println!("Enter person details:");

    let mut new_username = String::new();

    io::stdin()
        .read_line(&mut new_username)
        .expect("error input");

    let mut new_user_data = String::new();

    io::stdin()
        .read_line(&mut new_user_data)
        .expect("error input");

    let index = index.trim().parse::<i32>().unwrap();

    client.execute(
        "UPDATE person SET name=$1,data=$2 WHERE id=$3;",
        &[&new_username, &new_user_data, &index],
    )?;

    Ok(())
}

pub fn display(client: &mut Client) -> Result<(), Error> {

    println!("*****************\nDisplay All Data!\n*****************");

    for row in client.query("SELECT id, name, data FROM person", &[])? {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let data: &str = row.get(2);

        println!("Id: {} |Name: {} |Data: {}", id, name, data);
        println!("-----------------------");
    }

    Ok(())
}

pub fn delete(client: &mut Client) -> Result<(), Error> {

    println!("\n***********\nDelete Data\n***********");

    display(client)?;
    println!("Enter person id: ");

    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Please enter a number");
    let index = index.trim().parse::<i32>().unwrap();
    
    client.execute("DELETE FROM person WHERE id = $1;",&[&index])?;

    Ok(())
}

pub fn query(client: &mut Client) -> Result<(), Error> {
    println!("");
    println!("***********");
    println!("Query a row");
    println!("***********");
    println!("");
    println!("Enter an id:");
    let mut index = String::new();
    io::stdin()
        .read_line(&mut index)
        .expect("Please enter a number");
    let index = index.trim().parse::<i32>().unwrap();
    for row in client.query("SELECT  name, data FROM person  WHERE id =$1", &[&index])? {
        let name: &str = row.get(0);
        // let data: Option<&[u8]> = row.get(1);

        println!("Selected row has {}  ", name);
    }

    Ok(())
}

fn main() -> Result<(), Error> {

    let mut client: Client = get_connection_client()?;

    // deleting table (developing purpose)
    // client.execute("DROP TABLE person;", &[])?;

    create_table_if_not_exist(&mut client)?;

    loop {

        // Menu
        println!("\n
            1. Insert Data\n
            2. Edit Data\n
            3. Delete Data\n
            4. Display Data\n
            5. Query Data\n
            6. Exit");

        let mut menu_choice = String::new();

        io::stdin()
            .read_line(&mut menu_choice)
            .expect("failed to read the input");

        // Matching the option
        controller(menu_choice, &mut client)?;
    }

}

// docker build -t postgres-rust-cli .