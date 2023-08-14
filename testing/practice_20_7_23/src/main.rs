use rand::Rng;
use std::cmp::Ordering;

#[derive(Debug)]
struct UserType{
    x: i32
}

fn main() {

    // number_guessing_game();

    // let mut  s1 = String::from("hello");

    // let mut x = 32; 
    // println!("{x} {s1}");
    // mutable_ref(&mut x, &mut s1);
    // println!("{x} {s1}");

    // let mut ut = UserType{ x:50 };

    // println!("{:?}", ut);
    // mutable_ref_struct(&mut ut);
    // println!("{:?}", ut);

///////////
    let mut s = String::from("hello");

    // let r1 = &mut s;
    // let r2 = &mut s;
    mutable_ref_2(&mut s);

    println!("{}", s);
////////////

    // let mut s = String::from("hello");

    // let r1 = &s; // no problem
    // let r2 = &s; // no problem
    // let r3 = &mut s; // BIG PROBLEM

    // println!("{}, {}, and {}", r1, r2, r3);

/////////////

    // slices();

/////////////

    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);

}

fn number_guessing_game(){


    let mut input_str = String::new(); 
    
    std::io::stdin().read_line(&mut input_str);
    
    let guess_val:i32 = input_str.trim().parse().expect("Enter only number");


    // random
    let random_num:i32 = rand::thread_rng().gen_range(0..=2);

    println!("Input number is:{guess_val} secret number is:{random_num} equals={}", guess_val == random_num);

    match guess_val.cmp(&random_num){

        Ordering::Less => println!("Lesser val"),
        Ordering::Greater => println!("Greater val"),
        Ordering::Equal=> println!("Equal val")
    }

}

fn mutable_ref(var: &mut i32, str_var: &mut String){
    *var = 64;

    // str_var.push_str("added_str");
    *str_var = String::from("changed");

}

fn mutable_ref_2(str_val: &mut String){
    *str_val = String::from("changed");
}

fn mutable_ref_struct(object: &mut UserType){
    object.x = 5;
}

fn slices(){

    let string_ = String::from("This is the string");


    let x = string_.as_bytes();

    println!("{:?}", x);
    println!("{}", b' ');

    for (i, itr) in x.iter().enumerate(){

        if *itr == b' '{
            println!("{}", &string_[0..i] );
            break;
        }

    }


}