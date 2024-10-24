# Spin

- [Spin](#spin)
  - [Introduction](#introduction)
  - [Spin for a Short Duration (Busy Wait)](#spin-for-a-short-duration-busy-wait)
  - [Backoff Mechanism (Adaptive)](#backoff-mechanism-adaptive)
  - [Yielding to the OS (Scheduler-Aware)](#yielding-to-the-os-scheduler-aware)
  - [Park/Unpark (Futex) Mechanism](#parkunpark-futex-mechanism)
  - [Summary](#summary)

## Introduction

Mutexes that implement spinning without using 100% CPU usually employ a technique called adaptive spinning or yielding, which intelligently decides when to continue spinning and when to yield to the OS scheduler to avoid burning CPU cycles unnecessarily.

how it works:

## Spin for a Short Duration (Busy Wait)

Initially, the mutex will spin for a short period, repeatedly checking if the lock is available. This is useful when the lock is expected to become available soon, as switching to another thread (context switching) can be more expensive than waiting for the lock.

- Busy-waiting: During this short period, the mutex checks the lock state in a loop, but instead of going full throttle, it may include a small delay or some kind of relaxation, like using pause (on x86 architectures) to hint to the processor that it's in a spin loop.

## Backoff Mechanism (Adaptive)

If the lock isn't acquired after a few iterations, the mutex may implement an exponential backoff strategy, where it waits longer between subsequent checks. This helps reduce the chance of burning CPU while waiting.

- Exponential backoff: After a few spins, it doubles the wait time each time, which increases the gap between checks for the lock, further reducing CPU usage.

## Yielding to the OS (Scheduler-Aware)

If the lock is still not available after the backoff, the mutex may yield the thread to the operating system scheduler. The yield allows the OS to schedule another thread, freeing up CPU resources. This is done using system calls like sched_yield() (on Linux) or similar, which tells the scheduler to give another thread or process a turn.

- Thread yielding: The thread explicitly gives up the CPU for a short period, allowing other processes or threads to run, preventing the spinning thread from consuming all the CPU.

## Park/Unpark (Futex) Mechanism

More advanced mutex implementations (e.g., Linux's futex or Rust's parking_lot) may use a futex (fast userspace mutex) mechanism. Here, the mutex spins for a while in userspace but, after a certain number of spins or time, it will ask the kernel to "park" the thread (put it to sleep). When the lock becomes available, another thread can "unpark" the waiting thread, waking it up.

- Futex (Fast Userspace Mutex): It spins in userspace and only resorts to kernel-level sleeping if contention remains high.

## Summary

A mutex implementing spin locking without using 100% CPU relies on:

- Short initial spins to avoid the overhead of context switching.
- Adaptive backoff to reduce spin frequency over time.
- Yielding to the OS scheduler to allow other threads to execute.
- Optionally, futexes or park/unpark mechanisms to put threads to sleep when contention is too high.

This combination allows the system to balance waiting and performance while avoiding the waste of CPU resources.
