// Delaring and Defining modules

pub mod front_of_house{
    
    pub mod hosting {
        pub fn add_to_waitlist(){
             println!("Added to List");
        }

        pub fn seat_at_table(){
            println!("Seat at table");
        }
    }

    pub mod serving{
        pub fn take_order(){
            println!("Take order");
        }

        pub fn serve_order(){
            println!("Serve order");
        }

        pub fn take_payment(){
            println!("Take payment");
        }
    }
}