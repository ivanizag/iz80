use std::time::{Duration, Instant};

use super::cpu::Cpu;
use super::machine::Machine;

/// Helper to emulate real CPU speed.
///
/// Runs the CPU counting cycle and wall time.
pub struct TimedRunner {
    mhz: f64,
    quantum_cycles: u64,

    prev_cycle: u64,
    prev_time: Instant,
}

impl Default for TimedRunner {
    /// Returns a Timed Runner instance
    fn default() -> TimedRunner {
        TimedRunner {
            mhz: 0.0,
            quantum_cycles: 0,
            prev_cycle: 0,
            prev_time: Instant::now(),
        }
    }
}

impl TimedRunner {
    /// Define a new CPU speed in `MHz`
    pub fn set_mhz(&mut self, cpu: &Cpu, mhz: f64, quantum_cycles: u64) {
        self.mhz = mhz;
        self.quantum_cycles = quantum_cycles;

        // Reset times
        self.prev_cycle = cpu.cycle_count();
        self.prev_time = Instant::now();
    }

    /// Executes a single instruction, waiting if needed to emulate real CPU speed.
    pub fn execute(&mut self, cpu: &mut Cpu, sys: &mut dyn Machine) {
        if cpu.is_halted() {
            // The CPU is in HALT state. Only interrupts can execute.
            return;
        }

        if self.mhz != 0.0 {
            let cycles_elapsed = cpu.cycle_count() - self.prev_cycle;
            if cycles_elapsed > self.quantum_cycles {
                // Let's sleep if needed
                let now = Instant::now();
                let target_time = self.prev_time + Duration::from_nanos((cycles_elapsed as f64 * 1000.0 / self.mhz) as u64);

                if now < target_time {
                    std::thread::sleep(target_time - now);
                }

                self.prev_cycle = cpu.cycle_count();
                self.prev_time = target_time;
            }
        }

        cpu.execute_instruction(sys);
    }
}
