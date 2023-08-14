#[derive(Debug)]
struct User{
    active: bool,
    username: String
}

impl User{

    fn build_user(active: bool, username:&String) -> Self //User
    {
        User{
            active, 
            username : username.clone()
        }
    }

    // fn new()
}

// enum with array
enum Message{
    arr([i32;5]),
    None
}


#[derive(Debug, PartialEq, PartialOrd)]
// Unit-'()' struct 
struct QuitMessage;


fn main() {

// ###############################################

    let user0 = User::build_user(true, &String::from("user_0A"));

    println!("{:?}", user0);


// ###############################################

    let user1 = User{
        active: true,
        username: String::from("User_A")
    };

    let user2 = User{
        username: user1.username.clone(),
        ..user1
    };

    println!("{:?} {:?}", user1, user2);

// ###############################################

    println!("{}", area( [2,3, 1]));

// ###############################################

    // string slicing will not work 
    // let tpl = (4, 5, true);

    // println!("{}", &tpl(i32, i32));

// ###############################################

    let mut vec1:Vec<i32> = Vec::new();
    vec1.push(5);
    vec1.push(4);
    vec1.push(3);
    vec1.push(2);
    vec1.push(1);

    let something =  &vec1[1..4];

    println!("{:?} ", something);

// ###############################################

    let m1 = QuitMessage;
    let m2 = QuitMessage;    

    println!("{}", m1>m2 );

// ###############################################

    let config_max:Option<u8> = Some(3u8);
    
    if None == config_max {
        println!("The maximum is not configured " );
    }

    if let Some(max) = config_max{
        println!("The maximum is configured to be {}", max);
    }
    
// ###############################################


}

fn area(dimensions: [u32;3] ) -> u32 {
    dimensions[0]* dimensions[1] + dimensions[2]
}
