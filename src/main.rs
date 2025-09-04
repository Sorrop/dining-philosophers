use core::time;
use std::{sync::{Arc, Mutex}, thread, time::{Duration, SystemTime}};
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
    times_fed: Arc<Mutex<u64>>
}

impl Philosopher {
    fn new(
        id: usize,
        left_chopstick: Arc<Mutex<Chopstick>>,
        right_chopstick: Arc<Mutex<Chopstick>>,
    ) -> Philosopher {
        Philosopher {
            id,
            left_chopstick,
            right_chopstick,
            times_fed: Arc::new(Mutex::new(0))
        }
    }

    fn think(&self) {
        let mut rng = rand::rng();
        let interval = rng.random_range(1..=5000);
        let millis = time::Duration::from_millis(interval);
        println!("{} is thinking...", self.id);
        thread::sleep(millis)
    }

    fn eat(&self, left_id: usize, right_id: usize) {
        let mut rng = rand::rng();
        let interval = rng.random_range(1..=5000);
        let millis = time::Duration::from_millis(interval);
        println!("{} is eating with {:?} and {:?}",
                 self.id,
                 left_id,
                 right_id);
        thread::sleep(millis)
    }

    fn try_to_eat(&self) -> bool {
        let locked_left = self.left_chopstick.try_lock();
        let locked_right = self.right_chopstick.try_lock();
        if let Ok(left_guard) = locked_left {
            if let Ok(right_guard) = locked_right {
                self.eat(left_guard.id, right_guard.id);
                let mut t = self.times_fed.lock().unwrap();
                *t += 1;
                drop(right_guard);
                drop(left_guard);
                println!("{} finished eating", self.id);
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

fn is_hungry() -> bool {
    let mut rng = rand::rng();
    rng.random_bool(0.5)
}

fn main() {
    let n: usize = 5;
    let chopsticks = n_chopsticks(n);
    let mut philosophers: Vec<Philosopher> = Vec::new();
    for i in 0..n {
        philosophers.push(
            Philosopher::new(
                i,
                chopsticks[(n - 1 + i) % n].clone(),
                chopsticks[i].clone(),
            ));
    }

    let stats: Vec<_> = philosophers.iter().map(|p| (p.id, p.times_fed.clone())).collect();

    let timeout = Duration::new(60, 0);
    let now = SystemTime::now();

    thread::scope (|scope| {
        for p in philosophers {
            scope.spawn(move || {
                loop {
                    if let Ok(elapsed) = now.elapsed() {
                        if elapsed >= timeout {
                            break;
                        }
                    } else {
                        eprintln!("Error getting system time");
                        break;
                    }
                    p.think();
                    if is_hungry() {
                        println!("{} is trying to eat", p.id);
                        p.try_to_eat();
                    }
                }
            });
        }
    });

    println!("--------- Results ---------");
    for (id, times_fed) in stats {
        println!("{} ate {} times", id, times_fed.lock().unwrap());
    }
}
