// Implement a function that returns the kth smallest element in a given array.

fn find_k_smallest(mut k: u32, array: &mut[i32]) -> i32 {

    array.sort();

    let mut i=0;

    let mut result = array[0];
    k-=1;
    while i<array.len() && k>0 {
        if array[i] != result {
            result = array[i];
            k -= 1;
        }
        i+=1;
    }

    result
}

fn main(){
    let mut array = [2,4,5,3,7,3,7,4,8,4,6,4,5,3,7,3,7,4,8,4,6];
    let mut k: u32 = 4;
    let k_smallest: i32 = find_k_smallest(k, &mut array);
    println!("The {}th smallest element is {}", k, k_smallest);
}