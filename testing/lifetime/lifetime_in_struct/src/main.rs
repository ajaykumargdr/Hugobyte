#[derive(Debug)]
struct ImportantExcerpt<'a>{
    part: &'a str
}

fn main() {

    let novel = String::from("Call me HugoBytian. Some years ago...Moral of the story");

    // split gives Split<'_, char>
    // next() gives Option<&'a str>
    // expect gives &str or throws panics

    let first_sentence = novel.split('.').next().expect("!!!!Could not fund a '.'!!!!");
    
    // at this step first_sentence will have &str

    let i = ImportantExcerpt{
        part: first_sentence,
    };

    println!("{:?}", i);

    println!("{first_sentence}");

    test();
}

fn test(){

    let ie:ImportantExcerpt;  

    {
        let string1 = String::from("string");

        ie = ImportantExcerpt{
            part: &string1
        };

        println!("{:?}", ie);        // works   

    }

    // println!("{:?}", ie);        // not works   

}

fn test_static_lifetime(){

    let ie: ImportantExcerpt;

    {
        let string1 = "the static lifetime string";

        ie = ImportantExcerpt{
            part: &string1
        };

        println!("{:?}", ie);
    }

    println!("{:?}", ie);

}


