/*
根据累积体积 m，计算层数

每层均是立方体，边长 n
层与层之间边长差 1
*/
fn main() {
}

fn find_nb(m: u64) -> i32 {
    let mut vol_sum = m;
    let mut n:u64 = 1;
    loop {
        let tmp = n.pow(3_u32);
        vol_sum -= tmp;
        match vol_sum {
            0 => return n as i32,
            _x if _x > tmp => n += 1,
            _ => return -1,
        };
    }
}

fn testing(n: u64, exp: i32) -> () {
    assert_eq!(find_nb(n), exp);
}

#[test]
fn basics_find_nb() {
    testing(4183059834009, 2022);
    testing(24723578342962, -1);
    testing(135440716410000, 4824);
    testing(40539911473216, 3568);
    testing(26825883955641, 3218);
}
