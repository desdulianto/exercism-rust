use std::collections::{BTreeMap, BTreeSet};
use std::iter::FromIterator;

pub struct School {
    roster: BTreeMap<u32, BTreeSet<String>>,
}

impl Default for School {
    fn default() -> Self {
        School::new()
    }
}

impl School {
    pub fn new() -> School {
        School {
            roster: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster
            .entry(grade)
            .or_insert_with(BTreeSet::new)
            .insert(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        self.roster.keys().copied().collect()
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.roster.get(&grade) {
            Some(students) => Some(Vec::from_iter(students.to_owned())),
            None => None,
        }
    }
}
