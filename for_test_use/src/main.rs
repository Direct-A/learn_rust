fn main() {
    // let s = String::from("hello world");
    // let f = || {
    //     println!("S: {}", s);
    // };
    // f();
    // println!("S: {}", s);
    // println!("{}", s.find("world").unwrap());
    // println!("{}", s.contains("world"));
    // let n = 0;
    // match n {
    //     0 => println!("{} is zero",n),
    //     x if x < 3 => println!("{}", x),
    //     _ => println!("{}", n),
    // }
    // let v = vec![15, 35, 25, 21, 22, 30, 30];
    // let v_b = vec![15, 35, 25];
    // v.dedup_by_key(|x| *x % 10);
    // assert_eq!(v, [15, 21, 22, 30]);
    // v.dedup_by_key(|x| *x / 10);
    // assert_eq!(v, [15, 35, 25, 30]);
    // v.dedup_by_key(|x| *x);
    // assert_eq!(v, [15, 35, 25, 21, 22, 30]);
    // fn array_diff<T: PartialEq>(mut a: Vec<T>, b: Vec<T>) -> Vec<T> {
    //     a.drain_filter(|x| b.contains(&x)).collect::<Vec<_>>()
    // }
    // assert_eq!(array_diff(v, v_b), [15, 35, 25, 21, 22, 30]);
    // let s = "aAbBcC";
    // fn disemvowel(s: &str) -> String {
    //     s.chars().filter(|&x| !"aAeEiIoOuU".contains(x)).collect()
    // }
    // assert_eq!(disemvowel(s), "bBcC");
    // let a: [u32; 5] = [1, 0, 0, 1, 1];
    // // let b = [a.len()..0];
    // // 同时迭代
    // fn binary_slice_to_number(slice: &[u32]) -> u32 {
    //     let mut sum: u32 = 0;
    //     for (i, &j) in slice.iter().enumerate() {
    //         sum += j * 2_u32.pow((slice.len() - i -1) as u32);
    //         println!("i: {}\tj: {}", slice.len() - i -1, j)
    //     }
    //     sum
    // }
    // assert_eq!(binary_slice_to_number(&a), 19);
    // println!("float: {}", 5f64 / 2f64);
    // println!("int: {}", 5 / 2);
    // println!("int: {}", 5 % 2);
    // println!("length: {}", "fvck off ".len());
    // println!("length: {}", 12153.to_string().len());
    for (i, j) in 211354_u64.to_string().chars().enumerate() {
        println!("{}{} +", j, "0".repeat(211354_u64.to_string().len() - i));
    }
}

fn expanded_form(n: u64) -> String {
    let mut s = String::new();
    for (i, j) in n.to_string().chars().enumerate() {
        if j != '0' {
            s += &j.to_string();
            s += &"0".repeat(n.to_string().len() - i - 1);
            s += " + ";
        }
    }
    s.trim_end_matches(&" + ").to_string()
}

#[test]
fn test() {
    assert_eq!(expanded_form(2890077), "2000000 + 800000 + 90000 + 70 + 7");
    assert_eq!(expanded_form(270000), "200000 + 70000");
}
