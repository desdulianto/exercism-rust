use std::collections::HashMap;

pub struct School {
    roster: HashMap<u32, Vec<String>>,
}

impl Default for School {
    fn default() -> Self {
        School::new()
    }
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.roster.entry(grade).or_insert_with(Vec::new);
        (*students).push(student.to_string());
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut grades = self.roster.keys().copied().collect::<Vec<u32>>();
        grades.sort();
        grades
    }

    // If grade returned an `Option<&Vec<String>>`,
    // the internal implementation would be forced to keep a `Vec<String>` to lend out.
    // By returning an owned vector instead,
    // the internal implementation is free to use whatever it chooses.
    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        match self.roster.get(&grade) {
            Some(students) => {
                let mut students = students.to_owned();
                students.sort();
                Some(students)
            }
            None => None,
        }
    }
}
