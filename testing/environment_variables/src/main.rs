// use `set` command in windows command line to export environmental variables
// use `export` for linux

fn main() {

    let smtp_username = std::env::var("smtp_username");

    let smtp_password = std::env::var("smtp_password");

    if let Err(su) = smtp_username{
        println!("not set");
    };


    println!("{}", smtp_username.unwrap());

}
