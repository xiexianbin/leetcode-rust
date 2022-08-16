#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}



pub struct Solution {}

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        for (i, x) in nums.iter().enumerate() {
            for (j, y) in nums.iter().enumerate() {
                if i != j {
                    if target == x + y {
                        let mut v: Vec<i32> = Vec::new();
                        v.push(i as i32);
                        v.push(j as i32);
                        return v
                    }
                }
            }
        }

        return vec![0, 0];        
    }
}

fn main() {
    let nums = vec![2,7,11,15];
    let target = 9;
    println!("{:?}", Solution::two_sum(nums, target));
}
