use std::result;

fn longest_scope<'a>(longest: &'a str, smallest: &str) -> &'a str{
    longest
} 

fn main() {

    let string1:String = String::from("outer text");

    let result:&str;

    {
        let string2:String = String::from("inner scope text");

        // Note: you havn't used scope of the smallest variable 
        //      so you can declare the 'result' variable in outer 
        //      scope also

        result = longest_scope(&string1, &string2);
        
    }    
    
    println!("{result}");
}
