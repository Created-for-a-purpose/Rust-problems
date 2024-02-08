// Implement a function that finds the longest common prefix of a given set of strings.

fn find_lcp(set: &[String]) -> String {
    let mut prefix: String = String::new();
    let mut i = 0;

    while i<set[0].len() {
        let c: Option<char> = set[0].chars().nth(i);
        for s in &set[1..] {
            if i>=s.len() || s.chars().nth(i) != c {
                return prefix;
            }
             
        }
        prefix.push(c.unwrap());
        i+=1;
    }

    prefix
}

fn main (){
    let set = vec![String::from("abcb"), String::from("abcb"), String::from("abcbc"), String::from("abcbaabc")];

    let lcp = find_lcp(&set);
    println!("Longest common prefix is {}", lcp);
}