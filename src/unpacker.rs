use crate::header::check_header;

pub fn start_unpack_process(next_pack: String) {
    pack_opener(next_pack);
}

fn pack_opener(packs: String) {
    check_header(&packs);
}
