use std::slice;

fn main() {

    let mut num = 5;

// creating raw pointers

    let r1 = &num as *const i32; // cont immmutatble pointer 
    let r2 = &mut num as *mut i32; // mutable pointer 

    num = 10;

    unsafe{
        // *r2 = 20;
        println!("{:?} {:?}", *r1, *r2);
    }

/////////////// unsafe function

    unsafe fn dangerous() {  /*does nothing */ }

    unsafe {
            dangerous();
    }


//////////////////////

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    println!("{:?} {:?}", a, b);    // [1, 2, 3] [4, 5, 6]

    let (c, d) = split_at_mut(&mut v, 3);
    println!("{:?} {:?}", c, d);

/* ///////////////////////////////////////////////////////

    let address = 0x01234usize;

    let r = address as *mut i32;

    // allows at compile time but throws error at runtime
    // because we don't own these memory (memory that greater than address  )
 
    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 10000) };

    println!("{:?}", values);

*////////////////////////////////////////////////////////
// using c abs function
// this maybe unsafe because c don't follows rust rules 

    unsafe{
        println!("abs of -3 {}", abs(-3));
    }

    println!("{HELLO_WORLD}");


    // counter is mutable, if multi thread using this same variable
    // that would be unsafe.

    unsafe{

        COUNTER += 1;
        println!("{COUNTER}");
    }

}


// c's abs function
extern "C"{
    fn abs(input: i32) -> i32;
}

// Global variable
static HELLO_WORLD: &str = "Hello, world";      // holding immutable reference


static mut COUNTER: u32 = 0;



fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {

    let len = values.len();
    let ptr = values.as_mut_ptr();  // raw pointer

    assert!(mid <= len);

    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }

}