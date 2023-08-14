use std::collections::HashMap;

fn main() {
    
    // Declaration
    let mut map1:HashMap<String, i32> = HashMap::new();

    let s = String::from("key1");

    map1.insert(s, 10);

    // println!("{s}"); // not accessable

    println!("{:?}", map1);

    // getting value back // return Optoion<&T>
    println!("{:?}", map1.get("key1"));

    let val = map1.get("key02").copied().unwrap_or(0);

    println!("{val}");

// iterating on hashmap

    for (key, value) in &map1{
        println!("{key} {value}");
    }


// Overwriting a value

    let mut map1 = HashMap::new();

    map1.insert("Blue".to_string(), 10);
    map1.insert("Blue".to_string(), 25); // 10 replaced

    println!("{:?}", map1); // 25   

// add a pair only if it is not there in the map

    map1.entry(String::from("Blue")).or_insert(50);
    map1.entry(String::from("Green")).or_insert(50);
    map1.entry(String::from("Red")).or_insert(100);
    println!("{:?}", map1); // Green 50    
}
