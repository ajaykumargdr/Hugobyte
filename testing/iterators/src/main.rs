fn main() {

// ########################################################

    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter();

    for val in v1_iter{
        println!("{val}");
    }

// ########################################################

   let mut v1_iter = v1.iter();

   println!("{:?}", v1_iter);   v1_iter.next();
   println!("{:?}", v1_iter);   v1_iter.next();
   println!("{:?}", v1_iter);   v1_iter.next();
   println!("{:?}", v1_iter);   v1_iter.next();
   println!("{:?}", v1_iter);   // no error just Iter([])

// ########################################################

// method consumes iterators

    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();

    let sum:i32 =  v1_iter.sum();
    println!("{sum}");

    // println!("{:?}",v1_iter);   // moved


// ########################################################

// method consumes iterators

    let v1 = vec![1, 2, 3];
    let mut v1_iter= v1.iter().map(|x| x+1);  // returns another iter
    v1_iter.next();
    println!("{:?}", v1_iter);

// ########################################################

    let shoes = vec![
            Shoe{  size:10, style:String::from("sneker")  },
            Shoe{  size:13, style:String::from("sandal")  },
            Shoe{  size:10, style:String::from("boot")  },
        ];

    println!("{:?}", shoes_in_size(shoes, 10));    

}

#[derive(Debug)]
struct Shoe{
    size: u32,
    style: String
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe>{
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

