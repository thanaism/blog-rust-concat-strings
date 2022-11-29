fn main() {
    let text = String::from("yes");
    let text_extended = text + ", fall in love"; // ここでtextの所有権がムーブされる
    println!("{}", text); // 所有権がないので使用できない
    println!("{}", text_extended);
}
