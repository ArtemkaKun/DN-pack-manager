use std::io;
use std::io::Read;

use std::fs::File;
use std::fs;

use std::path::Path;
use std::ffi::OsStr;

use std::env;
use std::str;

use std::io::Cursor;
use byteorder::{LittleEndian, ReadBytesExt};

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
        pack_opener(search_pack());
    }
}

fn search_pack() -> Vec<String> {
    let args = argument_collector();

    let folder_fith_pack_path = &args[0];

    search_in_dir(folder_fith_pack_path)
}

fn argument_collector() -> Vec<String> {
    let mut all_args: Vec<String> = env::args().collect();
    all_args.remove(0);
    all_args
}

fn search_in_dir(path: &String) -> Vec<String> {
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

struct Header {
    name: String,
    files_count: u32,
    file_offset: u32,
}

fn pack_opener(packs: Vec<String>) {
    check_header(&packs[2]);   
}

fn check_header(one_pack_path: &String) -> Header {
    let mut header = Header {name: "".to_string(), files_count: 0, file_offset: 0};

    let read_buffer = read_whole_header(one_pack_path);

    read_pack_name(&mut header, &read_buffer);
    read_files_count(&mut header, &read_buffer);
    read_file_offset(&mut header, &read_buffer);
    println!("{}", header.file_offset);
    header
}

fn read_whole_header(one_path: &String) -> [u8; 1024] {
    let mut one_pack = File::open(one_path)
        .expect("Unable to open");

    let mut read_buffer = [0; 1024];

    match one_pack.read(&mut read_buffer) {
        Ok(v) => v,
        Err(e) => panic!("Cannot read the pack's header!"),
    };

    read_buffer
}

fn read_pack_name(header: &mut Header, bytes: &[u8]) {
    const DN_PACK_HEADER_LINE: &str = "EyedentityGames Packing File 0.1";

    let read_headname_buffer = &bytes[0..32];

    let string_from_header = match str::from_utf8(&read_headname_buffer) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    if string_from_header != String::from(DN_PACK_HEADER_LINE) {
        panic!("This pack doesn't Dragon Nest pack or this pack damaged!");
    }

    header.name = string_from_header.to_string();
}

fn read_files_count(header: &mut Header, bytes: &[u8]) {
    check_start_flag(bytes);

    let read_files_bytes = &bytes[260..264];

    let mut bites_to_int = Cursor::new(read_files_bytes);
    let files_count = bites_to_int.read_u32::<LittleEndian>().unwrap();
    
    header.files_count = files_count;
}

fn check_start_flag(bytes: &[u8]) {
    let start_flag_bytes = &bytes[256..260];

    let mut bites_to_int = Cursor::new(start_flag_bytes);
    let start_flag = bites_to_int.read_u16::<LittleEndian>().unwrap();

    if start_flag != 11 {
        panic!("This pack damaged");
    }
}

fn read_file_offset(header: &mut Header, bytes: &[u8]) {
    let read_files_bytes = &bytes[264..268];

    let mut bites_to_int = Cursor::new(read_files_bytes);
    let files_offset = bites_to_int.read_u32::<LittleEndian>().unwrap();
    
    header.file_offset = files_offset;
}