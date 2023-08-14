mod front_of_house{

    pub mod hosting{
        pub fn add_to_waitlist(){
            println!("Added to waitlist");
        }
    }
    
}

// importing 
use crate::front_of_house::hosting::add_to_waitlist;

fn main() {

    // after importing this has a scope
    add_to_waitlist();

}
