use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use super::*;

type Worker<W> = Arc<W>;

pub struct Manager<W: StorageWorker> {
    workers: Vec<Worker<W>>,
    cursor: AtomicUsize,
}

impl<W: StorageWorker> Default for Manager<W> {
    fn default() -> Self {
        Self {
            workers: Vec::new(),
            cursor: AtomicUsize::new(0),
        }
    }
}

impl<W: StorageWorker> Manager<W> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_worker<T: IntoStorageWorker<W>>(&mut self, x: T) {
        self.workers.push(x.into_worker());
    }

    pub fn get_worker(&self, pos: usize) -> Option<Worker<W>> {
        self.workers.get(pos).map(|worker| worker.clone())
    }

    pub fn get_worker_next(&self) -> Option<Worker<W>> {
        let workers_len = self.workers.len();

        if workers_len == 0 {
            return None;
        }

        let pos = self.cursor.fetch_update(
            Ordering::SeqCst,
            Ordering::SeqCst,
            |pos| {
                if pos >= workers_len {
                    Some(0)
                } else {
                    Some(pos + 1)
                }
            }
        ).unwrap_or_else(|pos| pos);

        self.get_worker(pos)
    }

    pub fn get_worker_best(&self) -> Option<Worker<W>> {
        let mut workers = self.workers.iter();
        let mut best = workers.next()?;

        for worker in workers {
            if Arc::strong_count(worker) < Arc::strong_count(best) {
                best = worker;
            }
        }

        Some(best.clone())
    }
}
