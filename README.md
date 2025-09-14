# Dining Philosophers simulation

A cli application for simulating the [dining philosphers](https://en.wikipedia.org/wiki/Dining_philosophers_problem) problem. 
Here it is solved with the Dijkstra's approach

### Prerequisites

- Rust (edition 2021 or newer)

### Build

```bash
cargo build --release
```

### Run
```bash
Usage: dinphils [OPTIONS]

Options:
  -n, --number <NUMBER>      The number of philosphers and chopsticks [default: 5]
  -d, --duration <DURATION>  Simulation duration (in seconds) [default: 60]
  -t, --think <THINK>        Thinking max duration (in millis) [default: 5000]
  -e, --eat <EAT>            Eating max duration (in millis) [default: 5000]
  -h, --help                 Print help
  -V, --version              Print version

```

📄 License
EPL-1.0 License. See LICENSE for details.
