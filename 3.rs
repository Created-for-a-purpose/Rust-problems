// Given a string of words, implement a function that returns the shortest word in the string.
fn find_shortest_word(string: &str) -> &str {
    let words: Vec<&str> = string.split_whitespace().collect();

    let mut shortest_word = words[0];
    for word in words.iter(){
        if word.len() < shortest_word.len() {
            shortest_word = word;
        }
    }

    return shortest_word
}

fn main() {
 let string: &str = "Given a string of words";
 let shortest_word = find_shortest_word(string);
 println!("The shortest word is {:?}", shortest_word);
}