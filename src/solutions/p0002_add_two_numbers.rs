#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut head: &mut ListNode = &mut dummy;
        let mut l1 = l1.as_ref();
        let mut l2 = l2.as_ref();
        let mut carry = 0;

        while l1.is_some() || l2.is_some() {
            let mut sum = carry;
            if let Some(n1) = l1 {
                sum += n1.val;
                l1 = n1.next.as_ref();
            }
            if let Some(n2) = l2 {
                sum += n2.val;
                l2 = n2.next.as_ref();
            }
            carry = sum / 10;
            head.next = Some(Box::new(ListNode::new(sum % 10)));
            head = head.next.as_mut().unwrap();
        }

        if carry > 0 {
            head.next = Some(Box::new(ListNode::new(1)));
        }

        dummy.next
    }
}
