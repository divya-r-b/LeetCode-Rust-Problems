// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
use std::collections::VecDeque;

impl Solution {
    pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut dummy_head = Some(Box::new(ListNode::new(0)));
        let mut current = &mut dummy_head;
        let mut carry = 0;
        
        let (mut p1, mut p2) = (l1, l2);
        
        while p1.is_some() || p2.is_some() {
            let mut sum = carry;
            
            if let Some(node) = p1 {
                sum += node.val;
                p1 = node.next;
            }
            
            if let Some(node) = p2 {
                sum += node.val;
                p2 = node.next;
            }
            
            carry = sum / 10;
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(sum % 10)));
            current = &mut current.as_mut().unwrap().next;
        }
        
        if carry > 0 {
            current.as_mut().unwrap().next = Some(Box::new(ListNode::new(carry)));
        }
        
        dummy_head.unwrap().next
    }
}
