use std::{fs, io::Read};

fn get_rdm_string(size: usize) -> String {
    let mut data = fs::File::open("/dev/random").unwrap();
    let mut buf = vec![0u8; size];
    let mut final_vec: Vec<u8> = Vec::new();

    let _ = data.read_exact(&mut buf);

    for i in buf {
        final_vec.push(48 + (i % 10));
    }
    String::from_utf8(final_vec).unwrap_or_default()
}

pub fn generate_uuid() -> String {
    format!(
        "{}-{}-{}-{}",
        get_rdm_string(10),
        get_rdm_string(10),
        get_rdm_string(10),
        get_rdm_string(10),
    )
}

pub fn get_fd_list() -> Vec<usize> {
    for file in fs::read_dir("/proc/self/fd/").unwrap() {
        println!("{}", file.unwrap().file_name().to_str().unwrap())
    }
    Vec::new()
}
