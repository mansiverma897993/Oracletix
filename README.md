# Oracletix Protocol

Oracletix is a Solana-based protocol for tokenizing real-world events as dynamic, programmable assets on-chain.

It introduces a new primitive where events are represented as evolving on-chain objects that reflect real-world state transitions such as creation, live execution, and completion.

---

## Overview

Oracletix enables developers and organizers to represent physical experiences as verifiable digital assets. Each event exists as an on-chain account and can be linked to an NFT that evolves over time.

Instead of static tickets or proof-of-attendance tokens, Oracletix models events as stateful objects that update as the real-world event progresses.

---

## Core Concepts

### Event

An Event is the core on-chain object. It represents a real-world occurrence with a lifecycle.

Each event contains:

* Organizer
* Name
* Start and end time
* State (Created, Live, Completed)
* Linked NFT mint
* Attendance count

---

### Event State Machine

Events follow a simple lifecycle:

* Created → Initial state after deployment
* Live → Event is ongoing
* Completed → Event has finished

State transitions are enforced on-chain.

---

### Dynamic NFT Linkage

Each event can be linked to an NFT mint. This allows external systems to attach dynamic metadata that reflects the event’s current state.

The protocol does not enforce metadata logic directly, making it composable with off-chain or hybrid systems.

---

### Attendance Tracking

The protocol supports attendance recording during the live phase of an event.

Attendance is stored as a simple counter and can be extended with signature-based or oracle-based verification systems.

---

## Architecture

Oracletix is designed as a minimal on-chain protocol with extensibility in mind.

* On-chain: Event state, lifecycle, authority
* Off-chain: Metadata rendering, oracle inputs, UI
* Optional: Decentralized attestations and oracle networks

---

## Program Instructions

### initialize_event

Creates a new event account.

Inputs:

* name (string)
* start_time (i64)
* end_time (i64)

---

### attach_mint

Associates an NFT mint with the event.

Inputs:

* mint (Pubkey)

---

### update_state

Updates the lifecycle state of the event.

Inputs:

* next_state (u8)

---

### mark_attendance

Increments attendance for a live event.

---

## Data Structure

Event account layout:

* organizer: Pubkey
* name: String
* start_time: i64
* end_time: i64
* state: u8
* mint: Pubkey
* attendance: u32
* bump: u8

---

## Getting Started

### Prerequisites

* Solana CLI
* Anchor Framework
* Rust

---

### Setup

```bash
anchor init oracletix
cd oracletix
```

---

### Build

```bash
anchor build
```

---

### Deploy

```bash
anchor deploy
```

---

## Design Philosophy

Oracletix keeps the on-chain layer minimal and deterministic while allowing flexibility off-chain.

The protocol focuses on:

* Clear state transitions
* Composability with NFT systems
* Extensibility for oracle-driven updates
* Lightweight event representation

---

## Future Extensions

* Native NFT minting via CPI
* Dynamic metadata standards
* Oracle-based state updates
* Multi-attestation attendance verification
* Reward and access control layers
* Event-based identity systems

---

## License

MIT
