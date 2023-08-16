use std::thread;
use std::time::Duration;

fn _spawn_thread(){

    //4 ) this will run until the main thread is sleeping
    thread::spawn( || {

        for i in 1..=10{
            println!("===spwan thread print {i}");

        // 5) This will make the crnt spwan thread sleep
        
        thread::sleep(Duration::from_millis(50));
        
        // 6) even this thread is sleeping, if the main thread
        //    wakes up then this spwan thread will be paused

        // 7) when the main thread again start to sleep 
        //    this spwan thread continues it's sleep

        // 8) after finishing it's sleep time it start to print

        }
    });


    // 1) first this will run

    for i in 1..=5{
        println!("main thread print {i}");

    // 2) this makes the main thread sleep 
    // 3) spwan thread will run while this is sleeping

        thread::sleep(Duration::from_millis(1));
    }

}

fn _join_handles(){

    // type thread::JoinHandle<()> 
    let handle= thread::spawn( || {

        for i in 1..=10{
            println!("===spwan thread print {i}");       
            thread::sleep(Duration::from_millis(50));
        }
    });

// 2
    // this means, first wait spwan to finish then move forward

    // handle.join().unwrap();

    for i in 1..=5{
        println!("main thread print {i}");
        thread::sleep(Duration::from_millis(1));
    }

// 1 
    // here same process happens, but after main thread came to
    // this line, and it's wait until spwan thread finish it's work

    // handle.join().unwrap();
}

fn _move_closures(){
    
    let v =  vec![1, 2, 3];

    // let handle = thread::spawn( || {
    let handle = thread::spawn(move || { // works

        // will not work if you don't use move
        println!("Vector in main thread: {:?}", v);

    });

    // what if spwan is not yet started to run (waiting)
    // and we try to delete the vector

    // drop(v);

    // this is why using vector inside other thread is not allowed

    handle.join().unwrap();

    // in this example we moved the entire vector to anoher thread
    // so we don't find any problem
    // but we can't use it again in main thread

}
 
// message passing
// multiple producer, single consumer
use std::sync::mpsc;

fn _message_passing(){

    // tx  => transmitter
    // rx  => receiver

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {     // we should have tx inside

        let val = String::from("hi");

        println!("=== on spawn thread before sending the value");

        tx.send(val).unwrap();
    
        println!("=== on spawn thread after sending the value");

        for i in 0..100{
            println!("{i}");
        }

    });

    println!("on main thread before recieving");

    let received = rx.recv().unwrap();
    println!("Got: {received}");

    println!("on main thread after recieving");

    /* Note:-
        * here first he main thread runs
        * then it want to receive value from other thread it sleeps 
        * it waits until the spwan thread sending the value
        * whenever the spawn thread send a value it receives 
        * and restarts working

        * at this point both thread will work together
        * but the spwan thread works only until the main thread 
          is not dead 

    */

}

fn _multiple_messages_passing(){

    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {

        let vals = vec![
            String::from("msg1"),
            String::from("msg2"),
            String::from("msg3"),
            String::from("msg4")
        ];

        for val in vals{

            println!("=== yet to send {val}");
            tx.send(val).unwrap();
            println!("=== value is sent!\n");
            
            // thread::sleep(Duration::from_secs(1));
        }
    });

    // starts from here

    for received in rx{
        println!("Got: {}", received);
    }

    // if we don't use sleep, spwan thread can bit more time
    // which means it can pass more than 1 message wihtin the 
    // time taken by the main thread to restart

    // once the main thread is restarted, both thread starts 
    // work in parallal

}

fn _multiple_threads(){

    let (tx, rx) = mpsc::channel();

    let tx2 = tx.clone();

    thread::spawn(move || {

        let vals = vec![
            String::from("msg1 from sThread1"),
            String::from("msg2 from sThread1"),
            String::from("msg3 from sThread1"),
            String::from("msg4 from sThread1")
        ];

        for val in vals{

            println!("===ST1 yet to send {val}");
            tx.send(val).unwrap();
            println!("===ST1 value is sent!\n");
            
            thread::sleep(Duration::from_secs(1)); 
        }
    });

    thread::spawn(move || {

        let vals = vec![
            String::from("msg1 from sThread2"),
            String::from("msg2 from sThread2"),
            String::from("msg3 from sThread2"),
            String::from("msg4 from sThread2")
        ];

        for val in vals{

            println!("===ST2 yet to send {val}");
            tx2.send(val).unwrap();
            println!("===ST2 value is sent!\n");
            
            thread::sleep(Duration::from_secs(1));
        }
    });


    // starts from here

    for received in rx{
        println!("Got: {}", received);
    }

    // starts to receive,
    // starts all spwan threads at same time
    // when one of the thread transmits any value 
    // the main thread joins the process

}

fn main() {

// threads

    // _spawn_thread();
    // _join_handles();
    // _move_closures();

// message passing

    // _message_passing();    
    // _multiple_messages_passing();
    _multiple_threads();


}
