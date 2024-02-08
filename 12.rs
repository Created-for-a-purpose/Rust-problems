// Find the maximum subarray sum in Rust
use std::cmp;

fn max_subarray_sum(array: &[i32]) -> i32 {
    let mut max_ending_here = 0;
    let mut max_so_far = std::i32::MIN;

    for n in array {
        max_ending_here = cmp::max(0, max_ending_here+n);
        max_so_far = cmp::max(max_ending_here, max_so_far);
    }

    max_so_far
}

fn main(){
    let array = [-4, -3, -1, 1, 2, -3, -2, 4, 5, 3];

    let max_sum = max_subarray_sum(&array);
    println!("The maximum subarray sum is {}", max_sum);
}