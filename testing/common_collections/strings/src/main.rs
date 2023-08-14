fn main() {
    
    let mut s1 = "Hugo".to_string();
    let s2 = "Bytes";
    s1.push_str(s2);

    println!("{s2}");

// string adding

    let s1 = String::from("Hello!, ");  
    
    let s2 = String::from("World");

    let s3 = s1 + &s2;  // adding

    println!("{s3}");    //  s1 is not accessable 

// format! macro

    let s1 = String::from("Hug");
    let s2 = String::from("o");
    let s3 = String::from("Byte");

    let s = format!("{s1}--{s2}--{s3}");

    println!("{s}");

//  string slices (works in &str also)

    let s = "HugoByte".to_string();

    println!("{}", &s[0..3] );

// Iterating over string

    // character values
    for c in s.chars(){
        println!("{c}");
    }

    // Integer values
    for i in s.as_bytes(){
        print!("{i} ");
    }

}
