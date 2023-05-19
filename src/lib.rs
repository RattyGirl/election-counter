use std::{path::PathBuf, fs::{self, File}, io::BufReader};


pub struct BLT_Format {
    can_count: i32,
    vacancies: i32,
    withdraws: Vec<i32>,
    votes: Vec<Vote>,
    candidate_names: Vec<String>,
    election_name: String
}

impl BLT_Format {
    pub fn read_from_file(file: PathBuf) {
        let file: File = File::open(file).expect("Could not open file");
        let mut buf_reader = BufReader::new(file);
        if buf_reader.fill_buf()?.len() > 0 {
            assert!(!buf_reader.buffer().is_empty());
        }
    }
}

struct Vote {
    string: String
}