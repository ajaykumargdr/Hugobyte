// similer to interface 
// (one declaration and many definition)

pub trait Summary{

// similer to abstract method

    fn summarize(&self) -> String;

// Default implementation

    fn to_string(&self)-> String{
        String::from("Default implementation")
    }
    
}