fn main() {
    let mut v = vec![1,2,3,4,5];
    let first = v[0];
    let mut second = "xxx".to_string();
    second.push_str("ooo");
    let mut third = String::from("xxxxxxx");
    third.push_str(&second);
    v.push(6);
    println!("first: {}", first);
    println!("v[0]: {}", v[0]);
    println!("second: {}", second);
    println!("third: {}", third);
}
