// Reverse a string in Rust

fn reverse(string: String) -> String {
    let mut reversed_string: String = String::new();

    let characters: Vec<char> = string.chars().rev().collect();
    for c in characters {
        reversed_string.push(c);
    }

    // Another way
    
    // for i in (0..string.len()).rev() {
    //     reversed_string.push(string.chars().nth(i).unwrap());
    // }

    reversed_string
}

fn main() {
    let string: String = String::from("Reverse me");
    let reversed_string = reverse(string);

    println!("The reversed string is --> {}", reversed_string);

}