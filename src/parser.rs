use std::collections::HashMap;

pub enum KeyFlags{
    InputFolder = 1,
    SearchRegex = 2,
    Delete = 3,
}

pub fn ParseArgs(args: Vec<String>) -> HashMap<KeyFlags, String>{
    let mut flags = HashMap::new();

    let mut iter = args.iter().enumerate();

    loop {
        match iter.next() {
            Some((i, arg)) if arg == "-f" => {
                if let Some((_, value)) = iter.next() {
                    println!("{}", value);
                }
            },
            Some((i, arg)) if arg == "-s" => {
                if let Some((_, value)) = iter.next() {
                    println!("{}", value);
                }
            },
            Some((i, arg)) if arg == "-d" => {
                println!("DELETE = True");
            },
            Some((_, _)) => {
                println!("out of Range");
            },
            None => break,
        }
    }

    // for mut i in 0..args.len(){
    //     match args[i].as_str(){
    //         "-f"=>{
    //             println!("FLAG VALUE={}", args[i + 1].as_str());
    //             i = i + 1;
    //         },
    //         _=>println!("{}", args[i].as_str()),
    //        }
    // }

    return flags;
}