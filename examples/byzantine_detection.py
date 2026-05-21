"""Byzantine detection: locate a malicious agent via cycle bisection."""

from fleet_agent.holonomy_stubs import HolonomyMatrix, HolonomyConsensusStub
from fleet_agent.holonomy_stubs import ConsensusTile as Tile


def main():
    n = 6
    hc = HolonomyConsensusStub(tolerance=0.01)

    for i in range(n):
        neighbors = [(i - 1) % n, (i + 1) % n]
        # Agent 3 is compromised — its transform has a 0.5 radian rotation
        if i == 3:
            matrix = HolonomyMatrix.from_rotation([0.0, 0.0, 1.0], 0.5)
        else:
            matrix = HolonomyMatrix.identity()

        hc.add_tile(Tile(
            id=i,
            holonomy=matrix,
            neighbors=neighbors,
            cycle_id=None,
        ))

    result = hc.check_consensus()
    print(f"Agents:  {n}")
    print(f"Consensus reached: {result.consensus_reached}")
    print(f"Max deviation:     {result.max_deviation:.6f}")
    if result.faulty_tiles:
        print(f"Faulty agents:     {result.faulty_tiles}")
    else:
        print("No faulty agents detected (deviation below tolerance).")


if __name__ == "__main__":
    main()
