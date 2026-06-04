
struct Solution;
use std::collections::HashMap;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut nums_map : HashMap<i32,i32> = HashMap::new();
        // 3,2,5,9 target 7  temp = 4
        for(index_nums,&num) in nums.iter().enumerate(){
            let temp = target - num;
            if nums_map.contains_key(&temp) {
                println!(" {}, {} " ,*nums_map.get(&temp).unwrap() ,index_nums as i32);
                return vec![*nums_map.get(&temp).unwrap(),index_nums as i32, ];

            }
            nums_map.insert(num, index_nums as i32);
        }
        return vec![];


     
    }

    
}

