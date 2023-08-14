
fn main() {

  let arr:[i32;6] = [0, 1, 2, 5, 7, 9];
  let target = 7;
  
  let ans = bs(&arr, target);


  if Option::None != ans{
    println!("The Number {} Found at the inded: {}",arr[ans.unwrap() as usize],  ans.unwrap() );
  }else{
    println!("Number not found in the array!");
  }

}


fn bs(arr:&[i32], target: i32)-> Option::<i32>{

    let mut low = 0 as i8;
    let mut high = (arr.len() -1) as i8;

  while low <= high {

    let mid = ((high - low) / 2) + low; 

    if arr[mid as usize] == target {
        return Some(mid as i32)
    } else if arr[mid as usize] < target {
      low = mid + 1;
    } else {

      // if mid as i32 -1 < 0 { break}
      // if mid == 0 {break}
      
      high = mid - 1 ;

    }
  }
  
  None
}



