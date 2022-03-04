use std::sync::mpsc;
use std::thread;
use std::time::Duration;

pub fn thread_t() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    handle.join().unwrap(); // 等待spawned线程结束
}

pub fn thread_t_move() {
    let v = vec![1, 2, 3, 4];

    let handle = thread::spawn(move || {
        println!("here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}

pub fn thread_t_channel() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {}", received);
}

pub fn thread_t_multiple_thread_to_one_channel() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();

    thread::spawn(move || {
        let vs = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for v in vs {
            tx.send(v).unwrap();
        }
    });

    thread::spawn(move || {
        let vs = vec![
            String::from("more"),
            String::from("message"),
            String::from("from"),
            String::from("thread2"),
        ];

        for v in vs {
            tx1.send(v).unwrap();
        }
    });

    for v in rx {
        println!("received: {}", v);
    }
}

/*
        Mutex
*/

use std::sync::{Arc, Mutex};

pub fn thread_t_mutex() {
    let m = Mutex::new(5);

    {
        let mut num = m.lock().unwrap();
        *num = 6;
        /*
         * 正如你所怀疑的，
         * Mutex<T> 是一个智能指针。
         * 更准确的说，lock 调用 返回 一个叫做 MutexGuard 的智能指针。
         * 这个智能指针实现了 Deref 来指向其内部数据；
         * 其也提供了一个 Drop 实现当 MutexGuard 离开作用域时自动释放锁，
         */
    }

    println!("m = {:?}", m);
}

pub fn thread_t_mutex_arc() {
    let counter = Arc::new(Mutex::new(0));
    /*
     * Rc<T> 并不能安全的在线程间共享。
     * 当 Rc<T> 管理引用计数时，
     * 它必须在每一个 clone 调用时增加计数，
     * 并在每一个克隆被丢弃时减少计数。Rc<T> 并没有使用任何并发原语，
     * 来确保改变计数的操作不会被其他线程打断。
     * 在计数出错时可能会导致诡异的 bug，
     * 比如可能会造成内存泄漏，或在使用结束之前就丢弃一个值。
     * 我们所需要的是一个完全类似 Rc<T>，又以一种线程安全的方式改变引用计数的类型。
     *
     * 所幸 Arc<T> 正是 这么一个类似 Rc<T> 并可以安全的用于并发环境的类型。
     * 字母 “a” 代表 原子性（atomic），
     * 所以这是一个 原子引用计数（atomically reference counted）类型。
     */
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });

        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
