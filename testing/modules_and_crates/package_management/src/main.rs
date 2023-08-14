use tree_crate;

mod simple_crate;

use tree_crate::tree_crates_branch1::branch1_crate_mod_function1;

fn main() {
    println!("Main function called!");

    simple_crate::public_function();
    simple_crate::simple_crate_mod::simple_crate_mod_function();

    branch1_crate_mod_function1();
    tree_crate::tree_crates_branch1::branch1_crate_mod_function2();
        
    tree_crate::tree_crates_branch2::branch2_crate_mod_function1();
    tree_crate::tree_crates_branch2::branch2_crate_mod_function2();

}

pub fn public_function_main(){
    println!("public function of main called!");
} 
