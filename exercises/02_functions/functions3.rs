use std::u8;

fn call_me(num: u8) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn main() {
    // TODO: Fix the function call.
    call_me(u8::MAX); // should be 255
    println!("{}", i8::MAX); // should be 127 given one bit is for positive/negative flag
}
