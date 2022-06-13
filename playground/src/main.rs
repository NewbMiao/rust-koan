use std::{sync::Arc, thread, time::Duration};

use tokio::{sync::Semaphore, task::JoinHandle};

#[tokio::main]
async fn main() {
    let library = Box::new(Library::new(2));
    let mut tasks: Vec<JoinHandle<()>> = vec![];
    for i in 0..5 {
        tasks.push(library.enter(move || {
            thread::sleep(Duration::from_secs(i));
            println!("no: {}, borrow book", i);
        }));
    }
    for task in tasks {
        task.await.unwrap();
    }
    println!("{:?}", library.semaphore.available_permits());
}

struct Library {
    semaphore: Arc<Semaphore>,
}

impl Library {
    fn new(capacity: usize) -> Self {
        Self {
            semaphore: Arc::new(Semaphore::new(capacity)),
        }
    }

    fn enter(&self, chore: impl Fn() + Send + 'static) -> JoinHandle<()> {
        let semaphore = self.semaphore.clone();
        tokio::spawn(async move {
            // remove this you will get panic
            // semaphore.close();
            println!("{} quota left", semaphore.available_permits());

            let _s = semaphore.acquire_owned().await.unwrap();
            chore();

            drop(_s);
        })
    }
}
