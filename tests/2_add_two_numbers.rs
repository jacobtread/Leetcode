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

pub fn add_two_numbers(
    mut l1: Option<Box<ListNode>>,
    mut l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut out = None;

    let mut next: Option<&mut Option<Box<ListNode>>> = None;

    let mut carry = 0;

    loop {
        let (a, b) = match (l1, l2) {
            (Some(a), Some(b)) => {
                l1 = a.next;
                l2 = b.next;

                (a.val, b.val)
            }
            (Some(a), None) | (None, Some(a)) => {
                l1 = a.next;
                l2 = None;

                (a.val, 0)
            }
            _ => break,
        };

        let amount = a + b + carry;
        if amount > 9 {
            carry = 1
        } else {
            carry = 0
        };
        let amount = amount % 10;

        if let Some(n1) = next {
            let v = n1.insert(Box::new(ListNode::new(amount)));
            next = Some(&mut v.next)
        } else {
            let v = out.insert(Box::new(ListNode::new(amount)));
            next = Some(&mut v.next)
        }
    }

    if carry > 0 {
        if let Some(next) = next {
            *next = Some(Box::new(ListNode::new(carry)));
        }
    }

    out
}

#[cfg(test)]
mod test {

    use crate::ListNode;

    use super::add_two_numbers;

    // Converts the provided list into linked nodes
    fn create_list(vec: Vec<i32>) -> Option<Box<ListNode>> {
        let length = vec.len();
        let mut prev = None;
        for i in (0..length).rev() {
            let item = vec[i];

            let mut node = Box::new(ListNode::new(item));
            node.next = prev.take();
            prev = Some(node);
        }

        prev
    }

    fn expect_list(item: Option<Box<ListNode>>, expected: Vec<i32>) {
        let mut out = item;
        for item in expected {
            let i2 = out.unwrap();

            assert_eq!(item, i2.val);
            out = i2.next;
        }
    }

    #[test]
    fn example_1() {
        let l1 = create_list(vec![2, 4, 3]);
        let l2 = create_list(vec![5, 6, 4]);
        let out = add_two_numbers(l1, l2);
        expect_list(out, vec![7, 0, 8])
    }

    #[test]
    fn example_2() {
        let l1 = create_list(vec![0]);
        let l2 = create_list(vec![0]);
        let out = add_two_numbers(l1, l2);
        expect_list(out, vec![])
    }

    #[test]
    fn example_3() {
        let l1 = create_list(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = create_list(vec![9, 9, 9, 9]);
        let out = add_two_numbers(l1, l2);
        expect_list(out, vec![8, 9, 9, 9, 0, 0, 0, 1])
    }
}
