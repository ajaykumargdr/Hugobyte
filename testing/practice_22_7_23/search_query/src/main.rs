use search::{self, Config};
use std::env;

mod tests;

fn main(){

    let args:Vec<String> = env::args().collect();

    let c = Config::build(&args).expect("Pass needed arguments");

    match search::run(&c){
        Ok(()) => (),
        Err(err) => {
            panic!("{:?}", err);
        }
    }

}



