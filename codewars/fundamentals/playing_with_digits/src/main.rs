fn main() {
}

fn dig_pow(n: i64, p: i32) -> i64 {
    let mut p_inner = p;
    let n_len = n.to_string().len() as u32;
    let mut v = Vec::new();
    let mut pow_sum: i64 = 0;
    for i in (0..n_len).rev() {
        let tmp = n / (10_i64.pow(i));
        let tmp1 = n / (10_i64.pow(i + 1));
        let n_single = tmp - tmp1*10;
        v.push(n_single);
        pow_sum += n_single.pow(p_inner as u32);
        p_inner += 1;
    }
    let k = pow_sum as f64 / n as f64;
    let j = k - (k as i64) as f64;
    let k = (k - j) as i64;
    match j == 0 as f64 {
        true => k,
        false => -1,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn dotest(n: i64, p: i32, exp: i64) -> () {
        println!(" n: {:?};", n);
        println!("p: {:?};", p);
        let ans = dig_pow(n, p);
        println!(" actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!(" {};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic_tests() {
        dotest(89, 1, 1);
        dotest(92, 1, -1);
        dotest(46288, 3, 51);
    }
}
