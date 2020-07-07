use std::cell::Cell;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::fmt;

pub struct Switch {
    values: RefCell<Vec<VecDeque<Packet>>>,
    activity: RefCell<i64>,
    nat_packet: Cell<Option<Packet>>,
}

impl Switch {
    pub fn new(size: usize) -> Self {
        let values = (0..size).map(|_| VecDeque::new()).collect();
        Switch {
            values: RefCell::new(values),
            activity: RefCell::new(0),
            nat_packet: Cell::new(None),
        }
    }

    pub fn get_activity(&self) -> i64 {
        *self.activity.borrow()
    }

    pub fn write(&self, addr: usize, data: Packet) -> () {
        //println!("Addr {}: Writing {}", addr, data);
        if addr < self.values.borrow().len() {
            self.values.borrow_mut()[addr].push_back(data);
            *self.activity.borrow_mut() += 1;
        } else {
            //println!("NAT packet: {}", data);
            self.nat_packet.set(Some(data));
        }
    }

    pub fn read(&self, addr: usize) -> Option<Packet> {
        let read = self.values.borrow_mut()[addr].pop_front();
        if read.is_some() {
            *self.activity.borrow_mut() += 1;
        }
        //println!("Addr {}: Reading {:?}", addr, read);
        read
    }

    pub fn pop_nat_packet(&self) -> Option<Packet> {
        let result = self.nat_packet.get();
        self.nat_packet.set(None);
        result
    }

    pub fn is_quiet(&self) -> bool {
        self.values.borrow().iter().all(|s| s.is_empty())
    }
}

#[derive(Clone, Copy, Debug)]
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
