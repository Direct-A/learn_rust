fn main() {
    println!("🌲 The Twelve Days of Christmas\n");
    let mut counter: i32 = 2;
    while counter > 0 {
        println!("Have yourself a merry little Christmas,");
        if counter == 2 {
            println!("Let your heart be light");
        } else {
            println!("Make the Yule-tide gay,");
        }
        println!("From now on,");
        let lyric_freg = if counter == 2 {
            "out of sight"
        } else {
            "miles away."
        };
        println!("Our troubles will be {}\n", lyric_freg);
        counter -= 1;
    }
    println!(
        "Here we are as in olden days,
Happy golden days of yore.
Faithful friends who are dear to us
Gather near to us once more.

Through the years
We all will be together,
If the Fates allow
Hang a shining star upon the highest bough.
And have yourself A merry little Christmas now.
"
    );
}
