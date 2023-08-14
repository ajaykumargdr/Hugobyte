use rand::*;

pub mod random_num{
   use super::*;

    pub fn get_random_number()->i32{
        rand::thread_rng().gen_range(1..=100) 
    }
} 