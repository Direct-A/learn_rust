fn main() {
    let mut name_itor = "Song Yishai".split_ascii_whitespace();
    let a = name_itor.next().unwrap()[..1].to_ascii_uppercase();
    let b = name_itor.next().unwrap()[..1].to_ascii_uppercase();
    println!("{}", a +"."+&b);
}
