# Advent of Code 2024 Solutions

All solutions in this module are ported directly from the original [`aoc24`](file:///home/jpursell/github/jpursell/aoc24) repository.

## Porting & Fidelity Overview

Every solution faithfully preserves the original logic, data structures, and algorithms from `aoc24/src/bin/`:

- **Trait Integration:** The standalone binaries (`01a.rs`, `01b.rs`, ..., `25a.rs`) have been unified into the [`AocSolution`](file:///home/jpursell/github/jpursell/aoc/src/lib.rs) trait (`part1(&self, input: &str) -> String` and `part2(&self, input: &str) -> String`), accompanied by unit tests for examples and full inputs.
- **Original Logic Preserved:** Where solutions relied on specific heuristics, brute-force iterations, or custom structures, they remain completely faithful to the original implementation without artificial optimizations or algorithmic rewrites.

## Notable Implementations & Differences

- **Day 22 (Monkey Market):**
  - Directly ports `aoc24/src/bin/22b.rs`, preserving the original `BTreeMap<[i8; 4], i8>` and `BTreeSet` algorithm with the outer-sequence loop over all candidate sequences (~80 million lookups across all buyers).
  - Runtime is approximately 15 seconds in `--release` mode.
- **Day 24 (Crossed Wires):**
  - Directly ports `aoc24/src/bin/24b.rs`, preserving the manual swap logic (`perform_swap`, `swapped: Vec<String>`) that was discovered through inspection and debugging.
  - Does not use automated full-adder graph heuristic algorithms.
- **Days 01–21, 23, 25:**
  - 1-to-1 ports of the respective binaries in `aoc24`.
  - Day 25: Part 2 returns `"0"` as Advent of Code concludes with 49 stars and no part 2 on Day 25.
