pub fn function_in_global(){
    println!("Global Function called");
}

// Crate 
pub mod front_of_house{
    
    pub fn add_to_waitlist(){
        println!("Added to waitlist");
        function_in_same_module();
        super::function_in_global();
    }

    pub fn function_in_same_module(){
        println!("function in same module called");
    }
    
} 

fn main() {

    // obsaloute 
    crate::front_of_house::add_to_waitlist();

    // Relative
    front_of_house::add_to_waitlist();

}
