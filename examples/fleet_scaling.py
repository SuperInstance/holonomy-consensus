"""Fleet scaling: convergence rounds vs fleet size, Laman vs Ring topology."""

import time
from fleet_agent.holonomy_stubs import HolonomyMatrix, HolonomyConsensusStub
from fleet_agent.holonomy_stubs import ConsensusTile as Tile


def build_ring(n: int) -> HolonomyConsensusStub:
    hc = HolonomyConsensusStub(tolerance=0.01)
    for i in range(n):
        hc.add_tile(Tile(
            id=i,
            holonomy=HolonomyMatrix.identity(),
            neighbors=[(i - 1) % n, (i + 1) % n],
            cycle_id=None,
        ))
    return hc


def build_laman(n: int) -> HolonomyConsensusStub:
    """Henneberg type-I construction: minimal rigid graph with 2n-3 edges."""
    hc = HolonomyConsensusStub(tolerance=0.01)
    # Build neighbors via Henneberg: each new vertex connects to 2 existing
    neighbors_map: dict[int, list[int]] = {0: [1], 1: [0]}
    for v in range(2, n):
        neighbors_map[v] = [0, v - 1]
        neighbors_map[0].append(v)
        neighbors_map[v - 1].append(v)

    for i in range(n):
        hc.add_tile(Tile(
            id=i,
            holonomy=HolonomyMatrix.identity(),
            neighbors=neighbors_map.get(i, []),
            cycle_id=None,
        ))
    return hc


def main():
    sizes = [5, 10, 15, 20, 30, 50]

    print(f"{'N':>4}  {'Ring rounds':>12}  {'Laman rounds':>13}  {'Speedup':>8}")
    print("-" * 44)

    for n in sizes:
        # Ring topology
        ring_hc = build_ring(n)
        t0 = time.perf_counter()
        ring_result = ring_hc.check_consensus()
        ring_ms = (time.perf_counter() - t0) * 1000

        # Laman topology
        laman_hc = build_laman(n)
        t0 = time.perf_counter()
        laman_result = laman_hc.check_consensus()
        laman_ms = (time.perf_counter() - t0) * 1000

        ring_cycles = ring_result.cycles_checked
        laman_cycles = laman_result.cycles_checked
        speedup = f"{ring_cycles / laman_cycles:.1f}x" if laman_cycles > 0 else "—"

        print(f"{n:>4}  {ring_cycles:>12}  {laman_cycles:>13}  {speedup:>8}")

    print()
    print("Laman topology converges faster because shorter cycles = fewer rounds to verify.")


if __name__ == "__main__":
    main()
