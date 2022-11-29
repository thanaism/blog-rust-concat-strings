fn main() {
    let text = "yes";
    // cannot add `&str` to `&str`
    let text_extended = text + ", fall in love";
    println!("{}", text);
    println!("{}", text_extended);
}
