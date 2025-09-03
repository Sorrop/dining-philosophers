use core::time;
use std::{sync::{Arc, Mutex}, thread};

use rand::Rng;

#[derive(Debug)]
struct Chopstick {
    id: usize,
}

impl Chopstick {
    fn new(id: usize) -> Chopstick {
        Chopstick { id }
    }
}

#[derive(Debug)]
struct Philosopher {
    id: usize,
    left_chopstick: Arc<Mutex<Chopstick>>,
    right_chopstick: Arc<Mutex<Chopstick>>,
}

impl Philosopher {
    fn new(
        id: usize,
        left_chopstick: Arc<Mutex<Chopstick>>,
        right_chopstick: Arc<Mutex<Chopstick>>,
    ) -> Philosopher {
        Philosopher {
            id,
            left_chopstick: left_chopstick,
            right_chopstick: right_chopstick,
        }
    }

    fn think(&self) {
        let mut rng = rand::rng();
        let interval = rng.random_range(1..=15000);
        let millis = time::Duration::from_millis(interval);
        println!("Philosopher {} thinking...", self.id);
        thread::sleep(millis)
    }

    fn eat(&self) {
        let mut rng = rand::rng();
        let interval = rng.random_range(1..=15000);
        let millis = time::Duration::from_millis(interval);
        println!("Philosopher {:?} eating with {:?} and {:?}",
                 self.id,
                 self.left_chopstick,
                 self.right_chopstick);
        thread::sleep(millis)
    }

    fn try_to_eat(&self) -> bool {
        let locked_left = self.left_chopstick.try_lock();
        let locked_right = self.right_chopstick.try_lock();
        if let Ok(left_guard) = locked_left {
            if let Ok(right_guard) = locked_right {
                self.eat();
                drop(right_guard);
                drop(left_guard);
                return true
            }
        }
        return false
    }
}

fn n_chopsticks(n: usize) -> Vec<Arc<Mutex<Chopstick>>> {
    let mut out = Vec::new();
    for i in 0..n {
        out.push(Arc::new(Mutex::new(Chopstick::new(i))));
    }
    out
}

fn main() {
    let n: usize = 5;
    let chopsticks = n_chopsticks(n);
    let mut philosophers: Vec<Philosopher> = Vec::new();
    for i in 0..n {
        philosophers.push(Philosopher::new(
            i,
            chopsticks[(n - 1 + i) % n].clone(),
            chopsticks[i].clone(),
        ));
    }

    thread::scope (|scope| {
        for p in philosophers {
            scope.spawn(move || {
                loop {
                    p.think();
                    p.try_to_eat();
                }
            });
        }
    });
}
