// 显式引用，非prelude模块内均需要这部操作
use std::io;
// trait??
use rand::Rng;
// 枚举类型??
use std::cmp::Ordering;

// 主函数
fn main() {
    // 宏
    // 打印内容并换行
    println!("##### 猜数 #####");
    // 整数类型，默认i32，存在i32 u32 i64??
    let s_num = rand::thread_rng().gen_range(1..101);
    println!("随机数：{}", s_num);

    loop {
        println!("猜个数：");
        // 声明可变变量，并实例化一个字符串
        let mut guess = String::new();
        // 返回的Result枚举类型，需要处理
        // expect用以处理该枚举类型。
        // 模块的函数的方法？？？
        io::stdin().read_line(&mut guess).expect("ERROR");
        // 类型转换
        // 同名变量隐藏原变量，shadow，仅在类型转换时
        // trim去左右空格和回车
        // var: 类型 显式声明变量类型
        // u32 无符号整数类型，内置类型
        // let guess: u32 = guess.trim().parse().expect("Transform ERROR");
        // mach 应对Result变体，处理报错的惯用手段
        // _ 通配符
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("你猜的数：{}", guess);
        // 拥有多个匹配分支的模式
        match guess.cmp(&s_num) {
            // arms
            Ordering::Greater => println!("Too Big."),
            Ordering::Less => println!("Too Small."),
            Ordering::Equal => {
                println!("RIGHT!!!");
                break;
            }
        }
    }
}
