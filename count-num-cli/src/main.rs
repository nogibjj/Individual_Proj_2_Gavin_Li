use clap::Parser;
// use std::collections::HashMap;
use count_num_cli::{logic, string_to_list};

/// cli tool
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    ///
    #[arg(short, long)]
    default: bool,

    #[arg(short, long, default_value = "None")]
    cust_list: String,
}

fn main() {
    let defualt_nums = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 1, 3];
    let args = Args::parse();
    if args.default {
        let result = logic(defualt_nums);
        println!("The count of each number is {:?}", result);
        // return result;
    } else if args.cust_list == "None" {
        println!(
                "Please use --default flat to count numbers in the default list, or use --cust-list flag to input customized list"
            );
        // return None;
    } else {
        // deal with the string list
        let result = logic(string_to_list(args.cust_list));
        println!("The count of each number is {:?}", result);
        // return result;
    }
}
