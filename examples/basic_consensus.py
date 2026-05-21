"""Basic consensus: 5 agents reach zero-holonomy agreement."""

# pip install holonomy-consensus  (or: from the Rust crate via PyO3)
#
# This example uses the Python stubs that ship with the companion
# `fleet-agent` package.  In production you'd use the compiled Rust
# bindings for full performance.

from fleet_agent.holonomy_stubs import HolonomyMatrix, HolonomyConsensusStub
from fleet_agent.holonomy_stubs import ConsensusTile as Tile


def main():
    # 5 agents arranged in a ring.  Each transform is the identity
    # (no rotation) → every cycle product is identity → consensus.
    n = 5
    hc = HolonomyConsensusStub(tolerance=0.01)

    for i in range(n):
        neighbors = [(i - 1) % n, (i + 1) % n]
        hc.add_tile(Tile(
            id=i,
            holonomy=HolonomyMatrix.identity(),
            neighbors=neighbors,
            cycle_id=None,
        ))

    result = hc.check_consensus()
    print(f"Agents:  {n}")
    print(f"Cycles:  {result.cycles_checked}")
    print(f"Consensus reached: {result.consensus_reached}")
    print(f"Max deviation:     {result.max_deviation:.6f}")


if __name__ == "__main__":
    main()
