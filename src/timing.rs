use std::time::{Duration, Instant};

pub struct Measurement {
    epochs: u64,
    iters_per_epoch: u64,
    epoch_times: Vec<Duration>,
    iter_times: Vec<Vec<Duration>>,
    epoch_start: Instant,
    iter_start: Instant,
}

impl Measurement {
    pub fn new(epochs: u64, iters_per_epoch: u64) -> Self {
        Self {
            epochs,
            iters_per_epoch,
            epoch_times: Vec::with_capacity(epochs as usize),
            iter_times: Vec::with_capacity(epochs as usize),
            epoch_start: Instant::now(),
            iter_start: Instant::now(),
        }
    }

    pub fn start_epoch(&mut self) {
        self.epoch_start = Instant::now();
        self.iter_times.push(Vec::with_capacity(self.iters_per_epoch as usize));
    }

    pub fn end_epoch(&mut self) {
        self.epoch_times.push(self.epoch_start.elapsed());
    }

    pub fn start_iter(&mut self) {
        self.iter_start = Instant::now();
    }

    pub fn end_iter(&mut self, epoch_idx: usize) {
        let dur = self.iter_start.elapsed();
        self.iter_times[epoch_idx].push(dur);
    }

    pub fn avg_epoch_time(&self) -> Duration {
        self.epoch_times.iter().sum::<Duration>() / self.epochs as u32
    }

    pub fn avg_iter_time(&self) -> Duration {
        let total_iters = self.epochs * self.iters_per_epoch;
        self.iter_times
            .iter()
            .flatten()
            .copied()
            .sum::<Duration>()
            / total_iters as u32
    }

    pub fn epoch_report(&self, epoch_idx: usize) -> (Duration, Duration) {
        let epoch_time = self.epoch_times[epoch_idx];
        let iters = &self.iter_times[epoch_idx];
        let avg_iter = iters.iter().copied().sum::<Duration>() / self.iters_per_epoch as u32;
        (epoch_time, avg_iter)
    }

    pub fn print_epoch_report(&self, epoch_idx: usize){
        let (epoch_time, avg_iter_time) = self.epoch_report(epoch_idx);
        println!(
            "Epoch {} complete. Epoch time: {:.4}s, Avg iter time: {:.6}s",
            epoch_idx,
            epoch_time.as_secs_f64(),
            avg_iter_time.as_secs_f64()
        );
    }

    pub fn print_report(&self){
        println!(
            "All epochs complete.\nAvg epoch time: {:.4}s\nAvg iter time: {:.6}s",
            self.avg_epoch_time().as_secs_f64(),
            self.avg_iter_time().as_secs_f64()
        );
    }
}