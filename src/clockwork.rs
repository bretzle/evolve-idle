use std::time::{Duration, Instant};

pub struct Job<S> {
    frequency: Duration,
    last_run: Instant,
    handler: Box<dyn FnMut(&mut S)>,
}

impl<S> Job<S> {
    fn new(frequency: Duration) -> Self {
        Self {
            frequency,
            last_run: Instant::now(),
            handler: Box::new(|_| panic!("Make sure to add a handler to every Job!")),
        }
    }

    pub fn run<F: FnMut(&mut S) + 'static>(&mut self, handler: F) {
        self.handler = Box::new(handler);
    }

    fn is_pending(&self, now: &Instant) -> bool {
        *now - self.last_run >= self.frequency
    }

    fn execute(&mut self, now: &Instant, state: &mut S) {
        self.last_run = now.clone();
        (self.handler)(state)
    }
}

pub struct Clockwork<S> {
    jobs: Vec<Job<S>>,
}

impl<S> Clockwork<S> {
    pub fn new() -> Self {
        Self { jobs: vec![] }
    }

    pub fn every(&mut self, frequency: Duration) -> &mut Job<S> {
        let job = Job::new(frequency);
        self.jobs.push(job);
        let idx = self.jobs.len() - 1;
        &mut self.jobs[idx]
    }

    pub fn run_pending(&mut self, state: &mut S) {
        let now = Instant::now();
        for job in &mut self.jobs {
            if job.is_pending(&now) {
                job.execute(&now, state);
            }
        }
    }
}
