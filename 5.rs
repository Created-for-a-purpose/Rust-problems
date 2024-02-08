// Given a sorted array of integers, implement a function that returns the median of the array.

fn find_median(array: &[i32]) -> f64 {
    
    if array.len()%2 == 0 {
        let mid_i = array.len()/2;
        let mid_1 = array[mid_i];
        let mid_2 = array[mid_i - 1];
        (mid_1 + mid_2) as f64/ 2.0
    } else {
        let mid = array[array.len()/2];
        mid as f64
    }

}

fn main() {
    let sorted_array = [1,1,2,2,2,3,4,4,4,4,5,6];
    // let sorted_array = [1,2,3,4,5];

    let median: f64 = find_median(&sorted_array);
    println!("Median of array is {}", median);
}