fn main() {
    // let goods = vec![0; 5];
    let mut goods = Vec::new();
    goods.push(5);
    goods.push(6);
    goods.push(7);
    goods.push(8);

    println!("a new vector {:?}", goods);
    println!("the last member of vector {}", goods[goods.len()-1]);
    goods.get(index)
}
