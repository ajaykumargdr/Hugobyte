// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
  pub val: i32,
  pub next: Option<Box<ListNode>>
}

impl ListNode {
  #[inline]
  fn new(val: i32) -> Self {
    ListNode {
      next: None,
      val
    }
  }
}


fn main(){
    swap
}

pub fn swap_pairs(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
    
    let mut root = &mut head;

    while let Some(node) = root{

        println!("{}", node.val);
        
        if let Some(next) = node.next{

        println!("{}", next.val);

        // let temp = node.val;
        // node.val = next.val;
        // next.val = temp; 

        root = &mut next.next;

        }else {
        break
        }
        
        
    }

    head
}
