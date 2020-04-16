use std::io;
use std::env;

fn main() {
    println!("1) Unpack\n");

    let mut answer_method = String::new();
    io::stdin().read_line(&mut answer_method)
        .expect("Failed to read");

    let answer_method: u32 = answer_method.trim().parse()
        .expect("Need a number");

    match answer_method {
        1 => SearchPack(),
        _ => println!("Please, choose from list"),
    }
}

fn SearchPack() {
    let args = ArgumentCollector();
    println!("{:?}", args);
}

fn ArgumentCollector() -> Vec<String> {
    let mut all_args: Vec<String> = env::args().collect();
    all_args.remove(0);
    return all_args;
}