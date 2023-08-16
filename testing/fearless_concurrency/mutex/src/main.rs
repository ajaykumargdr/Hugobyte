use std::sync::Mutex;
use std::thread;

fn _mutex(){

    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num =6;
    }

    println!("m = {:?}", m);
}

/*
fn _trying_mutex(){

    let counter = Mutex::new(0);
    let mut handles = vec![];

    for _ in 0..10 {

        let handle = thread::spawn(move ||{
  
            let mut num = counter.lock().unwrap();

            *num + 1;

        }); // mutex will be automatically unlocked here 

        handles.push(handle);
    }

    for handle in handles{
        handle.join().unwrap();
    }

    println!("Result: {:#?}", *counter.lock().unwrap());


    // Note:-
    // this code will not compile
    // because even we are locking the variable, 
    // still we are moving the variable `counter`
   
    // we have to send mutable reference to all of these threads
    // Note: (mutex can only do locking)
    // 
}
*/

use std::rc::Rc;
 
/*
fn _trying_rc_mutex_(){

    let counter = Rc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Rc::clone(&counter);
        let handle = thread::spawn(move || {
            
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Note: in this situation you cannot use Rc pointer also
    // because it doesnot support counting reference on multiple threads
    // [or] it doesn't shares reference counter to multiple threads 
} */

// using Arc pointer

use std::sync::Arc;

fn _arc_mutex_(){

    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            
            println!("working on {i}th thread");
            
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());

    // Note:-
    // Arc is same like Rc but this shares ref counter upon multiple threads 

    // finally we are using a single variable upon multi threads 
    // and we can also mutate it
    // but before mutating we will lock it
    // so that no other thread will access it
}


fn main() {

    // _mutex();

    _arc_mutex_();

}
