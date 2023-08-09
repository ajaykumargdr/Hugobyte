// modules1.rs
// Execute `rustlings hint modules1` or use the `hint` watch subcommand for a hint.

mod sausage_factory {
    // Don't let anybody outside of this module see this!
    pub fn get_secret_recipe() -> String {
        String::from("Ginger")
    }

    pub fn make_sausage() {
        get_secret_recipe();
        println!("sausage!");
    }
}

use sausage_factory::*;

fn main() {
    make_sausage();
    get_secret_recipe();
}
