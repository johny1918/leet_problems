pub mod intro;
use intro::*;

pub fn intro_execution_workspace() {
    let nums = vec![1,2,3,4];
    let ret = sum_one_array(nums);

    assert_eq!(3, ret[1]);

    let nums = vec!{
        vec![1,2,3],
        vec![3,2,1],
    };
    let n = maximum_wealth(nums);
    assert_eq!(n, 6);

    let nums = vec!{
        vec![1,5],
        vec![7,3],
        vec![3,5],
    };

    let n = maximum_wealth(nums);
    assert_eq!(n, 10);


    fizz_buzz(3);

    let n = number_of_steps(14);
    assert_eq!(6, n);

    // [1, 2, 3, 4, 5] -> middle is 3
    let mut n5 = ListNode::new(5);
    let mut n4 = ListNode::new(4);
    let mut n3 = ListNode::new(3);
    let mut n2 = ListNode::new(2);
    let mut n1 = ListNode::new(1);
    n4.next = Some(Box::new(n5));
    n3.next = Some(Box::new(n4));
    n2.next = Some(Box::new(n3));
    n1.next = Some(Box::new(n2));

    let result = middle_node(Some(Box::new(n1)));
    assert_eq!(result.as_ref().unwrap().val, 3);

    // [1, 2, 3, 4, 5, 6] -> middle is 4 (second middle)
    let mut n6 = ListNode::new(6);
    let mut n5 = ListNode::new(5);
    let mut n4 = ListNode::new(4);
    let mut n3 = ListNode::new(3);
    let mut n2 = ListNode::new(2);
    let mut n1 = ListNode::new(1);
    n5.next = Some(Box::new(n6));
    n4.next = Some(Box::new(n5));
    n3.next = Some(Box::new(n4));
    n2.next = Some(Box::new(n3));
    n1.next = Some(Box::new(n2));

    let result = middle_node(Some(Box::new(n1)));
    assert_eq!(result.as_ref().unwrap().val, 4);

    let ransom_note = "a".to_string();
    let  magazine = "b".to_string();
    let result = can_construct(ransom_note, magazine);
    assert_eq!(false, result);

    let ransom_note = "aa".to_string();
    let  magazine = "aab".to_string();
    let result = can_construct(ransom_note, magazine);
    assert_eq!(true, result);
}