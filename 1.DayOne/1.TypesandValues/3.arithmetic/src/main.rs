// fn interproduct(a: i32, b: i32, c: i32) -> i16 { //you can convert an `i32` to an `i16` and panic if the converted value doesn't fit
//     return a * b + b * c + c * a;
// }

fn interproduct(a: i32, b: i32, c: i32) -> i32 {
    return a * b + b * c + c * a;
}

fn main() {
    println!("resultado: {}", interproduct(120, 100, 248));
}