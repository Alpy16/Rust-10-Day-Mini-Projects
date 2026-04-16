# 🦀 10 Days of Rust Systems & Memory

This repository is a dedicated 10-day sprint to master the core of Rust: **Memory Management.** After building several functional projects (CLI tools, games, and persistent apps), I realized that the true power of Rust lies in its ability to handle data with surgical precision. This sprint is about moving away from "making it work" and moving toward "making it optimal."

## 🎯 The Mission
To eliminate the "Borrow Checker friction" and build a cemented mental model for how data lives, moves, and dies in a Rust program.

## 🛠️ The Constraints
1. **Zero `.clone()`:** Data must be moved or borrowed. Cloning is treated as a last resort.
2. **Explicit Lifetimes:** No dodging compiler errors with `static` or unnecessary heap allocations.
3. **Performance First:** Favor stack allocation and references over heap-managed types where possible.

---

## 📅 The Sprint Log

| Day | Project | The Challenge | Technical Focus |
|:--- |:--- |:--- |:--- |
| **01** | [Hot Potato](hot_potato) | Ownership Handoff | Move semantics & Return-by-Value |
| **02** | [Borrow Ledger](bank_account) | The Reference Limit | Mutable vs Immutable scope control |
| **03** | [Slice Parser](slice_parser) | Non-owning Views | String slices (`&str`) & Memory layout |
| **04** | [Boxed List](recursive_box) | Heap Recursion | Recursive types & `Box<T>` |
| **05** | [Shared State](./day05) | Multi-owner Data | `Rc<T>` and the limits of single-thread sharing |
| **06** | [Lifetimes](./day06) | Scope Validation | Lifetime annotations (`'a`) |
| **07** | [Interior Mutability](./day07) | Runtime Checking | `RefCell` and the "Escape Hatch" |
| **08** | [Zero-Copy Parser](./day08) | Zero-Allocation | Building structs that "borrow" their fields |
| **09** | [Unsafe Peek](./day09) | Raw Pointers | Memory addresses & Byte-level inspection |
| **10** | **Final: Optimized KV** | Systems Integration | Applying all concepts to a high-speed store |

---


**GitHub:** [Alpy16](https://github.com/Alpy16)