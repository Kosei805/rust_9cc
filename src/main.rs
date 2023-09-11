use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        panic!("引数の個数が正しくありません:{}",args.len());
    }

    println!(".intel_syntax noprefix");
    println!(".global main");
    println!("main:");
    println!("\tmov rax, {}",&args[1]);
    println!("\tret");
}
