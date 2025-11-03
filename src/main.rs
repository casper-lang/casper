use std::env;

use casper::pseudo_main;

fn main() {
    let mut args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        args.remove(0); // remove name from arg list
    }

    let args = args;

    pseudo_main(args);    
}
