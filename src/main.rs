use std::fs;

fn main() {

    let raw_votes = fs::read_to_string("results.txt").expect("Could not read file");

}

fn sheet_line_to_blt() -> String {
    "".to_string()
}

fn vote_to_blt() {

}

fn is_vote_valid(vote: &str) -> bool {
    let split = vote.split("	");
}

#[test]
fn valid_vote() {
    assert!(is_vote_valid("3	2	6	5	1	4"));
    assert!(!is_vote_valid("6	5		3	1	2"));
}
// 1    2   3   4   5   6
// 3	2	6	5	1	4
// 5,2,1,6,4,3