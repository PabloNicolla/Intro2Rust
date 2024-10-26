# Fence

- [Fence](#fence)
  - [Introduction](#introduction)
  - [Types of Fences](#types-of-fences)
  - [Why Fences are Useful](#why-fences-are-useful)
  - [Example Use Case](#example-use-case)

## Introduction

In multithreading, fences (also called memory barriers) are synchronization mechanisms that control the ordering of memory operations (reads and writes) in code. Fences are used to ensure that certain instructions are completed before others, which can be essential in multi-core systems where each core may have its own cache and execute instructions out of order for performance reasons.

## Types of Fences

There are different types of fences, each providing different levels of ordering:

- Acquire Fence: Prevents memory reads/writes before the fence from being reordered after the fence. It ensures that any read or write operation after the acquire fence sees the effects of operations before the fence.

- Release Fence: Ensures that all reads/writes before the fence complete before any reads/writes after the fence. It prevents memory operations after the fence from being reordered before it.

- Full Fence: Ensures both acquire and release semantics, meaning all preceding operations complete before the fence, and all subsequent operations are delayed until after the fence. It provides the strongest ordering guarantees.

## Why Fences are Useful

Without fences, the compiler and the CPU may reorder instructions to optimize performance, which can lead to inconsistent or unexpected behavior in concurrent programs. For example, in a program where one thread writes to a variable and another thread reads from it, a memory fence can guarantee that the changes made by one thread are visible to the other in a specific order.

## Example Use Case

Imagine you’re implementing a flag to signal between threads. Without a fence, the thread waiting on the flag might read an outdated value because the write operation could still be sitting in a cache or buffer. With an appropriate memory fence, you ensure that all threads see the update at the correct time.

Fences are often paired with atomic operations, which are indivisible actions that can’t be interrupted, providing further guarantees about memory visibility between threads.
