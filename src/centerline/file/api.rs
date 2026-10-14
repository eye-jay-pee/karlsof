use crate::centerline::file::{CLFile, CLRecord};
#[allow(dead_code)]
impl CLFile {
    pub fn push(&mut self, record: CLRecord) {
        self.records.push(record)
    }
    pub fn pop(&mut self) -> Option<CLRecord> {
        self.records.pop()
    }
    pub fn insert(&mut self, i: usize, val: CLRecord) {
        self.records.insert(i, val)
    }
    pub fn remove(&mut self, i: usize) -> CLRecord {
        self.records.remove(i)
    }
    pub fn clear(&mut self) {
        self.records.clear()
    }
    pub fn truncate(&mut self, n: usize) {
        self.records.truncate(n)
    }
}
#[allow(dead_code)]
impl CLFile {
    pub fn len(&self) -> usize {
        self.records.len()
    }
    pub fn is_empty(&self) -> bool {
        self.records.is_empty()
    }
    pub fn get(&self, i: usize) -> Option<&CLRecord> {
        self.records.get(i)
    }
    pub fn get_mut(&mut self, i: usize) -> Option<&mut CLRecord> {
        self.records.get_mut(i)
    }
    pub fn first(&self) -> Option<&CLRecord> {
        self.records.first()
    }
    pub fn first_mut(&mut self) -> Option<&mut CLRecord> {
        self.records.first_mut()
    }
    pub fn last(&self) -> Option<&CLRecord> {
        self.records.last()
    }
    pub fn last_mut(&mut self) -> Option<&mut CLRecord> {
        self.records.last_mut()
    }
}
