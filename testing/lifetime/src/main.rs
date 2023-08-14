
fn main() {

    long_and_short_scop();

    let s = static_lifetime();

    println!("{s}");

}


/*/ error comes because rust don't know whether ref is refers to
// x or y **Also** we don't know the scope of both

fn longest(x: &str, y: &str) -> &str{

    if x.len() > y.len(){
        x
    } else {
        y
    }
}
########################################################################
*/

// Life time annotating

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str{
    if x.len() > y.len(){
        x
    } else {
        y
    }
}

// Outer scope
fn long_and_short_scop(){

// var1 longests life time

    let string1 = String::from("abch");
    
    let result:&str;
    //Iner scope
    
    {   
        // var2 shortests life time
        
        let string2 = String::from("ABC");
        
        // result will have shortest life time of both var1 var2
        // Note: result should be declared in the same scope of 
        //       the variable which has shortest scope
        
        result= longest(string1.as_str(), &string2.as_str());
        
        println!("{string2}");

        println!("{result}");
    }
    
    // will not work
    // println!("{result}");  

    // will work
    // println!("{string1}");

}

fn static_lifetime() -> &'static str {
    "Static lifetime string"
}
