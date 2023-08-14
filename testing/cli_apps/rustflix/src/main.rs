mod args;

use args::{RustflixArgs, EntityType, UserSubCommand, User};
use clap::Parser;

fn main() {
  
    let args = RustflixArgs::parse();
    
    match args.entity_type{

        EntityType::User(userCmd) =>  { 
            
            match userCmd.command{
                 UserSubCommand::Create(user) => { 
                    println!("Username: {},  UserEmail: {}", user.name, user.email);
                    println!("User created");
                 }
                 UserSubCommand::Update(user) => {
                    println!("Username: {},  UserEmail: {}", user.name, user.email); 
                    println!("User Updated");
                 }
                 UserSubCommand::Delete(user) => {
                    println!("Username: {},  UserEmail: {}", user.name, user.email); 
                    println!("User Deleted");
                 }
                 UserSubCommand::Show => { 
                    println!("User List is Shown");
                 }
            }


        }

        EntityType::Video(_) => { 

        }

        EntityType::View(_) => { 

        }

    }
    

}


// cargo install --path .   // to install CLI
// rustflix                 // to run  