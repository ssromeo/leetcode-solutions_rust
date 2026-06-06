    struct Solution;

    #[derive(PartialEq, Eq, Clone, Debug)]
    pub struct ListNode{
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
    /*  Input: l1 = [2,4,3], l2 = [5,6,4]
    Output: [7,0,8]
    Explanation: 342 + 465 = 807. */
    impl Solution {
        pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>> {

            let mut temp_l1 : Option<Box<ListNode>> = l1.clone();
            let mut temp_l2 : Option<Box<ListNode>> = l2.clone();
            let mut final_vec : Option<Box<ListNode>> = Some(Box::new(ListNode { val: (0), next: None }));
            let mut l1_val : i32 = 0;
            let mut l2_val : i32 = 0;
            let mut retenue : i32 = 0;
            let mut courant = &mut final_vec;
            while temp_l1.is_some() || temp_l2.is_some() || retenue > 0 {

        

                if let Some(node1) = temp_l1{

                    l1_val = node1.val;
                    temp_l1 = node1.next;
                }else{
                    l1_val = 0;
                }

                if let Some(node2) = temp_l2{

                    l2_val = node2.val;
                    temp_l2 = node2.next;
                }else{
                    l2_val = 0;
                }

                let somme : i32 = l1_val + l2_val + retenue;

                let reste : i32 = somme % 10;
                courant.as_mut().unwrap().next = Some(Box::new(ListNode{val:reste,next:None}));;

    

                if somme >= 10 {
                    retenue = 1;

                }else{
                    retenue = 0
                }

                courant = &mut courant.as_mut().unwrap().next;
                
                
                

            }
            
            return final_vec.as_mut().unwrap().next.take();
        }
    }