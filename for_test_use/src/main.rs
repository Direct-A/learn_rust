fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let d = v.drain(1 .. 8);
    println!("{:?}", d);
}
