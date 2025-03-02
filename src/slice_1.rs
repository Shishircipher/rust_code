// slice is a kind of reference, so it does not have ownership.
//

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);
    println!("{}",word);
    s.clear(); // this empties the String, making it equal to ""

    // `word` still has the value `5` here, but `s` no longer has any content
    // that we could meaningfully use with the value `5`, so `word` is now
    // totally invalid!
    let mut t = String::from("hello world2");
    let word2 = first_word_1(&t);
    println!("{}",word2);
}


//We’re returning a usize on its own, but it’s only a meaningful number in the context of the &String. In other words, because it’s a separate value from the String, there’s no guarantee that it will still be valid in the future.

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in  bytes.iter().enumerate() {
        if item == b' '{
            return i;
        }
    }
   s.len()
}

fn first_word_1(s : &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
