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
impl Solution {
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        if head == None {
            return None;
        }

        let mut a = head.clone().unwrap();
        let bt = a.clone().next;
        a.next = None;
        if bt == None {
            return head;
        }
        let mut b = bt.unwrap();
        while let Some(c) = b.next {
            b.next = Some(a);
            a = b;
            b = c;
        }
        b.next = Some(a);
        Some(b)
    }
}
