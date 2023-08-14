#[derive(Debug, Copy, Clone)]
struct PointMixed<T, U>{
    x: T,
    y: U
}

// impl<T:Clone, U:Clone> PointMixed<T, U>{     // OR
impl<T, U> PointMixed<T, U>
where
    T: Clone, // + Copy,
    U: Clone,
{
        
    
// not working  ###################################

    fn mixup(&self, another:&PointMixed<U, T>) -> PointMixed<T, T> {

        PointMixed{ 
            x: self.x.clone(),
            y: another.y.clone()
        }
    }
}

fn main(){
    let point_fi = PointMixed{ x:5.00, y:2};    // var1 
    let point_if = PointMixed{x:5, y:2.00};     // var2

    let point_ff = point_fi.mixup(&point_if);   
    let point_ii = point_if.mixup(&point_fi); 
     

    println!("{:?}", point_fi);     // var1
    println!("{:?}", point_if);     // var2

    println!("{:?}", point_ff);     // mixed
    println!("{:?}", point_ii);     // mixed
} 