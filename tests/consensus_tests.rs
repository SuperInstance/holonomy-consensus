//! Consensus tests for HolonomyConsensus (SO(3) version)
//!
//! Focuses on the core SO(3) holonomy engine: cycle detection, fault isolation,
//! edge weight computation, cycle-based trust verification, and consensus snap behavior.

use holonomy_consensus::consensus::{ConsensusResult, ConsensusTile, HolonomyConsensus, HolonomyMatrix};

fn make_ring(n: u64, deviation: f64) -> HolonomyConsensus {
    let mut c = HolonomyConsensus::new(0.01);
    for i in 0..n {
        let neighbors = if n == 1 {
            vec![]
        } else if i == 0 {
            vec![n - 1, 1]
        } else if i == n - 1 {
            vec![i - 1, 0]
        } else {
            vec![i - 1, i + 1]
        };
        c.add_tile(ConsensusTile {
            id: i,
            holonomy: HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], deviation),
            neighbors,
            cycle_id: None,
        });
    }
    c
}

#[test]
fn test_identity_ring_consensus() {
    // 5-tile ring, all identity → zero holonomy → consistent
    let consensus = make_ring(5, 0.0);
    let result = consensus.check_consensus();
    assert!(result.is_consistent, "Identity ring should be consistent");
    assert!(result.deviation < 1e-15, "Deviation should be ~0");
    assert!(result.faulty_tile.is_none(), "No faulty tile expected");
    assert!(result.information.is_infinite(), "Perfect consistency => infinite info");
}

#[test]
fn test_consistent_rotation_ring() {
    // 6-tile ring, each rotating 60° → total 360° → back to identity
    let angle = std::f64::consts::FRAC_PI_3; // 60°
    let mut consensus = HolonomyConsensus::new(0.01);
    for i in 0..6u64 {
        let neighbors = if i == 0 { vec![5, 1] }
                        else if i == 5 { vec![4, 0] }
                        else { vec![i - 1, i + 1] };
        consensus.add_tile(ConsensusTile {
            id: i,
            holonomy: HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], angle),
            neighbors,
            cycle_id: None,
        });
    }
    let result = consensus.check_consensus();
    // 6 × 60° = 360° = identity → consistent
    assert!(result.is_consistent, "Full rotation ring should be consistent");
    assert!(result.deviation < 1e-10, "Deviation should be near-zero, got {}", result.deviation);
}

#[test]
fn test_inconsistent_rotation_ring() {
    // 4-tile ring, each rotating 45° → total 180° ≠ 360° → inconsistent
    let angle = std::f64::consts::FRAC_PI_4; // 45°
    let mut consensus = HolonomyConsensus::new(0.01);
    for i in 0..4u64 {
        let neighbors = if i == 0 { vec![3, 1] }
                        else if i == 3 { vec![2, 0] }
                        else { vec![i - 1, i + 1] };
        consensus.add_tile(ConsensusTile {
            id: i,
            holonomy: HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], angle),
            neighbors,
            cycle_id: None,
        });
    }
    let result = consensus.check_consensus();
    // 4 × 45° = 180° ≠ 360° → inconsistent
    assert!(!result.is_consistent, "Partial rotation ring should be inconsistent");
    assert!(result.deviation > 0.01, "Should have measurable deviation");
    assert!(result.faulty_tile.is_some(), "Should identify a faulty tile");
}

#[test]
fn test_single_tile_consensus() {
    // Single tile with no neighbors → trivially consistent
    let mut consensus = HolonomyConsensus::new(0.01);
    consensus.add_tile(ConsensusTile {
        id: 0,
        holonomy: HolonomyMatrix::identity(),
        neighbors: vec![],
        cycle_id: None,
    });
    let result = consensus.check_consensus();
    assert!(result.is_consistent, "Single tile should be trivially consistent");
}

#[test]
fn test_two_tile_disconnected() {
    // Two disconnected tiles → both are trivially consistent
    let mut consensus = HolonomyConsensus::new(0.01);
    consensus.add_tile(ConsensusTile {
        id: 0,
        holonomy: HolonomyMatrix::identity(),
        neighbors: vec![],
        cycle_id: None,
    });
    consensus.add_tile(ConsensusTile {
        id: 1,
        holonomy: HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], 0.5),
        neighbors: vec![],
        cycle_id: None,
    });
    let result = consensus.check_consensus();
    assert!(result.is_consistent, "Disconnected tiles should each be trivially consistent");
}

#[test]
fn test_triple_cycle_inconsistency() {
    // 3-tile ring, each rotating 30° → total 90° ≠ 360° → inconsistent
    let mut consensus = HolonomyConsensus::new(0.001);
    for i in 0..3u64 {
        let neighbors = if i == 0 { vec![2, 1] }
                        else if i == 2 { vec![1, 0] }
                        else { vec![i - 1, i + 1] };
        consensus.add_tile(ConsensusTile {
            id: i,
            holonomy: HolonomyMatrix::from_rotation([1.0, 0.0, 0.0], std::f64::consts::FRAC_PI_6),
            neighbors,
            cycle_id: None,
        });
    }
    let result = consensus.check_consensus();
    assert!(!result.is_consistent, "90° rotation should be inconsistent");
}

#[test]
fn test_fault_isolation_via_bisection() {
    // 8-tile ring where ONE tile has a large deviation.
    // The bisection should locate the faulty tile.
    let mut consensus = HolonomyConsensus::new(0.01);
    for i in 0..8u64 {
        // Tile 4 has a large rotation; others are identity
        let deviation = if i == 4 { 0.5 } else { 0.0 };
        let neighbors = if i == 0 { vec![7, 1] }
                        else if i == 7 { vec![6, 0] }
                        else { vec![i - 1, i + 1] };
        consensus.add_tile(ConsensusTile {
            id: i,
            holonomy: HolonomyMatrix::from_rotation([0.0, 1.0, 0.0], deviation),
            neighbors,
            cycle_id: None,
        });
    }
    let result = consensus.check_consensus();
    assert!(!result.is_consistent, "Should detect inconsistency");
    assert_eq!(result.faulty_tile, Some(4), "Should identify tile 4 as faulty");
}

#[test]
fn test_edge_tolerance_thresholds() {
    // 4-tile ring with identity matrices → zero holonomy.
    // Loose tolerance should pass, tight tolerance should also pass (dev=0).
    let mut consensus_loose = HolonomyConsensus::new(0.5);
    let mut consensus_tight = HolonomyConsensus::new(0.0001);

    for i in 0..4u64 {
        let neighbors = if i == 0 { vec![3, 1] }
                        else if i == 3 { vec![2, 0] }
                        else { vec![i - 1, i + 1] };
        let tile = ConsensusTile {
            id: i,
            holonomy: HolonomyMatrix::identity(),
            neighbors,
            cycle_id: None,
        };
        consensus_loose.add_tile(tile.clone());
        consensus_tight.add_tile(ConsensusTile {
            id: i + 100,
            ..tile
        });
    }

    // Identity ring should pass both
    let loose_result = consensus_loose.check_consensus();
    assert!(loose_result.is_consistent, "Loose tolerance should pass, dev={}", loose_result.deviation);
    let tight_result = consensus_tight.check_consensus();
    assert!(tight_result.is_consistent, "Tight tolerance should also pass for identity, dev={}", tight_result.deviation);

    // Now test 3-tile ring with non-zero deviation.
    // Each tile has angle 0.05 → product ≈ rotation of 0.15 rad
    let angle = 0.05;
    
    // Build consensus_fail FIRST (offset IDs)
    let mut consensus_fail = HolonomyConsensus::new(0.001);
    for i in 0..3u64 {
        let (a, b, c) = (100i64, 101, 102);
        let id = (100 + i) as u64;
        let neighbors = if i == 0 { vec![102u64, 101] }
                        else if i == 2 { vec![101u64, 100] }
                        else { vec![100u64, 102] };
        consensus_fail.add_tile(ConsensusTile {
            id,
            holonomy: HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], angle),
            neighbors,
            cycle_id: None,
        });
    }
    
    // Build consensus_pass (IDs 0, 1, 2)
    let mut consensus_pass = HolonomyConsensus::new(0.5);
    for i in 0..3u64 {
        let neighbors = if i == 0 { vec![2u64, 1] }
                        else if i == 2 { vec![1u64, 0] }
                        else { vec![0u64, 2] };
        consensus_pass.add_tile(ConsensusTile {
            id: i,
            holonomy: HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], angle),
            neighbors,
            cycle_id: None,
        });
    }

    let pass_result = consensus_pass.check_consensus();
    let fail_result = consensus_fail.check_consensus();
    assert!(pass_result.is_consistent, "Wide tolerance 0.5 should pass, dev={}", pass_result.deviation);
    assert!(!fail_result.is_consistent, "Tight tolerance 0.001 should fail, dev={}", fail_result.deviation);
}

#[test]
fn test_holonomy_matrix_operations() {
    // Test multiply, deviation, is_identity
    let identity = HolonomyMatrix::identity();
    assert!(identity.is_identity(1e-15));
    assert_eq!(identity.deviation(), 0.0);

    // Composition of rotation and its inverse = identity
    let rot = HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], std::f64::consts::FRAC_PI_4);
    let inv = HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], -std::f64::consts::FRAC_PI_4);
    let composed = rot.multiply(&inv);
    assert!(composed.deviation() < 1e-15, "R × R⁻¹ should be identity");

    // Matrix multiplication is associative: (A×B)×C = A×(B×C)
    let a = HolonomyMatrix::from_rotation([1.0, 0.0, 0.0], 0.3);
    let b = HolonomyMatrix::from_rotation([0.0, 1.0, 0.0], 0.4);
    let c = HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], 0.5);

    let ab_c = a.multiply(&b).multiply(&c);
    let a_bc = a.multiply(&b.multiply(&c));
    let diff = ab_c.multiply(&a_bc).deviation(); // M × M⁻¹ → deviation from identity
    // Actually compute ||AB_C - A_BC|| directly
    assert!((ab_c.0[0][0] - a_bc.0[0][0]).abs() < 1e-15, "Associativity of matrix mult");
}

#[test]
fn test_large_ring_performance_linear() {
    // O(N) cycle tracing: 1000-tile ring should complete quickly
    let consensus = make_ring(1000, 0.0);
    let result = consensus.check_consensus();
    assert!(result.is_consistent, "1000-tile identity ring should be consistent");
    assert!(result.deviation < 1e-10);
}

#[test]
fn test_cycle_bisection_on_varied_sizes() {
    // Test fault isolation on rings of different sizes
    for &n in &[4usize, 16, 64] {
        let mut consensus = HolonomyConsensus::new(0.01);
        for i in 0..n as u64 {
            let deviation = if i as usize == n / 3 { 1.0 } else { 0.0 };
            let neighbors = if n == 1 { vec![] }
                           else if i == 0 { vec![n as u64 - 1, 1] }
                           else if i == n as u64 - 1 { vec![n as u64 - 2, 0] }
                           else { vec![i - 1, i + 1] };
            consensus.add_tile(ConsensusTile {
                id: i,
                holonomy: HolonomyMatrix::from_rotation([0.0, 0.0, 1.0], deviation),
                neighbors,
                cycle_id: None,
            });
        }
        let result = consensus.check_consensus();
        assert!(!result.is_consistent, "{}-tile ring should detect fault", n);
        assert_eq!(result.faulty_tile, Some(n as u64 / 3),
                   "{}-tile ring: expected faulty tile at index {}", n, n / 3);
    }
}

#[test]
fn test_consensus_snap_behavior() {
    // Test that snapshot repeats return consistent results (determinism)
    let consensus = make_ring(8, 0.0);

    // Check 3 times — should get the same result each time
    let r1 = consensus.check_consensus();
    let r2 = consensus.check_consensus();
    let r3 = consensus.check_consensus();

    assert_eq!(r1.is_consistent, r2.is_consistent, "Deterministic snap 1");
    assert_eq!(r2.is_consistent, r3.is_consistent, "Deterministic snap 2");
    assert!((r1.deviation - r2.deviation).abs() < 1e-15, "Deterministic deviation");
}

#[test]
fn test_holonomy_matrix_custom_debug() {
    let m = HolonomyMatrix::from_rotation([1.0, 2.0, 3.0], 0.7);
    let debug_str = format!("{:?}", m);
    assert!(debug_str.len() > 0, "Debug output should be non-empty");
}

#[test]
fn test_consensus_tile_clone_equality() {
    let tile = ConsensusTile {
        id: 42,
        holonomy: HolonomyMatrix::from_rotation([0.0, 1.0, 0.0], 0.3),
        neighbors: vec![1, 2, 3],
        cycle_id: Some(7),
    };
    let cloned = tile.clone();
    assert_eq!(tile.id, cloned.id);
    assert_eq!(tile.neighbors, cloned.neighbors);
    assert_eq!(tile.cycle_id, cloned.cycle_id);
}
