use clap::Parser;
use std::path::Path;

extern crate remove_dir_all;

use remove_dir_all::*;

#[derive(Parser, Debug)]
#[clap(author, about, long_about = None)]
pub struct Args {
    /// remove directory where with locking files
    #[clap(short = 'r', long, default_value = "false")]
    pub remove_dir: String,
}
fn main() {
    let args = Args::parse();
    if args.remove_dir != "false" {
        if !Path::new(&args.remove_dir.to_string()).exists() {
            println!("Directory {} does not exist", args.remove_dir.to_string());
            std::process::exit(0);
        }
        match remove_dir_all(args.remove_dir) {
            Ok(_) => {
                println!("Directory removed successfully");
                std::process::exit(0);
            }
            Err(e) => {
                println!("Directory removed Error: {}", e);
                std::process::exit(1);
            }
        }
    }

    {
        println!("Without Any Parameter!");
        std::process::exit(1);
    }
}
