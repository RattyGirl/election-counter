use std::io::{BufRead, Read};
use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

#[derive(Debug, Clone)]
pub struct BLT_Format {
    can_count: u32,
    vacancies: u32,
    withdraws: Vec<u32>,
    votes: Vec<Vote>,
    candidate_names: Vec<String>,
    election_name: String,

}

impl From<GoogleSheetFormat> for BLT_Format {
    fn from(value: GoogleSheetFormat) -> Self {
        Self {
            can_count: value.can_count,
            vacancies: value.vacancies,
            withdraws: Vec::new(),
            votes: value.votes,
            candidate_names: value.candidate_names,
            election_name: "".to_string(),
        }
    }
}

impl BLT_Format {
    pub fn read_from_file(file: PathBuf) -> Self {
        let file = fs::read_to_string(file).unwrap();
        //assumes correct format
        let mut lines = file.lines();
        let (can_count, vacancies) = lines.next().unwrap().trim_end().split_once(' ').unwrap();
        let can_count = can_count.parse().unwrap();
        let vacancies = vacancies.parse().unwrap();

        let mut votes: Vec<Vote> = Vec::new();
        let mut withdraws: Vec<u32> = Vec::new();
        let mut inballot = true;
        let mut candidates: Vec<String> = Vec::new();
        for line in lines {
            if line.trim().eq("0") {
                inballot = false;
                continue;
            }
            if line.starts_with('-') {
                withdraws = line
                    .split(" ")
                    .map(|x| x.replace("-", "").parse().unwrap())
                    .collect();
                continue;
            }
            if inballot {
                let mut prefs = line.trim().split(" ");
                let count: i32 = prefs.next().unwrap().parse().unwrap();
                let mut ballot: Vec<u32> = Vec::new();
                for pref in prefs {
                    if pref.trim().eq("0") {
                        break;
                    }
                    ballot.push(pref.parse::<u32>().unwrap());
                }
                votes.push(Vote {
                    value: count,
                    preferences: ballot,
                });
            } else {
                candidates.push(line.trim().replace("\"", "").to_string());
            }
        }

        Self {
            can_count,
            vacancies,
            withdraws,
            votes,
            candidate_names: candidates,
            election_name: "".to_string(),
        }
    }

    pub fn cleanup(&mut self) {
        self.votes.iter_mut().for_each(|x| {
            x.clean();
        });
        self.votes.retain(|x| x.preferences.len() > 0);
    }

    pub fn remove_withdrawals(&mut self) {
        self.votes.iter_mut().for_each(|vote| {
            vote.preferences = vote.preferences.iter().filter(|x| {
                let y = *x;
                !self.withdraws.contains(y)
            }).map(|x| *x).collect();
        });
        self.cleanup();
    }

    pub fn info(&self) -> String {
        let mut count = vec![0; self.can_count as usize];
        self.votes.iter().for_each(|vote| {
            let pref: usize = *vote.preferences.first().unwrap() as usize;
            count[pref - 1] += vote.value;
        });

        let mut out: Vec<String> = Vec::new();

        for i in 0..self.can_count {
            out.push(
                format!(
                    "{}={}",
                    self.candidate_names.get(i as usize).unwrap(),
                    count.get(i as usize).unwrap()
                ),
            );
        }
        out.join("\n")

    }
}

#[derive(Debug, Clone)]
struct Vote {
    value: i32,
    preferences: Vec<u32>,
}

impl Vote {
    pub fn clean(&mut self) {
        self.preferences = self
            .preferences
            .clone()
            .into_iter()
            .filter(|x| x != &(0 as u32))
            .collect();
    }
}

#[derive(Debug)]
pub struct GoogleSheetFormat {
    can_count: u32,
    vacancies: u32,
    votes: Vec<Vote>,
    candidate_names: Vec<String>,
}

impl GoogleSheetFormat {
    pub fn read_from_file(file: PathBuf) -> Self {
        let file = fs::read_to_string(file).unwrap();
        //assumes correct format
        let mut lines = file.lines();
        let (can_count, vacancies) = lines.next().unwrap().trim_end().split_once(' ').unwrap();
        let can_count: u32 = can_count.parse().unwrap();
        let vacancies = vacancies.parse().unwrap();

        let mut votes: Vec<Vote> = Vec::new();
        let mut withdraws: Vec<i32> = Vec::new();
        let mut inballot = true;
        let mut candidates: Vec<String> = Vec::new();
        for line in lines {
            if line.trim().eq("0") {
                inballot = false;
                continue;
            }
            if line.starts_with('-') {
                withdraws = line
                    .split(" ")
                    .map(|x| x.replace("-", "").parse().unwrap())
                    .collect();
                continue;
            }
            if inballot {
                let mut prefs = line.trim().split("	");
                let count: i32 = 1;
                let mut ballot: Vec<u32> = vec![0; can_count as usize];

                for (x, pref) in prefs.enumerate() {
                    if pref.trim().eq("0") {
                        break;
                    }
                    if pref.ne("") {
                        let position: usize = pref.parse().unwrap();
                        ballot[position - 1] = (x + 1) as u32;
                    }
                }

                votes.push(Vote {
                    value: count,
                    preferences: ballot,
                });
            } else {
                candidates.push(line.trim().replace("\"", "").to_string())
            }
        }

        Self {
            can_count,
            vacancies,
            votes,
            candidate_names: candidates,
        }
    }
}
