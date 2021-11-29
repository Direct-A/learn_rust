fn main() {
    let mut s = String::from("hello");
    let r2 = &mut s;
    // println!("{}", r2);

    {
        let r1 = &mut s;
        println!("{}", r1)
    } // r1 在这里离开了作用域，所以我们完全可以创建一个新的引用

    let r3 = &mut s;
    println!("{} {}", r2,r3);
}
