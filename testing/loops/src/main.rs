fn main() {
    
// Normal loop

    loop{
        println!("Hello, world!");
        break   // can have ; also
    }

// Returning Values from loop

    let mut counter = 0;
    
    let result = loop{
        counter += 1;

        if counter == 10{
            break counter * 10
        }
    };

    println!("counter:{counter} result:{result}");
    
// Multiple loops & :loop lables-

    counter =0;

    'outer_loop: loop{

        'inner_loop: loop{
            counter += 1;
            
            if counter == 2 {
                break 'inner_loop;
            }
            
            if counter == 4{
                break 'outer_loop;
            }

            println!("Inner Loop {counter}");
            
        }
        
        println!("Outer Loop {counter}");

    }

// while Loop

    let a = [10, 20, 30, 40, 50];


    let mut index = 0;

   while index < 5 {
        println!("a[{index}] is: {}", a[index]);
        index += 1;
    };

    // println!("Last element is: {x}");

// for loop

    for i in a {
        println!("one of the value of array {i}");
    }

// for range loop and rev method

    for i in (0..5).rev(){
        println!("Value in a[{index}] is: {}",a[i]);
    }




}