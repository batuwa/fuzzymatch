// extern crate fuzzymatch;
mod algo;
use algo::{fuzzy_match, fuzzy_match_simple};

fn main() {
    println!("{:?}", fuzzy_match_simple("Faz", "warmFuzzy"));
    println!("{:?}", fuzzy_match_simple("arm", "warmFuzzy"));
    println!("{:?}", fuzzy_match("wfz", "warmFuzzy"));
    println!("{:?}", fuzzy_match("lib", "LinearStairBuilder"));
}