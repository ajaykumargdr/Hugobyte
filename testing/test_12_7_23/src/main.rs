fn main() {
    
    let mut count =1; 

    loop{

        if count%5 == 0{
            println!("beep");
        }else {
            println!("{count}");
        }

        if count==50{
            break
        }

        count+=1;
    }


}
