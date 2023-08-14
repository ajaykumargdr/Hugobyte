fn main() {
    
    // println!("{}",fahrenheit_to_celsius(98.6));
    // println!("{}",celsius_to_fahrenheit(37.0));

    nth_fibonacci(10);
    // sing_carol();

}

fn fahrenheit_to_celsius(fh: f32) -> f32 {
    ((fh-32.0)*5.0) / 9.0
}

fn celsius_to_fahrenheit(c: f32) -> f32{
    ((c*9.0) / 5.0) + 32.0 
}

//0, 1, 1, 2, 3, 5, 8, 13,
fn nth_fibonacci(n: u32){

    
    let mut a = 0;
    let mut b = 1;
    let mut c = a+b;

    for i in 1..=n {

        println!("{a}");

        c = a+b;
        b = a;
        a = c; 
    }

    
    
}

// On the fifth day of Christmas, my true love sent to me
// Five golden rings
// Four calling birds
// Three french hens
// Two turtle doves, and
// A partridge in a pear tree

fn sing_carol(){

    let song = [
        "On the first day of Christmas, my true love sent to me",
        "Five golden rings",
        "Four calling birds",
        "Three french hens",
        "Two turtle doves, and",
        "A partridge in a pear tree"
    ];

    for i in 0..6{ 
        println!("{}", song[0]);

        for i in 6-i..6{
            println!("{}", song[i]);
        }

        println!("");

    }


}