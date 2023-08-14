// use super::*;

#[cfg(test)]
mod tests{

    use std::vec;
    use super::super::*;

    #[test]
    // #[should_panic]
    fn search_query(){
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";
        
        assert_eq!(vec!["safe, fast, productive."],
        search::search(query, contents));
    }



}
