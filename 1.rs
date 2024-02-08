// Implement a function that checks whether a given string is a palindrome or not.

fn is_palindrome(string: &str) -> bool {
    let reversed_string: String = string.chars().rev().collect();
    string == reversed_string
}

fn main() {
    let strings = vec!["hello", "mom", "level", "madam", "radar", "good"];

    for string in strings{
        let isPalindrome: bool = is_palindrome(string);
        println!("{} is a palindrome: {}", string, isPalindrome);
    }

}