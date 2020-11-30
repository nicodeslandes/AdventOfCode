extern crate num;

use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::result::Result;

type MainResult<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

#[derive(Debug)]
struct Chemical {
    name: String,
    qty: u64,
    sources: HashMap<String, u64>,
}

#[derive(Clone)]
struct Quantities {
    chemical: String,
    produced: u64,
    available: u64,
}

impl Quantities {
    fn new(chemical: &String) -> Quantities {
        Quantities {
            chemical: chemical.clone(),
            produced: 0,
            available: 0,
        }
    }

    fn consume(&mut self, qty: u64) -> Result<(), String> {
        if self.available < qty {
            return Err(format!(
                "Not enough {} Need {}, but only {} available!",
                self.chemical, qty, self.available
            ));
        }

        self.available -= qty;
        // println!(
        //     "Consumed {} of {} - available: {}",
        //     qty, self.chemical, self.available
        // );
        Ok(())
    }

    fn produce(&mut self, qty: u64) {
        self.available += qty;
        self.produced += qty;
        // println!(
        //     "Produced an extra {} of {} - total: {}, available: {}",
        //     qty, self.chemical, self.produced, self.available
        // );
    }
}

#[allow(unused_variables)]

fn main() -> MainResult<()> {
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
                    let qty: u64 = reaction.clone().nth(0).unwrap().parse().unwrap();
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

    let ore = "ORE".to_string();
    let fuel = "FUEL".to_string();
    qties.insert(ore.clone(), Quantities::new(&ore));
    let total_ore = 1_000_000_000_000;
    qties.get_mut(&ore).unwrap().available = total_ore;

    // How much ore to mine 1 FUEL?
    mine_chemical(&fuel, 1, &chemicals, &mut qties)?;
    mine_chemical(&fuel, 1, &chemicals, &mut qties)?;
    let ore_qties = &qties[&ore];
    let fuel_qties = &qties[&fuel];
    let ore_per_fuel = (total_ore - ore_qties.available) / fuel_qties.produced;

    println!("Ore per fuel: {}", ore_per_fuel);
    // Roughly, we should be able to mine 1_000_000_000 / ore_unit fuel
    let mut fuel_amount = total_ore / ore_per_fuel;
    mine_chemical(&fuel, fuel_amount, &chemicals, &mut qties)?;

    // How much ore is left?
    println!("Ore available: {}", qties[&ore].available);

    let mut try_fuel_amount = |amount| {
        println!("Trying amount {}", amount);
        let original_qties = qties.clone();
        let res = mine_chemical(&fuel, amount, &chemicals, &mut qties);
        if res.is_err() {
            // Reset the qties
            qties = original_qties;
        }

        println!("Res: {:?}; Ore available: {}", res, qties[&ore].available);
        res
    };

    while let Ok(_) = try_fuel_amount(fuel_amount) {
        fuel_amount *= 2;
    }

    let mut high_limit = fuel_amount;
    let mut low_limit = fuel_amount / 2;
    fuel_amount = low_limit;
    loop {
        match try_fuel_amount(fuel_amount) {
            Ok(_) => low_limit = fuel_amount,
            Err(_) => high_limit = fuel_amount,
        }

        let candidate = (low_limit + high_limit) / 2;
        if candidate == fuel_amount {
            break;
        }
        fuel_amount = candidate;
    }
    //let fuel_qty: u64 = qties.get(&fuel).map(|x| x.produced).unwrap_or_default();
    println!("Result: {}", fuel_amount);

    Ok(())
}

fn mine_chemical(
    c: &String,
    qty: u64,
    chemicals: &HashMap<String, Chemical>,
    qties: &mut HashMap<String, Quantities>,
) -> Result<u64, String> {
    // println!("Looking for {} of {}", qty, c);
    if c == "ORE" {
        return Ok(qty);
    }

    let chemical = &chemicals[c];
    // How much do we have to produce?
    // We need the smallest multiple of chemical.qty that is greater than qty-available
    let available = qties.get(c).map(|x| x.available).unwrap_or_default();
    let required_amount = qty as i64 - available as i64;
    if required_amount <= 0 {
        // We've got enough already
        return Ok(0);
    }

    let factor = if required_amount as u64 <= chemical.qty {
        1
    } else {
        (required_amount as f64 / chemical.qty as f64).ceil() as u64
    };

    // println!(
    //     "Need {} more of {}; let's mine {}",
    //     required_amount,
    //     c,
    //     factor * chemical.qty
    // );
    for (source, q) in &chemical.sources {
        mine_chemical(source, q * factor, chemicals, qties)?;
        let source_qty = qties.entry(source.clone()).or_insert(Quantities::new(&c));
        source_qty.consume(q * factor)?;
    }

    let qties = qties
        .entry(c.clone())
        .or_insert(Quantities::new(&c.to_string()));
    qties.produce(chemical.qty * factor);
    Ok(chemical.qty * factor)
}
