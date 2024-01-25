use std::{sync::Arc, time::Duration};
use tokio::{sync::Semaphore, task, time::sleep};

#[tokio::main]
async fn main() {
    console_subscriber::init();
    // 线程1的令牌桶1初始一个令牌，可以先打印1
    let semaphore = Arc::new(Semaphore::new(1));
    let cnt = 3;
    let semaphore2 = semaphore.clone();

    // 线程2的令牌桶2初始没有令牌，直到1打印后增加令牌
    let semaphore_wait = Arc::new(Semaphore::new(0));
    let semaphore_wait2 = semaphore_wait.clone();

    let t1 = task::Builder::default()
        .name("t1")
        .spawn(async move {
            for i in 0..cnt {
                let permit = semaphore.acquire().await.unwrap();
                print!("1 ");
                sleep(Duration::from_secs(i)).await;
                // 消耗令牌，不放回令牌桶1
                permit.forget();
                // 令牌桶2增加令牌，可以打印2
                semaphore_wait2.add_permits(1);
            }
        })
        .unwrap();

    let t2 = task::Builder::default()
        .name("t2")
        .spawn(async move {
            for i in 0..cnt {
                let permit = semaphore_wait.acquire().await.unwrap();
                print!("2 ");
                sleep(Duration::from_secs(i)).await;
                // 消耗令牌，不放回令牌桶2
                permit.forget();
                // 令牌桶1增加令牌，可以打印1
                semaphore2.add_permits(1);
            }
        })
        .unwrap();

    tokio::try_join!(t1, t2).unwrap();
    // sleep time:
    //  t1: 0 1 2    (4s)
    //  t2:   0 1 2  (6s)

    // Tasks (2) BUSY Running (0) IDLE Idle (1)
    // Warn  ID  State  Name  Total- Busy   Sched  Idle   Polls Target      Location          Fields
    //        15 IDLE   t2        6s    5ms    1ms     6s 6     tokio::task src/main.rs:33:10 kind=task
    //        14 DONE   t1        4s    3ms    1ms     4s 7     tokio::task src/main.rs:18:10 kind=task
}
