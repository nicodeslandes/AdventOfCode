extern crate num;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};

type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct Chemical {
    name: String,
    qty: u32,
    sources: HashMap<String, u32>,
}

struct Quantities {
    chemical: String,
    produced: u32,
    available: u32,
}

impl Quantities {
    fn new(chemical: &String) -> Quantities {
        Quantities {
            chemical: chemical.clone(),
            produced: 0,
            available: 0,
        }
    }

    fn consume(&mut self, qty: u32) {
        if self.chemical != "ORE" {
            if self.available < qty {
                panic!(
                    "Not enough {} Need {}, but only {} available!",
                    self.chemical, qty, self.available
                );
            }

            self.available -= qty;
            println!(
                "Consumed {} of {} - available: {}",
                qty, self.chemical, self.available
            );
        }
    }

    fn produce(&mut self, qty: u32) {
        self.available += qty;
        self.produced += qty;
        println!(
            "Produced an extra {} of {} - total: {}, available: {}",
            qty, self.chemical, self.produced, self.available
        );
    }
}

#[allow(unused_variables)]

fn main() -> Result<()> {
    let file_name = env::args().nth(1).expect("Enter a file name");
    let file = File::open(file_name)?;

    let chemicals: HashMap<String, Chemical> = BufReader::new(file)
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line: Vec<_> = line.split("=>").collect();
            let sources = line[0]
                .split(",")
                .map(|s| {
                    let reaction = s.trim().split(" ");
                    let qty: u32 = reaction.clone().nth(0).unwrap().parse().unwrap();
                    let chemical = reaction.clone().nth(1).unwrap();
                    (chemical.to_string(), qty)
                })
                .collect();
            let chemical_def: Vec<&str> = line[1].trim().split(" ").collect();
            let qty = chemical_def[0].parse().unwrap();
            let name = chemical_def[1].to_string();
            (
                name.clone(),
                Chemical {
                    name: name.clone(),
                    qty,
                    sources,
                },
            )
        })
        .collect();

    for c in chemicals.values() {
        println!("{} {}: {:?}", c.qty, c.name, c.sources);
    }

    let mut qties: HashMap<String, Quantities> = HashMap::new();
    fn mine_chemical(
        c: &String,
        qty: u32,
        chemicals: &HashMap<String, Chemical>,
        qties: &mut HashMap<String, Quantities>,
    ) {
        //println!("Looking for {} of {}", qty, c);
        if c == "ORE" {
            let ore_qty = qties.entry(c.to_string()).or_insert(Quantities::new(&c));
            ore_qty.produce(qty);
            return;
        }

        let chemical = &chemicals[c];
        // How much do we have to produce?
        // We need the smallest multiple of chemical.qty that is greater than qty-available
        let available = qties.get(c).map(|x| x.available).unwrap_or_default();
        let required_amount = qty as i32 - available as i32;
        if required_amount <= 0 {
            // We've got enough already
            return;
        }

        let factor = if required_amount as u32 <= chemical.qty {
            1
        } else {
            (required_amount as f64 / chemical.qty as f64).ceil() as u32
        };

        println!(
            "Need {} more of {}; let's mine {}",
            required_amount,
            c,
            factor * chemical.qty
        );
        for (source, q) in &chemical.sources {
            mine_chemical(source, q * factor, chemicals, qties);
            let source_qty = qties.entry(source.clone()).or_insert(Quantities::new(&c));
            source_qty.consume(q * factor);
        }

        let qties = qties
            .entry(c.clone())
            .or_insert(Quantities::new(&c.to_string()));
        qties.produce(chemical.qty * factor);
    };

    mine_chemical(&"FUEL".to_string(), 1, &chemicals, &mut qties);
    let ore_qty: u32 = qties.get("ORE").map(|x| x.produced).unwrap_or_default();
    println!("Result: {}", ore_qty);

    Ok(())
}
