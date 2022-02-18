fn main() {
    let n = 9;
    let diamond: String = (1..=n)
        .chain((1..n).rev())
        .step_by(2)
        .map(|i| format!("{}{}\n", " ".repeat((n - i) / 2), "*".repeat(i)))
        .collect();
    let d = (1..=n)
        .chain((1..n).rev())
        .step_by(2);
    println!("d: {:#?}", d);
    println!("{}", diamond);
}

/*

    *
   ***
  *****
 *******
*********
 *******
  *****
   ***
    *

*/
fn print(n: i32) -> Option<String> {
    if let true = (n > 0 && n % 2 == 1) {
        let mut str = String::new();
        for ele in 0..n {
            let i = n / 2 - ele;
            let j = if i >= 0 { i as usize } else { -i as usize };
            let k = 2 * ele as usize;
            str += &" ".repeat(j);
            if ele <= n / 2 {
                str += &"*".repeat(k + 1);
            } else {
                str += &"*".repeat(k - j * 4 + 1);
            }
            str += "\n"
        }
        Some(str)
    } else {
        None
    }
}

#[test]
fn basic_test() {
    assert_eq!(print(3), Some(" *\n***\n *\n".to_string()));
    assert_eq!(print(5), Some("  *\n ***\n*****\n ***\n  *\n".to_string()));
    assert_eq!(print(-3), None);
    assert_eq!(print(2), None);
    assert_eq!(print(0), None);
    assert_eq!(print(1), Some("*\n".to_string()));
}
