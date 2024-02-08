// Merge two sorted arrays in Rust

fn merge(array_1: &mut[i32], array_2: &mut[i32]) -> Vec<i32>{
    let mut i=0;
    let mut j=0; 

    let mut merged: Vec<i32> = Vec::new();

    while i<array_1.len() && j<array_2.len() {
        if array_1[i]<array_2[j]{
            merged.push(array_1[i]);
            i+=1;
        }
        else {
            merged.push(array_2[j]);
            j+=1;
        }
    }

    while i<array_1.len() {
        merged.push(array_1[i]);
        i+=1;
    }

    while j<array_2.len() {
        merged.push(array_2[j]);
        j+=1;
    }

    merged

}

fn main () {
    let mut array_1 = [1,4,7,9,13,18];
    let mut array_2 = [2,6,8,12,15,17,19];

    let merged_array: Vec<i32> = merge(&mut array_1, &mut array_2);
    println!("The merged array: {:?}", merged_array);
}