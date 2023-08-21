trait Pilot {
    fn fly(&self);
}

trait Wizard {
    fn fly(&self);
}

struct Human;

impl Pilot for Human {
    fn fly(&self) {
        println!("This is your captain speaking.");
    }
}

impl Wizard for Human {
    fn fly(&self) {
        println!("Up!");
    }
}

impl Human {
    fn fly(&self) {
        println!("*waving arms furiously*");
    }
}

fn main() {
    
    let person = Human;

    person.fly();
    Human::fly(&person);
    Pilot::fly(&person);
    Wizard::fly(&person);

    // methods that don't have &self params

    println!("{}", Dog::baby_name());   // Spot
    // println!("{}", Animal::baby_name());  // will not work
    
    println!("{}", <Dog as Animal>::baby_name());   // Spot
    

}

// no &self parameter
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

// we have implemented Animal::baby_name() for Dog
impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}