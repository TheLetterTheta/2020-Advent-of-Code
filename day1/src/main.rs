use std::io::{self, BufRead};
use std::collections::{HashSet, BTreeSet};
use std::ops::Bound::{Included, Excluded};

fn main() {
    // find_two();
    find_three();
}

fn find_two() {
    let stdin = io::stdin();
    let mut needed = HashSet::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let num: u32 = line.parse().unwrap();
        let parity: u32 = 2020 - num;
        
        if needed.contains(&num) {
            eprintln!("{0} + {1} = 2020;\n{0} * {1} = {2}", num, parity, num * parity);
            return;
        } else {
            needed.insert(parity);
        }

    }
    eprintln!("Not found in the list!")
}

fn find_three() {
    let stdin = io::stdin();
    let mut sorted_tree: BTreeSet<u32> = BTreeSet::new();

    for line in stdin.lock().lines().map(|l| l.unwrap()) {
        let first: u32 = line.parse().unwrap();
        let max: u32 = 2020 - first;
        
        let mut needed = HashSet::new();
        for &second in sorted_tree.range((Excluded(0), Excluded(max))) {
            let third = max - second;
            if needed.contains(&third) {
                eprintln!("{0} + {1} + {2} = 2020;\n{0} * {1} * {2} = {3}", first, second, third, first * second * third);
                return;
            } else {
                needed.insert(second);
            }
        }

        sorted_tree.insert(first);
    }
    eprintln!("Not found in the list!");
}
