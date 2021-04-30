
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

fn main() {
    // test_borrow();
    // test_thread();
    // test_thread2();
    // test_thread3();
    // test_thread4();
    // test_future();
    test_async_await();
}

fn test_borrow() {
    let mut x = 5;
    let mut y = &x;

    dbg!(x);
    dbg!(y);

    x = 6;
    y = &x;

    dbg!(x);
    dbg!(y);
}

fn test_thread() {
    let mut handles = Vec::new();

    for x in 0..10 {
        handles.push(thread::spawn(move || {
            println!("hello thread: {}", x);
        }));
    }

    for handle in handles {
        let r = handle.join();
        dbg!(r);
    }
}

fn test_thread2() {
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += 1
        }));
    }

    for h in handles {
        let _ = h.join();
    }

    dbg!(data);
}

fn test_thread3() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || {
        let data = rx.recv().unwrap();
        println!("{}", data);
    });

    let _ = tx.send("Hello message passing");

    let _ = handle.join();
}

fn test_thread4() {
    let mut handles = Vec::new();
    let mut data = vec![1; 10];
    let mut snd_channels = Vec::new();
    let mut rcv_channels = Vec::new();

    for _ in 0..10 {
        let (snd_tx, snd_rx) = mpsc::channel();
        let (rcv_tx, rcv_rx) = mpsc::channel();

        snd_channels.push(snd_tx);
        rcv_channels.push(rcv_rx);

        handles.push(thread::spawn(move || {
            let mut data = snd_rx.recv().unwrap();
            data += 1;
            let _ = rcv_tx.send(data);
        }));
    }

    for x in 0..10 {
        let _ = snd_channels[x].send(data[x]);
    }

    for x in 0..10 {
        data[x] = rcv_channels[x].recv().unwrap();
    }

    for handle in handles {
        let _ = handle.join();
    }

    dbg!(data);
}

use futures::{executor, future::join_all};
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct CountDown(u32);

impl Future for CountDown {
    type Output = String;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<String> {
        if self.0 == 0 {
            Poll::Ready("Zero!!!".to_string())
        } else {
            println!("{}", self.0);
            self.0 -= 1;
            // cx.waker().wake_by_ref();
            cx.waker().clone().wake();
            Poll::Pending
        }
    }
}

fn test_future() {
    let countdown_future1 = CountDown(10);
    let countdown_future2 = CountDown(20);
    let cd_set = join_all(vec![countdown_future1, countdown_future2]);
    let res = executor::block_on(cd_set);
    for (i, s) in res.iter().enumerate() {
        println!("{}: {}", i, s);
    }
}

async fn async_add(left: i32, right: i32) -> i32 {
    left + right
}

async fn something_great_async_function() -> i32 {
    let ans = async_add(2,3).await;
    let ans2 = async_add(1, 2);
    let ans3 = async_add(ans2.await, 3).await;
    println!("ans = {}", ans);
    println!("ans3 = {}", ans3);
    ans
}

fn test_async_await() {
    executor::block_on(something_great_async_function());
}
