use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{interval, Duration};

/*
    Implementation found at:
    https://docs.rs/tokio/latest/tokio/sync/struct.Semaphore.html
*/
pub struct TokenBucket {
    pub sem: Arc<Semaphore>,
    pub join_handle: tokio::task::JoinHandle<()>,
}

impl TokenBucket {
    pub fn new(duration: Duration, capacity: usize) -> Self {
        let sem = Arc::new(Semaphore::new(capacity));

        let join_handle = tokio::spawn({
            let sem = sem.clone();
            let mut interval = interval(duration);
            interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            async move {
                loop {
                    interval.tick().await;
                    // println!("tick at: {:.2?}", Instant::now());
                    println!("semaphore permits left: {}", sem.available_permits());

                    // add a token to the bucket if it is not full
                    if sem.available_permits() < capacity {
                        sem.add_permits(1);
                    }
                }
            }
        });

        Self { sem, join_handle }
    }

    pub async fn acquire(&self) {
        // this can return an err if the sem is closed, but we never close it, so this shouldn't happen
        let permit = self.sem.acquire().await.unwrap();

        // to avoid releasing the permit back to the sem, we use the forget method
        permit.forget();
    }
}

impl Drop for TokenBucket {
    fn drop(&mut self) {
        // kill background task so it stops taking resources when not needed
        self.join_handle.abort();
    }
}
