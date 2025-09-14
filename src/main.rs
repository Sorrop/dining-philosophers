use core::time;
use std::{collections::{HashMap, HashSet}, sync::{Arc, Mutex}, thread, time::{Duration, SystemTime}};
use rand::Rng;
use clap::Parser;

#[derive(Parser)]
#[clap(name = "dining-philosophers")]
#[clap(version = "1.0")]
#[clap(about = "Simulate the dining philosophers problem.", long_about = None)]
struct Cli {
    /// The number of philosphers and chopsticks
    #[arg(short, long, default_value_t = 5)]
    number: usize,

    /// Simulation duration (in seconds)
    #[arg(short, long, default_value_t = 60)]
    duration: u64,

    /// Thinking max duration (in millis)
    #[arg(short, long, default_value_t = 5000)]
    think: u64,

    /// Eating max duration (in millis)
    #[arg(short, long, default_value_t = 5000)]
    eat: u64,
}

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

#[derive(Debug, Clone)]
enum Event {
    Thinking(usize),
    Eating(usize, usize, usize),
    FinishedEating(usize, usize, usize)
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

    fn think(&self, max_think_duration: u64, events: Arc<Mutex<Vec<Event>>>) {
        let millis = rand_sleep_duration(max_think_duration);
        let mut es = events.lock().unwrap();
        es.push(Event::Thinking(self.id));
        drop(es);
        thread::sleep(millis)
    }

    fn eat(&self, max_eat_duration: u64, left_id: usize, right_id: usize, events: Arc<Mutex<Vec<Event>>>) {
        let millis = rand_sleep_duration(max_eat_duration);
        let mut es = events.lock().unwrap();
        es.push(Event::Eating(self.id, left_id, right_id));
        drop(es);
        thread::sleep(millis)
    }

    fn is_hungry(&self) -> bool {
        let mut rng = rand::rng();
        rng.random_bool(0.5)
    }

    fn try_to_eat(&self, max_eat_duration: u64, events: Arc<Mutex<Vec<Event>>>) {
        let locked_left = self.left_chopstick.try_lock();
        let locked_right = self.right_chopstick.try_lock();

        if let Ok(left_guard) = locked_left {
            if let Ok(right_guard) = locked_right {
                let left_id = left_guard.id;
                let right_id = right_guard.id;
                self.eat(max_eat_duration, left_id, right_id, events.clone());
                let mut t = self.times_fed.lock().unwrap();
                *t += 1;
                let mut es = events.lock().unwrap();
                es.push(Event::FinishedEating(self.id, left_id, right_id));
                drop(right_guard);
                drop(left_guard);
                drop(es);
            }
        }
    }
}

fn rand_sleep_duration(max_millis: u64) -> time::Duration {
    let mut rng = rand::rng();
    let interval = rng.random_range(1..=max_millis);
    time::Duration::from_millis(interval)
}

fn n_chopsticks(n: usize) -> Vec<Arc<Mutex<Chopstick>>> {
    let mut out = Vec::new();
    for i in 0..n {
        out.push(Arc::new(Mutex::new(Chopstick::new(i))));
    }
    out
}

fn analyze(events: Vec<Event>, n: usize) {
    println!("Analyzing...");

    let mut currently_eating: HashSet<(usize, usize, usize)> = HashSet::new();
    let mut currently_thinking: HashSet<usize> = HashSet::new();
    let mut discrepancies: Vec<(usize, usize, usize)> = Vec::new();
    let mut times_fed: HashMap<usize, u64> = HashMap::new();

    for (i, e) in events.iter().enumerate() {
        match e {
            Event::Thinking(v) => {
                currently_thinking.insert(*v);
            },
            Event::Eating(v, left, right) => {
                for (p, eating_left, eating_right) in currently_eating.iter() {
                    if left == eating_left ||
                       left == eating_right ||
                       right == eating_left ||
                       right == eating_right {
                           discrepancies.push((i, *v, *p));
                       }
                }
                currently_thinking.remove(v);
                currently_eating.insert((*v, *left, *right));
            },
            Event::FinishedEating(v, left, right) => {
                currently_eating.remove(&(*v, *left, *right));
                times_fed.entry(*v).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }

    for i in 0..n {
        println!("{} ate {} times", i, times_fed.get(&i).or(Some(&0)).unwrap());
    }

    if discrepancies.len() == 0 {
        println!("Simulation correct");
    } else {
        println!("The following discrepancies were found:");
        for d in discrepancies.iter() {
            println!("{:?}", d);
        }
    }
}

fn main() {

    let cli = Cli::parse();
    let n: usize = cli.number;
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

    let events = Arc::new(Mutex::new(Vec::new()));

    let timeout = Duration::new(cli.duration, 0);
    let now = SystemTime::now();

    println!("Simulating....");

    thread::scope (|scope| {
        for p in philosophers {
            let events = events.clone();
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
                    p.think(cli.think, events.clone());
                    if p.is_hungry() {
                        p.try_to_eat(cli.eat, events.clone());
                    }
                }
            });
        }
    });

    analyze(events.lock().unwrap().to_vec(), n);

}
