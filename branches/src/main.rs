fn main() {
    let number = 12;

    if number % 4 == 0 {
        println!("{number} is divised by 4");
    } else if number % 3 == 0 {
        println!("{number} is divised by 3");
    } else if number % 2 == 0 {
        println!("{number} is divised by 2");
    } else {
        println!("{number} is not divised by 4, 3 or 2");
    }
}
