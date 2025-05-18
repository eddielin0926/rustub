# 🧭 Roadmap

This document provides a high-level overview of **RusTub** development.

## 🎯 MVP Goals (Core Engine)

> Goal: Build a functional single-node database engine with minimal features.

- [ ] `DiskManager`: read/write fixed-size pages to single file
- [ ] `BufferPoolManager`: page caching with LRU eviction
- [ ] Page abstraction with `PageId`, `PAGE_SIZE`, etc.
- [ ] `TableHeap`: basic tuple storage
- [ ] Tuple and schema definitions
- [ ] Volcano-style executor framework
- [ ] Simple query support (e.g., scan, insert)

## 🗂️ Planned Workspaces

| Workspace     | Purpose                                                     | Status                                                                 |
| ------------- | ----------------------------------------------------------- | ---------------------------------------------------------------------- |
| `core`        | Core database engine: storage, execution, memory management | ![status](https://img.shields.io/badge/status-in--progress-blue)       |
| `cli`         | Interactive shell / REPL for running queries                | ![status](https://img.shields.io/badge/status-planned-yellow)          |
| `bench`       | Microbenchmarks and performance tests                       | ![status](https://img.shields.io/badge/status-planned-yellow)          |
| `distributed` | Raft, sharding, and replication                             | ![status](https://img.shields.io/badge/status-stretch--goal-lightgrey) |

## 📌 Future Features

### Logging & Recovery
- [ ] WAL (Write-Ahead Logging)
- [ ] Dedicated `LogManager` for log buffering and flushing
- [ ] Recovery logic (REDO/UNDO)

### Concurrency
- [ ] `Transaction` abstraction
- [ ] Lock manager (2PL or MVCC)
- [ ] Transaction isolation levels

### Query & Storage
- [ ] B+ Tree index
- [ ] Join algorithms (nested loop, hash join)
- [ ] Cost-based query optimizer

### Engine Internals
- [ ] Table catalog and metadata
- [ ] Asynchronous I/O support
- [ ] Disk I/O scheduler

## 🧠 Deferred Design Ideas

These ideas are not part of the MVP, but may be revisited in later stages:

- Modular separation of disk/log managers
- Lock-free structures (e.g., concurrent page table)
- Pluggable storage engines (row vs. column)
- WASM-based execution (browser environment)
- Distributed execution and 2PC
- Raft-based replication and leader election
- Multi-file storage backend

## 📝 Notes

- This project is under active development and intended primarily for learning.
- Feedback, suggestions, and contributions are welcome once the `core` is more stable.
