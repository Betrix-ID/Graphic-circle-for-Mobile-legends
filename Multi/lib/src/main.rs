mod mybe;
mod help;

use mybe::*;
use std::{env, process::exit, thread::sleep, time::Duration};
use help::usage;
use std::io::{self, Write};


fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        usage();
        exit(1);
    }

    match args[1].as_str() {
        "-O" => {           
            custem_hz_120();
            println!("\n- Apply use settings 120 Hz");
        }
        "-L" => {            
            custem_hz_90();
            println!("\n- Apply use settings 90 Hz");
        }
        "-Q" => {           
            custem_hz_60();
            println!("\n- Apply use settings 60 Hz");
        }
        "-h" | "--help" => usage(),
        other => {
            println!("Unknown option: {}", other);
            exit(1);
        }
    }

    sleep(Duration::from_secs(1));
    println!("\n⚠️ This module is protected by copyright and is\n\
              intended for use by regular users only. Any use of\n\
              this module, including its code, design, or features,\n\
              by other developers without written permission from\n\
              the copyright owner is strictly prohibited.\n\
              ________________________________________________(+)\n");
    io::stdout().flush().unwrap();
}