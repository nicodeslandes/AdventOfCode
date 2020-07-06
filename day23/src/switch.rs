use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;

pub struct Switch {
    values: RefCell<Vec<VecDeque<Packet>>>,
}

impl Switch {
    pub fn new(size: usize) -> Self {
        let values = (0..size).map(|_| VecDeque::new()).collect();
        Switch {
            values: RefCell::new(values),
        }
    }

    pub fn write(&self, addr: usize, data: Packet) -> () {
        //println!("Addr {}: Writing {}", addr, data);
        if addr < self.values.borrow().len() {
            self.values.borrow_mut()[addr].push_back(data);
        }
        else {
            println!("NAT packet: {}", data);
        }
    }

    pub fn read(&self, addr: usize) -> Option<Packet> {
        let read = self.values.borrow_mut()[addr].pop_front();
        //println!("Addr {}: Reading {:?}", addr, read);
        read
    }
}

#[derive(Clone, Copy)]
pub struct Packet {
    pub x: i64,
    pub y: i64,
}

impl Packet {
    pub fn new(x: i64, y: i64) -> Packet {
        Packet { x, y }
    }
}

impl fmt::Display for Packet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "(X: {}, Y: {})", self.x, self.y)
    }
}
