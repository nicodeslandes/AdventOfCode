use std::cell::RefCell;
use std::collections::VecDeque;

pub struct Switch {
    values: RefCell<Vec<VecDeque<i64>>>,
}

impl Switch {
    pub fn new(size: usize) -> Self {
        let values = (0..size).map(|_| VecDeque::new()).collect();
        Switch {
            values: RefCell::new(values),
        }
    }

    pub fn write(&self, addr: usize, data: i64) -> () {
        println!("Addr {}: Writing {}", addr, data);
        if addr < self.values.borrow().len() {
            self.values.borrow_mut()[addr].push_back(data);
        }
    }

    pub fn read(&self, addr: usize) -> Option<i64> {
        let read = self.values.borrow_mut()[addr].pop_front();
        //println!("Addr {}: Reading {:?}", addr, read);
        read
    }
}
