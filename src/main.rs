use std::env;

mod parser;
mod Parser;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    #[cfg(debug_assertions)]
    {
        //println!("Debugger is attached (Debug mode)");
        args = vec!["-f".to_string(), "TEST".to_string(), "-d".to_string(), "-s".to_string(), "101".to_string()];
    }

    parser::ParseArgs(args);
    //if let Some(first_arg) = args.get(1) {
    //    println!("{}", first_arg);
    //} else {
    //    println!("No arguments provided");
    //}
}
