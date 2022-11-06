fn goodbye(message: &str) {
    println!("{}", message);
}

fn divide_by_5(num: u32) -> u32 {
    num / 5
}

fn main() {
    let formal = "Good bye.";
    let casual = "See you later!";
    goodbye(formal);
    goodbye(casual);

    let num = 25;
    println!("{} divided by 5 = {}", num, divide_by_5(num));
}
