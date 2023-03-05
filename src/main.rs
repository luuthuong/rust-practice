use two_sum::two_sum::TwoSum;
use two_sum::two_sum::TwoSumTrait;
fn main() {
    let list_item = [1,2,3,4,5,12];
    let target = 9;
    let two_sum = TwoSum::new(&list_item, target);
    println!("{:?}", two_sum);
    let result = two_sum.calc().unwrap_or_default();
    println!("{:?}", result);
}
