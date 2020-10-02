use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: hex NUMBER");
        return;
    }

    println!(
        "{:X}",
        args[0].trim().replace(',', "").parse::<usize>().unwrap()
    );
}
