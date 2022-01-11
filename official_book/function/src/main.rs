/// test
fn main() {
  println!("Hello, world!");

  another_function();
}

/**
 * annotation for ra
 */
fn another_function() {
  /*!
   * test for ra
   */
  // println!("Another function.");
  for i in 0..10 {
    println!("{}", i)
  }
}
