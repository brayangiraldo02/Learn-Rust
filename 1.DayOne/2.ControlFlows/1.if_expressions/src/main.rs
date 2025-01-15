fn if_basic() {
  let x = 10;
  if x == 0 {
    println!("zero!");
  } else if x < 100 {
    println!("biggish");
  } else {
    println!("huge");
  }
}

fn if_small() {
  let x = 10;
  let size = if x < 20 { "small" } else { "large" };
  println!("number size: {}", size);
}

fn main() {
  if_basic();
  if_small();
}