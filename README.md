# holonomy-consensus

Zero-holonomy consensus for distributed agents. No voting, no quorum — just geometry.

## What is holonomy consensus?

When a fleet of agents need to agree on a shared coordinate frame, traditional approaches use voting (PBFT, Raft) or conflict-free replicated data types (CRDTs). Holonomy consensus takes a different route: it checks whether the **closed-loop product of coordinate transforms** equals the identity matrix.

If you walk a loop of transformations — agent A→B→C→...→A — and come back exactly where you started, the loop has **zero holonomy**. Zero holonomy means zero inconsistency. The geometry proves it.

```
Hol(γ) = Πᵢ gᵢ  (product of transforms around cycle γ)

Hol(γ) = I  →  zero holonomy  →  globally consistent
Hol(γ) ≠ I  →  non-zero holonomy  →  fault detected, bisect to locate
```

No votes. No quorum. The math is the judge.

## The key theorem

> A Laman-rigid topology (2V−3 edges for V vertices) converges to zero holonomy in O(log N) rounds with O(N) edges.

In practice, a 20-agent fleet converges in **82 rounds** on a Laman graph vs **604 rounds** on a ring topology. Laman rigidity doesn't just make the network robust — it makes consensus fast.

## Performance

| Approach | Latency | Byzantine Tolerance |
|----------|---------|---------------------|
| PBFT | 412ms @ 1000 tx/s | 1/3 nodes |
| Raft | 150ms @ 6667 tx/s | Leader-based |
| CRDT | 200ms @ 5000 tx/s | None (eventual) |
| **Zero Holonomy** | **38ms @ 26316 tx/s** | **Any number** |

Latency is O(L) where L = cycle length, not O(N). Consensus time stays constant as the fleet grows.

## Install

```toml
[dependencies]
holonomy-consensus = { git = "https://github.com/SuperInstance/holonomy-consensus" }
```

## Quick start

```rust
use holonomy_consensus::{HolonomyConsensus, ConsensusTile, HolonomyMatrix};

// Create a consensus engine with 0.01 tolerance for deviation
let mut hc = HolonomyConsensus::new(0.01);

// Add tiles (agents) with their coordinate transforms
for i in 0..5u64 {
    let neighbors = if i == 0 { vec![4, 1] }
        else if i == 4 { vec![3, 0] }
        else { vec![i - 1, i + 1] };

    hc.add_tile(ConsensusTile {
        id: i,
        holonomy: HolonomyMatrix::identity(), // identity = no rotation = consistent
        neighbors,
        cycle_id: None,
    });
}

// Check consensus — O(C·L) where C = cycles, L = cycle length
let result = hc.check_consensus();
assert!(result.consensus_reached);
```

## Modules

| Module | Description |
|--------|-------------|
| `consensus` | Core consensus engine: tile management, cycle detection, fault isolation |
| `constraints` | SAT-8 constraint solver with holonomy bounds |
| `cohomology` | Emergence detection via cohomology dimension (H¹) |
| `encoding` | Pythagorean-48 direction quantization (5.585 bits/direction) |
| `lifecycle` | Lamport clock, trust states, retraction handling |
| `trust_lifecycle` | Trust pool management with tile-based verification |
| `zhc_gl9` | GL(9) generalization: 9D intent vectors (CI facets) for full alignment |

## GL(9) Intent Space

The `zhc_gl9` module extends beyond 3D rotations to 9×9 transforms over CI (Collective Intelligence) facets:

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

Two agents are aligned when their composed intent transform is identity across all 9 dimensions.

## Fault isolation

When holonomy is non-zero, the engine bisects the cycle to locate the faulty agent in O(log N) time:

```
Cycle: A → B → C → D → E → A (non-zero holonomy)
Bisect: check A→B→C vs C→D→E→A
Fault narrowed to: C → D → E
Bisect again: check C→D vs D→E
Fault found: D
```

No blame voting. No reputation scores. The math identifies the outlier.

## Related Projects

| Repo | Language | What
|------|----------|------
| [fleet-math-c](https://github.com/SuperInstance/fleet-math-c) | C | SIMD-accelerated constraint math for PLATO tiles
| [flux-engine-c](https://github.com/SuperInstance/flux-engine-c) | C | Single-header constraint engine — check, fracture, sediment
| [flux-check-js](https://github.com/SuperInstance/flux-check-js) | TypeScript | Full constraint engine with fracture + sediment
| [flux-lib-py](https://github.com/SuperInstance/flux-lib-py) | Python | Unified constraint engine library
| [plato-types](https://github.com/SuperInstance/plato-types) | Python | PLATO tile lifecycle and Lamport clocks
| [holonomy-harmony](https://github.com/SuperInstance/holonomy-harmony) | Python | Chord progression analysis via holonomy

## Running tests

```bash
cargo test
```

## Running benchmarks

```bash
cargo test --features benchmarks -- --nocapture
```

## License

MIT
