use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;

const N: usize = 8;

struct RingBuffer<T> {
    data: [UnsafeCell<T>; N],
    write: AtomicUsize,
    read: AtomicUsize,
}

// Safe to share only because slot access follows the SPSC protocol below.
// Each slot has its own UnsafeCell; other slots may be accessed concurrently.
unsafe impl<T: Copy + Send> Sync for RingBuffer<T> {}

impl<T: Copy + Default> RingBuffer<T> {
    fn new() -> Self {
        Self {
            data: std::array::from_fn(|_| UnsafeCell::new(T::default())),
            write: AtomicUsize::new(0),
            read: AtomicUsize::new(0),
        }
    }

    // SAFETY: Only one producer may call push. Only one consumer may call pop.
    unsafe fn push(&self, value: T) -> bool {
        let write = self.write.load(Ordering::Relaxed);
        let read = self.read.load(Ordering::Acquire);

        if write.wrapping_sub(read) == N {
            return false; // Full: do not overwrite an unread element.
        }

        let position = write & (N - 1);
        unsafe { *self.data[position].get() = value; }
        self.write.store(write.wrapping_add(1), Ordering::Release);
        true
    }

    // SAFETY: Same single-producer, single-consumer requirement as push.
    unsafe fn pop(&self) -> Option<T> {
        let read = self.read.load(Ordering::Relaxed);
        let write = self.write.load(Ordering::Acquire);

        if read == write {
            return None; // Empty.
        }

        let position = read & (N - 1);
        let value = unsafe { *self.data[position].get() };
        self.read.store(read.wrapping_add(1), Ordering::Release);
        Some(value)
    }
}

fn main() {
    let buffer = RingBuffer::<i32>::new();

    thread::scope(|scope| {
        scope.spawn(|| {
            for value in 1..=20 {
                // SAFETY: This is the only producer thread.
                while !unsafe { buffer.push(value) } {
                    thread::yield_now();
                }
                println!("Added: {value}");
            }
        });

        scope.spawn(|| {
            let mut removed = 0;
            while removed < 20 {
                // SAFETY: This is the only consumer thread.
                if let Some(value) = unsafe { buffer.pop() } {
                    assert_eq!(value, removed + 1);
                    println!("Removed: {value}");
                    removed += 1;
                } else {
                    thread::yield_now();
                }
            }
        });
    });
}
