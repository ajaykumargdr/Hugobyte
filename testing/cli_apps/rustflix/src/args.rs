use clap::{Args, Parser, Subcommand};

/*//////////// Normal structure

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct RustflixArgs{
    
    /// The first argument
    pub first_arg: String,

    /// The second argument
    pub second_arg: String,

    /// The third argument
    pub third_arg: String
}

// *////////////////////////////////////////////////

#[derive(Debug, Parser)]
// #[clap(author, version, about)]
pub struct RustflixArgs{

    #[clap(subcommand)]
    pub entity_type: EntityType

}

#[derive(Debug, Subcommand)]
pub enum EntityType{

    /// create, update, delete or show users
    User(UserCommand),

    /// create, update, delete or show videos
    Video(UserCommand),

    /// create, update, delete or show views
    View(UserCommand)

}

#[derive(Debug, Args)]
pub struct UserCommand{

    #[clap(subcommand)]
    pub command: UserSubCommand,
    
}

#[derive(Debug, Subcommand)]
pub enum UserSubCommand{

    /// Create a new user
    Create(User),

    /// Update an existing user
    Update(User),

    /// Delete a user
    Delete(User),

    /// Show all users
    Show,
}


// Arguments

#[derive(Debug, Args)]
pub struct User{      // User

    /// The name of the user
    pub name: String,

    /// The email address of the user
    pub email: String,

}