use rand::Rng;
use std::io::{self, BufRead};

enum Items {
    Goat,
    Car,
}

enum Strategy {
    Switch,
    Stay,
}

fn simulate(strategy: Strategy) -> bool {
    let mut doors: Vec<_>;
    match rand::thread_rng().gen_range(0..=2) {
        0 => doors = vec![Items::Car, Items::Goat, Items::Goat],
        1 => doors = vec![Items::Goat, Items::Car, Items::Goat],
        2 => doors = vec![Items::Goat, Items::Goat, Items::Car],
        _ => panic!(),
    }

    let choice = rand::thread_rng().gen_range(0..=2);
    doors.remove(choice);

    let revealed = rand::thread_rng().gen_range(0..=1);
    match doors[revealed] {
        Items::Goat => doors.remove(revealed),
        Items::Car => doors.remove(1 - revealed),
    };

    matches!(
        (&doors[0], strategy),
        (Items::Car, Strategy::Switch) | (Items::Goat, Strategy::Stay)
    )
}

fn main() {
    let mut switch_successes = 0;
    let mut stay_successes = 0;

    println!("How many iterations should be ran?");
    let mut line = String::new();
    let stdin = io::stdin();
    stdin
        .lock()
        .read_line(&mut line)
        .expect("Line could not be read");
    let iterations: i32 = line
        .trim()
        .parse()
        .expect("Could not parse input as integer");

    for _ in 0..iterations {
        if simulate(Strategy::Switch) {
            switch_successes += 1;
        }
        if simulate(Strategy::Stay) {
            stay_successes += 1;
        }
    }
    println!(
        "{} successes with switching, {} successes with staying after {} simulations of each strategy.",
        switch_successes, stay_successes, iterations
    )
}
