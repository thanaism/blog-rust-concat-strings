fn main() {
    let text = String::from("yes");

    // Stringに対する加算演算子は内部的にstd::ops::Add::addが呼び出されている
    let text_extended_by_plus = text.clone() + ", fall in love!";
    let text_extended_by_add = std::ops::Add::add(text.clone(), ", fall in love!");

    println!("{}", text_extended_by_plus);
    println!("{}", text_extended_by_add);
}
