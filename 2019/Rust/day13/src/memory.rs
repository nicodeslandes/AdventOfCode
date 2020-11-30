use std::collections::HashMap;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Clone)]
pub struct Memory {
    _values: HashMap<usize, i64>,
}

impl Memory {
    pub fn new(values: HashMap<usize, i64>) -> Memory {
        Memory { _values: values }
    }

    pub fn parse(string: &str) -> Memory {
        let memory: HashMap<usize, i64> = string
            .split(",")
            .map(|x| {
                x.parse::<i64>()
                    .expect(format!("Failed to parse {}", x).as_str())
            })
            .enumerate()
            .collect();

        Memory::new(memory)
    }
}

impl Index<usize> for Memory {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        &self._values.get(&index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for Memory {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self._values.entry(index).or_insert(0)
    }
}
