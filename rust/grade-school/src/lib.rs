use std::collections::BTreeMap;

pub struct School(BTreeMap<u32, Vec<String>>);

impl School {
    pub fn new() -> School {
        School(BTreeMap::new())
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let student = student.to_owned();
        let v = self.0.entry(grade).or_insert_with(Vec::new);
        match v.binary_search(&student) {
            Ok(p) => v.insert(p, student),
            Err(p) => v.insert(p, student)
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.0.keys().cloned().collect::<Vec<u32>>()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        self.0.get(&grade).map(|v| v.clone())
    }
}
