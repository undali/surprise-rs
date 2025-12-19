// Copy & paste this to the editor in https://play.rust-lang.org , then press run button
fn func1() {
    println!("func1 is called.");
}

fn func2() {
    println!("func2 is called.");
}

fn main() {
    let function_storage = [func1, func2, func1];

    for f in function_storage {
        f();
    }
}

