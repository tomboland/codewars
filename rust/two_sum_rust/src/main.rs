use itertools::Itertools;
fn two_sum(numbers: &[i32], target: i32) -> (usize, usize) {
    match numbers
        .iter()
        .enumerate()
        .combinations(2)
        .find(|item| {
             item[0].1 + item[1].1 == target
        })
        .as_deref() {
            Some([x, y]) => (x.0, y.0),
            _ => (0, 0)
    }
}

#[test]
fn sample() {
    do_test(&[1, 2, 3], 4);
    do_test(&[1234, 5678, 9012], 14690);
    do_test(&[2, 2, 3], 4);
}

fn do_test(nums: &[i32], sum: i32) {
    let len = nums.len();
    let user_tuple = two_sum(nums, sum);
    assert!(
        user_tuple.0 < len && user_tuple.1 < len,
        "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple has an index out of bounds",
        nums, sum, user_tuple 
    );
    assert!(
        user_tuple.0 != user_tuple.1,
        "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nresult tuple must have two different indices",
        nums, sum, user_tuple 
    );
    let num1 = nums[user_tuple.0];
    let num2 = nums[user_tuple.1];
    let user_sum = num1 + num2;
    assert!(
        user_sum == sum,
        "\nnumbers: {:?}\ntarget: {}\nresult: {:?}\nnumber as index {}: {}\nnumber as index {}: {}\nsum of the two numbers: {}\nsum of the two numbers did not equal target",
        nums, sum, user_tuple, user_tuple.0, num1, user_tuple.1, num2, user_sum
    )
}