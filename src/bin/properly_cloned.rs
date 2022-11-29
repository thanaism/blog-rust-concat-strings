fn main() {
    let text = String::from("yes");
    let text_extended = text.clone() + ", fall in love"; // cloneでコピーを渡す
    println!("{}", text); // 所有権が残っているので使用できる
    println!("{}", text_extended);
}
