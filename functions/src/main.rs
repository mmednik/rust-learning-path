fn goodbye(message: &str) {
    println!("{}", message);
}

fn main() {
    let formal = "Good bye.";
    let casual = "See you later!";
    goodbye(formal);
    goodbye(casual);
}