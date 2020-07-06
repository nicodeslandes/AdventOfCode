use std::cell::RefCell;

pub struct Switch {
    values: RefCell<Vec<Vec<i64>>>,
}

impl Switch {
    pub fn new(size: usize) -> Self {
        let values = (0..size).map(|_| vec![]).collect();
        Switch {
            values: RefCell::new(values),
        }
    }

    pub fn write(&self, addr: usize, data: i64) -> () {
        println!("Addr {}: Writing {}", addr, data);
        self.values.borrow_mut()[addr].push(data);
    }

    pub fn read(&self, addr: usize) -> Option<i64> {
        let read = self.values.borrow_mut()[addr].pop();
        println!("Addr {}: Reading {:?}", addr, read);
        read
    }
}
