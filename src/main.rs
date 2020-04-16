use std::io;
use std::env;
use std::fs;
use std::path::Path;
use std::ffi::OsStr;
use std::fs::File;
use std::io::Read;

fn main() {
    println!("1) Unpack\n");

    let mut answer_method = String::new();
    io::stdin().read_line(&mut answer_method)
        .expect("Failed to read");

    let answer_method: u32 = answer_method.trim().parse()
        .expect("Need a number");

    match answer_method {
        1 => SearchPack(),
        _ => println!("Please, choose from list"),
    }
}

fn SearchPack() {
    let args = ArgumentCollector();

    let folder_fith_pack_path = &args[0];

    PackOpener(SearchInDir(folder_fith_pack_path));
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
    let mut file = File::open(&packs[0])
        .expect("Unable to open");

    let mut contents = String::new();
    file.read_to_string(&mut contents);

    println!("Here");
}