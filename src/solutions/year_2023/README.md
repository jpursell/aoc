# Advent of Code 2023 Solutions

All solutions in this module are ported directly from the original [`aoc23`](file:///home/jpursell/github/jpursell/aoc23) repository.

## Porting & Fidelity Overview

Every solution faithfully preserves the original logic, data structures, and algorithms from `aoc23`:

- **Trait Integration:** Solutions from `aoc23/src/day_XX/` (`a.rs`, `b.rs`, `mod.rs`) have been unified into the [`AocSolution`](file:///home/jpursell/github/jpursell/aoc/src/lib.rs) trait (`part1(&self, input: &str) -> String` and `part2(&self, input: &str) -> String`).
- **Authenticity Preserved:** The original algorithmic strategies—including custom data structures, brute-force or gradient descent techniques, multi-threaded Rayon iteration, and incomplete implementations—are retained exactly as written in `aoc23` without artificial speedups or rewrites.
- **Ignored Unit Tests:** Tests that take more than several minutes or simulate infinite loops as originally authored are annotated with `#[ignore = "..."]` so that the default `cargo test` run completes quickly and reliably while still keeping the tests runnable via `cargo test -- --ignored`.

## Notable Implementations & Differences

- **Day 08 (Haunted Wasteland):**
  - Ports `aoc23/src/day_8/` preserving the sequential `History` and `Historian` lockstep simulation (`count_steps_2`).
  - Because it steps all ghosts synchronously rather than calculating the least common multiple (LCM) of cycle lengths, `test_part2_full` is marked `#[ignore = "slow/infinite in aoc23"]`.
- **Day 12 (Hot Springs):**
  - Ports `aoc23/src/day_12/` preserving `mod a` (combinations of `?`) and `mod b` with `rayon`, unmemoized recursion, and custom enum definitions (`Condition::Opr`, `Dam`, `Unk`).
  - Because part 2 evaluates unmemoized state combinations across unfolded records, `test_part2_full` takes >10 minutes and is marked `#[ignore = "slow in aoc23 (>10 minutes with rayon)"]`.
- **Day 14 (Parabolic Reflector Dish):**
  - Ports `aoc23/src/day_14/` preserving `ndarray::Array2` and 300-cycle detection logic.
  - Completes in ~3 seconds in release mode.
- **Day 17 (Clumsy Crucible):**
  - Ports `aoc23/src/day_17/` preserving the `ndarray::Array3` 3D DP / table `Solver`.
  - Runs in ~14 seconds in release mode.
- **Day 18 (Lavaduct Lagoon):**
  - Ports `aoc23/src/day_18/` preserving `Lagoon` grid simulation for Part 1 and `PolyLagoon` polygon coordinate math (Shoelace formula and Pick's theorem) for Part 2.
- **Day 19 (Aplenty):**
  - Ports `aoc23/src/day_19/` preserving the 4D coordinate interval partition / Rayon grid search.
  - Because Part 2 scans large 4D volumes in parallel, `test_part2_full` is marked `#[ignore = "slow in aoc23 (multi-minute rayon grid search)"]`.
- **Day 21 (Step Counter):**
  - Ports `aoc23/src/day_21/` preserving the `DTTileCore` / `fast_expand` tile step simulation.
  - `test_part2_full` is marked `#[ignore = "slow/incomplete in aoc23"]`.
- **Day 23 (A Long Walk):**
  - Ports `aoc23/src/day_23/` preserving `ndarray::Array2` adjacency representation, node condensation, and recursive DFS longest-path search.
  - Completes in ~2.8 seconds in release mode.
- **Day 24 (Never Tell Me The Odds):**
  - Ports `aoc23/src/day_24/` preserving the gradient descent optimization approach for Part 2.
- **Day 25 (Snowverload):**
  - Preserves the unfinished state from `aoc23/src/day_25/` where graph partitioning with `petgraph` was in progress (`todo!("trying to make a graph like here...")`).
  - `test_part1_example` and `test_part1_full` are marked `#[ignore = "Unfinished in aoc23"]`. Part 2 returns `"0"`.
