use std::io;
use std::io::Read;

use std::fs::File;
use std::fs;

use std::path::Path;
use std::ffi::OsStr;

use std::env;
use std::str;

fn main() {
    println!("1) Unpack\n");

    let mut answer_method = String::new();
    io::stdin().read_line(&mut answer_method)
        .expect("Failed to read");

    let answer_method: u32 = answer_method.trim().parse()
        .expect("Need a number");


    let action = match answer_method {
        1 => true,
        _ => false,
    };

    if action {
        PackOpener(SearchPack());
    }
}

fn SearchPack() -> Vec<String> {
    let args = ArgumentCollector();

    let folder_fith_pack_path = &args[0];

    SearchInDir(folder_fith_pack_path)
}

fn ArgumentCollector() -> Vec<String> {
    let mut all_args: Vec<String> = env::args().collect();
    all_args.remove(0);
    return all_args;
}

fn SearchInDir(path: &String) -> Vec<String> {
    let dir_path = Path::new(path);

    let mut packs = Vec::<String>::new();

    println!("Founded packs: \n");
    
    for one_file in fs::read_dir(dir_path).expect("Unable to list or connect") {
        let one_file = one_file.expect("Unable to get file");

        if one_file.path().extension() == Some(OsStr::new("pak")) {
            packs.push(one_file.path().display().to_string());
            println!("{}", one_file.path().display());
        }
    }

    packs
}

fn PackOpener(packs: Vec<String>) {
    CheckHeaderLine(&packs[2]);   
}

fn CheckHeaderLine(one_pack_path: &String) -> bool {
    const DN_PACK_HEADER_LINE: &str = "EyedentityGames Packing File 0.1";

    let mut one_pack = File::open(one_pack_path)
        .expect("Unable to open");

    let mut read_buffer = [0; 32];

    let can_read_header = match one_pack.read(&mut read_buffer) {
        Ok(_) => true,
        Err(_) => true,
    };

    if !can_read_header {
        println!("Cannot read the pack's header!");
        return false;
    }

    let string_from_header = match str::from_utf8(&read_buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    if string_from_header != String::from(DN_PACK_HEADER_LINE) {
        println!("This pack doesn't Dragon Nest pack or this pack damaged!");
        return false;
    }

    println!("{}", string_from_header);
    true
}