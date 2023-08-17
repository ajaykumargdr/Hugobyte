struct Point {
    x: i32,
    y: i32,
}

fn _struct_obj_destruct(){

    let p = Point { x: 0, y: 7 };

    // same as if let Some(x)
    let Point { x: a, y: b} = p;
    println!("{a} {b}");

// ====================================

    let Point{x, y} = p;
    println!("{x} {y}");

// ====================================

    match p{
        Point {x, y:0} => println!("x anything, y is 0"),
        Point {x:0, y} => println!("y anything, x is 0"),
        Point {x, y} => println!("x and y anything")
    }

    // here x and y should be named x and y itself

}

fn _ignorable_matches(){

    // _struct_obj_destruct();

    let mut setting_value = None;
    let new_setting_value = Some(10);

    match (setting_value, new_setting_value) {
        (Some(_), Some(_)) => {
            println!("\nCan't overwrite an existing customized value");
        }
        _ => {
            setting_value = new_setting_value;
        }
    }

    println!("setting is {:?}", setting_value);   

// ========================================================================

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, _, third, _, fifth) => {
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

}

fn _move_prevent_matching(){

    let s = Some(String::from("Hello!"));

    // if let Some(_s) = s {    // value will move to _s
    if let Some(_) = s {

        println!("found a string");
    }

    println!("{:?}", s);

}

struct Point2{
    x: i32,
    y: i32,
    z: i32
}

fn _rang_ignore(){

    let origin = Point2 { x:2, y:4, z:8};

    match origin{
        Point2 { x, .. } => println!("only first value focused"),
        // Point2 { .., z} => println!("only last value focused"), // will not work
        // Point2 { x, .., z } => println!("middle focused"), // will not work
        Point2 { x, .., z } => println!("first and last focused")
    }

}

fn main() {

    // _struct_obj_destruct();
    // _ignorable_matches();
    // _move_prevent_matching();    

}
