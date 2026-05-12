# holonomy-consensus

**Zero-holonomy consensus for distributed constraint checking — geometric agreement without voting.**

[![CI](https://github.com/SuperInstance/holonomy-consensus/actions/workflows/ci.yml/badge.svg)](https://github.com/SuperInstance/holonomy-consensus/actions/workflows/ci.yml)
[![crates.io](https://img.shields.io/crates/v/holonomy-consensus.svg)](https://crates.io/crates/holonomy-consensus)
[![License: Apache-2.0](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE)

> Part of the [SuperInstance](https://github.com/SuperInstance) constraint theory ecosystem.

---

## The Problem

Traditional distributed consensus — PBFT, Raft, CRDTs — reaches agreement through **voting**. Nodes exchange messages, count quorums, and decide by majority. This is slow (412ms for PBFT), fragile (only tolerates ⅓ Byzantine faults), and wasteful (O(N²) message complexity).

What if you could know the entire network is consistent by checking a **single geometric property**?

## The Insight: Zero Holonomy = Consensus

In differential geometry, **holonomy** measures how a vector changes after being parallel-transported around a closed loop. If the loop returns the vector unchanged, the holonomy is trivial (zero).

**Applied to consensus:** Each agent in a network applies a transformation to incoming data. If you compose all transformations around any cycle and get the identity matrix, the entire network is globally consistent.

```
Hol(γ) = Πᵢ Tᵢ    (product of transforms around cycle γ)

Hol(γ) = I  →  Consistent (zero holonomy)
Hol(γ) ≠ I  →  Inconsistent, locate fault by cycle bisection in O(log N)
```

No voting. No quorums. No leader election. Just math.

## Performance

| Approach | Latency | Message Complexity | Fault Tolerance |
|----------|---------|--------------------|-----------------|
| PBFT | ~412ms | O(N²) | ⅓ Byzantine |
| Raft | ~150ms | O(N) | 0 Byzantine |
| CRDT | ~200ms | O(N) | N-1 (eventual) |
| **Zero Holonomy** | **~38ms** | **O(C·L)** | **Any** (geometric check) |

Holonomy consensus doesn't replace BFT consensus — it's a **structural consistency check** that detects when agent transforms have drifted from agreement. It's complementary to, not a substitute for, crash-fault consensus protocols.

## Architecture

### Two Holonomy Spaces

The library provides consensus checking in two spaces:

1. **SO(3) — 3D Rotation Matrices** (`consensus` module): Lightweight holonomy via 3×3 rotation matrices. Fast, but 3D projection can lose information about multi-dimensional intent.

2. **GL(9) — General Linear Group** (`zhc_gl9` module): Full 9-dimensional intent space. Each dimension is a CI (Collective Intelligence) facet:

   | Index | CI Facet | Description |
   |-------|----------|-------------|
   | 0 | C1 Boundary | System boundaries and scope |
   | 1 | C2 Pattern | Recognized patterns |
   | 2 | C3 Process | Process models |
   | 3 | C4 Knowledge | Knowledge structures |
   | 4 | C5 Social | Social dynamics |
   | 5 | C6 Deep Structure | Underlying structures |
   | 6 | C7 Instrument | Instruments and tools |
   | 7 | C8 Paradigm | Paradigmatic frameworks |
   | 8 | C9 Stakes | Stakes and values |

   GL(9) preserves the full intent structure. Deep-dive experiments showed that 3D projection **destroys** correlation (r = −0.045) between holonomy and alignment; the 9D version maintains meaningful correlation.

### Modules

| Module | Description |
|--------|-------------|
| [`consensus`](src/consensus.rs) | SO(3) holonomy consensus engine — 3×3 rotation matrices, O(1) tile lookup, O(log L) fault location |
| [`zhc_gl9`](src/zhc_gl9.rs) | GL(9) zero holonomy consensus — 9×9 intent transforms, alignment scoring, Pearson correlation |
| [`constraints`](src/constraints.rs) | INT8-saturated constraint boundaries — certifiable bound checking (Coq-proven) |
| [`cohomology`](src/cohomology.rs) | Sheaf cohomology emergence detection — H¹ dimension detects emergent patterns |
| [`encoding`](src/encoding.rs) | Pythagorean vector encoding — 48 exact directions, 5.585 bits/vector, zero drift |
| [`benchmarks`](src/benchmarks.rs) | Comparative benchmarks vs PBFT, Raft, CRDT |

## Installation

Add to your `Cargo.toml`:

```toml
[dependencies]
holonomy-consensus = "0.1"
```

Or via cargo:

```bash
cargo add holonomy-consensus
```

## Quick Start

### SO(3) Consensus

```rust
use holonomy_consensus::consensus::{
    HolonomyConsensus, ConsensusTile, HolonomyMatrix,
};

fn main() {
    let mut engine = HolonomyConsensus::new(1e-6);

    // Add tiles (agents) with rotation transforms
    for i in 0..5u64 {
        let neighbors = if i == 0 {
            vec![4, 1]
        } else if i == 4 {
            vec![3, 0]
        } else {
            vec![i - 1, i + 1]
        };

        engine.add_tile(ConsensusTile {
            id: i,
            holonomy: HolonomyMatrix::identity(),
            neighbors,
            cycle_id: None,
        });
    }

    let result = engine.check_consensus();
    println!("Consistent: {}", result.is_consistent);       // true
    println!("Deviation:  {}", result.deviation);            // ~0.0
    println!("Faulty:     {:?}", result.faulty_tile);        // None
}
```

### GL(9) Consensus (Full Intent Space)

```rust
use holonomy_consensus::zhc_gl9::{
    GL9HolonomyConsensus, GL9Agent, GL9Matrix, IntentVector,
};

fn main() {
    let mut engine = GL9HolonomyConsensus::with_default_tolerance();

    // 4 agents in a ring, each rotating π/2 in the C1–C2 plane
    // Total rotation = 4 × π/2 = 2π → identity → zero holonomy
    for i in 0..4u64 {
        let neighbors = if i == 0 {
            vec![3, 1]
        } else {
            vec![i - 1, (i + 1) % 4]
        };

        engine.add_agent(GL9Agent {
            id: i,
            transform: GL9Matrix::plane_rotation(0, 1, std::f64::consts::FRAC_PI_2),
            intent: IntentVector::unit(0),
            neighbors,
        });
    }

    let result = engine.check_consensus();
    assert!(result.is_consistent);  // Full rotation = zero holonomy

    // Check alignment across agents
    let alignment = engine.compute_alignment();
    println!("Fleet alignment: {:.3}", alignment);  // 0.0–1.0
}
```

### INT8 Constraint Checking

```rust
use holonomy_consensus::constraints::{HolonomyBounds, ConstraintResult};

fn main() {
    let bounds = HolonomyBounds::default();

    // Check a single deviation
    let result = ConstraintResult::check(0.005, &bounds);
    assert!(result.pass);
    println!("Deviation: {} (scaled ×1000)", result.deviation);

    // Batch check
    let deviations = [0.001, 0.005, 0.015, 0.5];
    let results = ConstraintResult::check_batch(&deviations, &bounds);
    for (i, r) in results.iter().enumerate() {
        println!("  [{}] pass={} mask={:#04x} dev={}",
            i, r.pass, r.error_mask, r.deviation);
    }
}
```

### Emergence Detection (Sheaf Cohomology)

```rust
use holonomy_consensus::cohomology::EmergenceDetector;

fn main() {
    // 1024 agents, 12000 connections, 1 connected component
    let result = EmergenceDetector::detect(1024, 12000, 1);

    if result.emergence_detected {
        println!("Emergent patterns: {} independent cycles (H¹ = {})",
            result.h1, result.h1);
    }

    // From an explicit edge list
    let vertices = vec![1, 2, 3, 4, 5];
    let edges = vec![(1, 2), (2, 3), (3, 4), (4, 5), (5, 1)];
    let result = EmergenceDetector::from_edge_list(&vertices, &edges);
    println!("H⁰ = {} (components), H¹ = {} (cycles)",
        result.h0, result.h1);
}
```

### Pythagorean Encoding (Zero-Drift Vectors)

```rust
use holonomy_consensus::encoding::{Pythagorean48, Vector48};

fn main() {
    // Encode a direction to one of 48 exact unit vectors (6 bits)
    let encoded = Pythagorean48::encode(0.6, 0.8);
    let (x, y) = Pythagorean48::decode(encoded);

    // Key property: zero drift after arbitrary hops
    // f32 accumulates 17° drift after 1000 hops;
    // Pythagorean48 is bit-identical forever.
    println!("Encoded ({:.3}, {:.3}) → ({:.3}, {:.3})", 0.6, 0.8, x, y);
    println!("Information: {:.3} bits/vector", Pythagorean48::BITS_PER_VECTOR);
}
```

## Test Coverage

The test suite covers:

- **INT8 saturation:** identity, clamping, negation symmetry, monotonicity, closure
- **Constraint checking:** pass/fail, saturation warnings, batch processing
- **SO(3) consensus:** identity loops, cycle holonomy computation
- **GL(9) consensus:** identity loops, plane rotations, rotation loops (2π = identity), partial rotations (nonzero holonomy), tolerance thresholds
- **Intent vectors:** normalization, cosine similarity, orthogonality, transform application
- **Determinants:** identity (1.0), rotations (1.0)
- **9D vs 3D information preservation:** proves GL(9) distinguishes vectors that project identically in 3D
- **Holonomy–alignment correlation:** validates that 9D preserves the correlation that 3D destroys
- **Pearson correlation:** perfect positive, perfect negative
- **Emergence detection:** flock formation, rigid fleet (Laman's theorem)
- **Benchmarks:** PBFT, Raft, CRDT, holonomy — latency, throughput, memory, Byzantine tolerance

Run all tests:

```bash
cargo test
```

Run benchmarks (prints comparative results):

```bash
cargo test --release -- benchmark_ --nocapture
```

## Why It Works

### Mathematical Foundation

1. **Holonomy** is a well-studied concept from differential geometry. A cycle with zero holonomy is globally consistent by definition — this is a theorem, not a heuristic.

2. **Fault isolation** uses cycle bisection: split a failing cycle in half, check which half has nonzero holonomy, recurse. O(log N) to find the faulty agent.

3. **INT8 bounds** make the system certifiable. All arithmetic uses saturated INT8 ([-127, 127]), which is proven correct in Coq (7 theorems). This provides a DO-178C DAL A certification path.

4. **Pythagorean encoding** uses the 48 exact unit vectors representable with 16-bit integer numerators. Unlike floating-point, these directions never accumulate rounding error — bit-identical after unlimited hops.

5. **Sheaf cohomology** (H¹) detects emergent patterns in agent networks. H¹ > 0 means there are independent cycles — emergent structure that no individual agent sees. This replaces 12,000-line ML pipelines with a single formula: H¹ = E − V + H⁰.

### Relationship to Laman's Theorem

The default `HolonomyBounds` uses `min_agreement = 7` based on Laman's theorem for rigidity: a graph with V vertices needs E = 2V − 3 edges to be rigid. For V = 12 neighbors, that means 7 agreeing edges for rigidity. This isn't coincidence — the constraint theory fleet is built on the same geometric rigidity that makes holonomy consensus work.

## Ecosystem

| Crate | Purpose | Repository |
|-------|---------|------------|
| **holonomy-consensus** | Zero-holonomy consensus, constraint checking, emergence detection | [holonomy-consensus](https://github.com/SuperInstance/holonomy-consensus) |
| constraint-theory-core | Core constraint theory primitives (INT8 saturation, tile operations) | [constraint-theory-core](https://github.com/SuperInstance/constraint-theory-core) |
| dodecet-encoder | Dodecet (12-tuple) encoding for fleet coordinate compression | [dodecet-encoder](https://github.com/SuperInstance/dodecet-encoder) |
| fleet-coordinate | Fleet-wide coordinate system and agent positioning | [fleet-coordinate](https://github.com/SuperInstance/fleet-coordinate) |

## License

Licensed under the [Apache License 2.0](LICENSE).

---

*Built by [SuperInstance](https://github.com/SuperInstance) for the Cocapn constraint theory fleet.*
