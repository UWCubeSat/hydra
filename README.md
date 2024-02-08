<div align="center">
  <h1>Hydra: HuskY Deterministic Realtime Avionics</h1>
  
  <!-- Link to Hydra Board -->
  <a href="https://github.com/uwcubesat/hydra-board"><img src="https://img.shields.io/badge/hydra-board-green"></img></a><br>
  Hydra is a deterministic real-time avionics solution for HuskySat 2.

  <h4>Everything here is preliminary and not set in stone.</h4>
</div>

This will be the repository ONLY for the firmware. In terms of versioning, we will utilize a modified [Semantic Versioning](https://semver.org/).

### Hydra Semantic Versioning
  - MAJOR version when you make incompatible API changes
    - Any board or firmware with the same MAJOR version shall be compatible.
  - MINOR version when you add functionality in a backward compatible manner
    - All boards or firmware with different MINOR versions but the same MAJOR version shall be compatible.
  - PATCH version when you make backward compatible bug fixes
    - All boards or firmware with different PATCH versions but the same MAJOR version shall be compatible.


## Firmware Architectural Overview
HYDRA will achieve deterministic execution by having a static set of tasks. These tasks will each have a single responsibility and will share a processor through a cooperative scheduler.

Realtime considerations will happen later with regards to the firmware payload.

Tasks will be generally uninterruptable (except by on chip interrupts) and will each do one task (ex. a UARTSenderTask will send a single message over UART before returning/yielding). This allows for us to verify deterministic behavior by modeling our program as a state machine.

General Notes:
- Loops will not function well in this architecture (especially loops with a large N) and should generally be avoided.
    - For example, sensor averaging is usually done in a loop. A better way to do it in this architecture would be with a moving average that adds the value to a cirular buffer on every invocation.
- Inter-task communication will happen through statically allocated queues.
    - Certain tasks will be "producers" who write into the queue and certain tasks will be "consumers" who read out of the queue. If a queue is full, producers will not be allowed to run (due to their predicate). If a queue is empty, consumers will not be allowed to run (due to their predicate).

Hydra will utilize a Predicate Priority Cooperative Scheduler [^1]
  - Predicate: All tasks will have predicates that will need to be true for them to be eligible for them to run.
  - Priority: All tasks will have a priorty attached that dictates the order they are run in.
  - Cooperative: All tasks cannot (generally) be interrupted and will have to return or yield.

## Getting Started
1. Install Rust using [rustup](https://rustup.rs/)
2. Install ARM Toolchain (for Cortex-M3) using Rustup
```zsh
  rustup target add thumbv7m-none-eabi
```
3. Install Cargo Binutils
```zsh
  cargo install cargo-binutils
```
4. Install LLVM-tools
```zsh
  rustup component add llvm-tools
```

[^1]: There is a version of this scheduler written in Zig that was functional and was able to handle soft real-time tasks reliably. Priority order was very important however when it came to ensuring that tasks ran as expected.
