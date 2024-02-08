// Given a sorted array of integers, implement a function that returns the index of the first occurrence of a given number.

fn find_first_occurence(array: &[i32], num: i32) -> Option<usize> {   // O(n)
    let mut i=0;
  while i<array.len() {
    if array[i] == num {
        return Some(i);
    }
    i+=1;
  }

  return None;
}

fn find_first_occurence_bs(array: &[i32], num: i32) -> Option<usize> { // O(logn)  Binary search
let mut low = 0;
let mut high = array.len();
let mut index = None;

while low<=high {
  let mid = low + (high-low)/2;

  if array[mid] == num {
    index = Some(mid);
    high = mid - 1;
  }
  else if array[mid]<num {
    low = mid+1;
  }
  else {
    high = mid -1;
  }
}

  index
}

fn main(){
    let sorted_array = [1,1,2,2,2,3,4,4,4,4,5,6];

    let num: i32 = 4;
    let index_of_num = find_first_occurence(&sorted_array, num);
    match index_of_num {
        Some(index_of_num) => println!("First occurence of {} is {}", num, index_of_num),
        None => println!("{} not found", num)
    }
    
    let index_of_num = find_first_occurence_bs(&sorted_array, num);
    match index_of_num {
      Some(index_of_num) => println!("First occurence of {} is {}", num, index_of_num),
      None => println!("{} not found", num)
  }
}