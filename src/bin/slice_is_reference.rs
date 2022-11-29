fn main() {
    let mut digits = String::from("0123456");
    let slice = &digits[..4]; // ここで参照を取っているので
    println!("{}", digits);
    digits.clear(); // 書き換えはコンパイルエラーになる
    println!("{}", slice);
}
