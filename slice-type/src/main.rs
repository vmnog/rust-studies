fn main() {
    let s1 = String::from("hello world");
    
    let word = first_word(&s1);

    println!("s1: {}", &s1);
    println!("word: {}", &word);
    
    let a = [1, 2, 3, 4, 5];

    let slice = &a[1..3];

    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}