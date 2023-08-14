pub fn public_function(){
    println!("Public function of simple crate called!");
}

pub mod simple_crate_mod{
    use crate::public_function_main;


    pub fn simple_crate_mod_function(){
        println!(" Simple crate mod function called!");

        public_function_main();

    }
}