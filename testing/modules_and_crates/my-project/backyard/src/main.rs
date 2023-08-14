use restaurant;

// // Crate root file
// // Declaring main module
pub mod garden;

use crate::garden::vegetables::Asparagus;
use crate::garden::fruites::Mango;
use crate::garden::fruites::Goa;

fn main() {
    let planet = Asparagus {hw: String::from("Hello World")};
    println!("{:?}", planet);

    println!("{:?}", Mango{name: String::from("Mango_Type1")});

    println!("{:?}", Goa{name: String::from("Goa_Type1")});

    restaurant::front_of_house::hosting::add_to_waitlist();

}
