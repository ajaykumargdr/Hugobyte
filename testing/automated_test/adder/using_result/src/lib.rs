/// cargo test -- --show-output // to use println
/// cargo test <function_name1> <fun_name2> // run particular test cases 
/// or pass substring to test matching functions
// cargo test -- --ignored


#[cfg(test)]
mod tests{

    #[test]
    fn it_works() -> Result<(), String>{
        let x = 4;

        if 2 + 2 == x{

            // will not work 
            // use cargo test -- --show-output
            println!("### 2 + 2 is equal to {x} ###");

            Ok(())  // returnes unit value
        } else {
            Err(String::from("two plus two does not equal to {x}"))
        }
    }

    #[test]
    fn it_returns_error() -> Result<(), String>{
        let x = 6;

        if 2 + 2 == x{
            Ok(())  // returnes unit value
        } else {

            // works
            // println!("###Enteres into not equls block###");

            Err(String::from("two plus two does not equal to {x}"))
        }
    }

    #[test]
    fn function_calls_another_failed_function(){
          
       match it_returns_error(){

            Ok(()) => panic!("### The function is passed ###"),

            Err(msg) => print!("### This function is passed because the called function failed ### error was:{msg}#")

       } 

       // Note: in Ok(()) case it will not print anything
       // it just passes the case 
       // you can use cargo test -- --show-output to 
       //print the output
    }
 
    #[test]
    #[ignore = "## this is the reason ###"]
    // #[should_panic(expect="### Should panic because of this ###")]
    fn ingored_test_case(){
        panic!("\n### This test case will be ignored until we use #[ignore] you can also specify the reason###");
    }

    

}