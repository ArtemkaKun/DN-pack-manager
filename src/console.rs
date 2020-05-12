use std::{io, env};

use crate::pack_seeker::search_pack;
use crate::unpacker::start_unpack_process;

pub fn start_console() {
    println!("Choose the option: \n1) Unpack\n");

    let mut answer_method = String::new();
    io::stdin()
        .read_line(&mut answer_method)
        .expect("Failed to read");

    let answer_method: u32 = answer_method.trim().parse().expect("Need a number");

    if answer_method == 1 {
        if handle_args().path_to_packs.is_empty() || handle_args().path_where_unpack.is_empty() {
            println!("Need to provide path to packs and where unpack it");
            return;
        }

        let mut pack_list = search_pack(handle_args().path_to_packs);

        println!("Founded packs: \n");

        let mut i = 1;
        for one_pack in &pack_list {
            println!("{}. {}", i, one_pack);
            i += 1;
        }

        println!("Please, choose at least one to continue unpacking process: ");

        let mut answer_pack = String::new();
        io::stdin()
        .read_line(&mut answer_pack)
        .expect("Failed to read");

        // if answer_pack.len() > 2 {
            //TODO
        // }

        let answer_pack: usize = answer_pack.trim().parse().expect("Need a number");

        start_unpack_process(pack_list.remove(answer_pack - 1));

    } else {
        println!("Not an option");
    }
}

struct ProgramArgs {
    path_to_packs: String,
    path_where_unpack: String,
    path_where_files_for_pack: String,
    path_where_create_packs: String,
}

fn handle_args() -> ProgramArgs {
    let mut args = ProgramArgs {
        path_to_packs: String::new(),
        path_where_unpack: String::new(),
        path_where_files_for_pack: String::new(),
        path_where_create_packs: String::new(),
    };

    argument_seeker(&mut args, argument_collector());

    args
}

fn argument_seeker(args: &mut ProgramArgs, all_args: Vec<String>) {
    for mut one_arg in 0..all_args.len() {
        if all_args[one_arg] == "packs_path".to_string() {
            if all_args[one_arg + 1].to_string().is_empty() {
                println!("Empty path to packs!");
                continue;
            }

            args.path_to_packs = all_args[one_arg + 1].to_string();
        } else if all_args[one_arg] == "unpack_folder".to_string() {
            if all_args[one_arg + 1].to_string().is_empty() {
                println!("Empty path to unpack folder!");
                continue;
            }

            args.path_where_unpack = all_args[one_arg + 1].to_string();
        } else if all_args[one_arg] == "files_for_pack".to_string() {
            if all_args[one_arg + 1].to_string().is_empty() {
                println!("Empty path to files that need pack!");
                continue;
            }

            args.path_where_files_for_pack = all_args[one_arg + 1].to_string();
        } else if all_args[one_arg] == "new_packs_path".to_string() {
            if all_args[one_arg + 1].to_string().is_empty() {
                println!("Empty path where need to store new packs!");
                continue;
            }

            args.path_where_create_packs = all_args[one_arg + 1].to_string();
        }
    }
}

fn argument_collector() -> Vec<String> {
    let mut all_args: Vec<String> = env::args().collect();
    all_args.remove(0);
    all_args
}