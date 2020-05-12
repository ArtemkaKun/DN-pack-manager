use byteorder::{LittleEndian, ReadBytesExt};
use std::{fs::File, io::Cursor, io::Read, str};

pub struct Header {
    name: String,
    files_count: u32,
    file_offset: u32,
}

pub fn check_header(one_pack_path: &String) -> Header {
    let mut header = Header {
        name: "".to_string(),
        files_count: 0,
        file_offset: 0,
    };

    let read_buffer = read_whole_header(one_pack_path);

    read_pack_name(&mut header, &read_buffer);
    read_files_count(&mut header, &read_buffer);
    read_file_offset(&mut header, &read_buffer);
    println!("{}", header.file_offset);
    header
}

fn read_whole_header(one_path: &String) -> [u8; 1024] {
    let mut one_pack = File::open(one_path).expect("Unable to open");

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
