fn main() {
    let s = "नमस्ते";

    let mut j = 1;
    for i in s.bytes() {
        println!("{}:\t{}", j, i);
        j += 1;
    }

    let mut j = 1;
    for i in s.chars() {
        println!("{}:\t{}", j, i);
        j += 1;
    }
}
