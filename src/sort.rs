use std::cmp::Ordering;

pub fn number_sort<T: Ord>(array: &mut [T]) {
    let len = array.len();
    for i in 0..len.saturating_sub(1) {
        for j in 0..(len - 1 - i) {
            if array[j] > array[j + 1] {
                array.swap(j, j + 1);
            }
        }
    }
}

#[derive(Debug, PartialEq, Ord, Eq)]
pub struct Person {
    pub name: String,
    pub age: i32,
}

impl PartialOrd for Person {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
