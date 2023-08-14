#[derive(Debug, PartialEq)]
pub struct Rectangle{
    width: u32,
    height: u32,
}

impl Rectangle{

    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && 
        self.height > other.height
    }
}

pub fn add_two( x: i32) -> i32{
    x + 2
}

#[cfg(test)]
mod tests {

    use super::*;

// Test exampl1

    #[test]
    fn exploration() {
        // let result = 2 + 2;

        // throws panic when test case fails
        assert_eq!(2 + 2, 4);
    }

// Test example2

    #[test]
    fn another(){
        panic!("Make this test fail");
    }

// Test example3

    #[test]
    fn larger_can_hold_smaller(){
        
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger(){
        
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        assert!(smaller.can_hold(&larger));
    }

    #[test]
    fn smaller_cannot_hold_larger_not(){
        
        let larger = Rectangle{
            width: 8,
            height: 7
        };

        let smaller = Rectangle{
            width: 5,
            height: 1
        };

        assert!(!smaller.can_hold(&larger));
    }

    #[test]
    fn it_adds_two(){
        assert_eq!( 52, add_two(50));
    }

    #[test]
    fn it_adds_two_eq(){
        assert_ne!(52, add_two(50));
    }

// assert_eq! and assert_ne! in User defined structs

    #[test]
    fn rects_are_equal(){
        
        let rect1 = Rectangle{
            width: 8,
            height: 7
        };

        let rect2 = Rectangle{
            width: 5,
            height: 1
        };

        // must derive PartialEq
        assert_eq!(
            rect1, 
            rect2,
            "\nThe rectangle {:?} and {:?} are not equal\n",rect1,rect2 
        );

    // Note: You can also pass a error message 
    // within assert!, assert_eq!, asser_ne! 
    
    }


//  #[should_Panic] (pass if throws panic!)

    #[test]
    #[should_panic]
    fn should_panic_test(){
        panic!("Test is passed if this throws panic message");
    }

    #[test]
    #[should_panic(expected = "given message")]
    fn should_panic_with_given_message(){
        panic!("Not equals to the given-message");
    }

    // Note: it passes if the message is substring
    // of the message passed 

    // if the ecpected message is not equals to the throwed message it fails



}
