use std::collections::HashMap;

#[derive(Debug)]
pub struct TwoSum {
    pub numbs: Vec<i32>,
    pub target: i32,
}

pub trait TwoSumTrait {
    fn calc(&self) -> Option<[usize; 2]>;
    fn print(&self);
    fn new(numbs:&[i32], target: i32) -> TwoSum;
}

impl TwoSumTrait for TwoSum {
    fn calc(&self) -> Option<[usize; 2]> {
        let mut map = HashMap::new();
        for (i, &num) in self.numbs.iter().enumerate() {
            let complement = self.target - num;
            if  map.contains_key(&complement) {
                let value = map.get(&complement).unwrap();
                return Some([*value, i]);
            }
            map.insert(num, i);
        }
        None
    }

    fn print(&self) {
        println!("{:?} - target: {}", self.numbs, self.target);
    }

    fn new( numbs: &[i32], target: i32) -> TwoSum {
        TwoSum{
            target: target,
            numbs: numbs.to_vec()
        }
    }
}
