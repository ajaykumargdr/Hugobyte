#[derive(Debug)]
struct User{
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64
}

struct Employee(bool, String, String, u64);

// not works (slice type &str not works)
// struct test(bool, &str, &str, u64);
let home = IpAddr::V4(String::from("127.0.0.1"));
    
fn main() {
    
// New user
    let mut user1 = User{
        active: true,
        username: String::from("Ajay Kumar"),
        email: String::from("ajay.kumar@hugobyte.com"),
        sign_in_count: 1
    };

// partial copy 

    let user2 = User{
        sign_in_count: 0,
        ..user1
    };
    
// Copy (deep copy)

    let user3 = User{
        ..user2
    };

    println!("{}", user3.active);

// move copy (ownership will be moved)

    let user4 = user3;

    // not works 
    // println!("{}", user3.active); 

    println!("{}", user4.active);

// Tuple Struct

    let employee1 = Employee(true, String::from("emp1"), String::from("ep@hg"), 2);

    println!("{}", employee1.2);

// Unit struct

    // let unit_ = AlwaysEqual;

    // println!("{unit_}" );

// Derived Traits

    // not works
    // println!("{}", user1);

    // *** Should have the same name 
    let active = true;

    let user_test = User{
        active,
        username:String::from("Test user"),
        email:String::from("ut@hb"),
        sign_in_count:56,
    };

    // works
    println!("{:?}", user_test);    // not formated
    println!("{:#?}", user_test);   // formated 
    dbg!(&user_test);

}

fn get_ak_info() -> User{

    User{
        active: true,
        username: String::from("Ajay Kumar"),
        email: String::from("ajay.kumar@hugobyte.com"),
        sign_in_count: 1
    }

}

fn get_new_user(username:String, email:String)->User{
    User{
        active: true,
        username,
        email,
        sign_in_count:1
    }
}