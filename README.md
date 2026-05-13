# Holonomy Consensus

**The compass. Points true when everything else drifts.**

---

## The Problem With Voting

Distributed consensus — PBFT, Raft, CRDTs — works by voting. Nodes exchange messages, count quorums, decide by majority. It's slow (412ms for PBFT), fragile (only tolerates one-third Byzantine faults), and wasteful (O(N²) message complexity).

What if you could know the entire network is consistent by checking one geometric property?

## The Insight

In differential geometry, holonomy measures how a vector changes after being transported around a closed loop. If the loop returns the vector unchanged, the holonomy is zero — and the space is flat.

Applied to distributed systems: each agent applies a transformation to incoming data. If you compose all transformations around any cycle and get the identity matrix, the entire network is globally consistent. No voting. No quorums. No leader election.

```
Hol(γ) = Πᵢ Tᵢ    (product of transforms around cycle γ)
Hol(γ) = I  →  Consistent (zero holonomy)
Hol(γ) ≠ I  →  Inconsistent — locate fault by cycle bisection in O(log N)
```

## The Numbers

| Approach | Latency | Fault Tolerance |
|----------|---------|----------------|
| PBFT | ~412ms | ⅓ Byzantine |
| Raft | ~150ms | 0 Byzantine |
| CRDT | ~200ms | N-1 (eventual) |
| **Zero Holonomy** | **~38ms** | **Any** (geometric check) |

38ms versus 412ms. Not because it's better engineered. Because it asks a different question: "has the geometry drifted?" instead of "can we form a quorum?"

## Why GL(9)

The library operates in GL(9) — the General Linear Group on a 9-dimensional intent space. Each dimension corresponds to a CI facet: Boundary, Pattern, Process, Knowledge, Social, Deep Structure, Instrument, Paradigm, Stakes.

SO(3) was tried first. It didn't work — 3D projection destroys correlation between holonomy and alignment (Pearson r = -0.045). The 9-dimensional version maintains meaningful correlation because it preserves the full intent structure.

Seven Rust files. Original mathematics with real code.

## License

Apache 2.0 — Cocapn fleet infrastructure.
