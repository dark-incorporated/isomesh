# isomesh — BACKLOG

**Updated:** 2026-08-12
**Companions:** `CLAUDE.md` (rules), `FINDINGS.md` (what we know and how well),
`BACKLOG_ARCHIVE.md` (completed tickets + why they changed),
`docs/2026-08-11-implementation-brief.md` (the how),
`docs/2026-08-11-bevy-examples-catalog.md` (example detail), `docs/research/` (the why).

**61 tickets archived, 23 open.** Completed rows move to `BACKLOG_ARCHIVE.md` with their amendments
attached — read that before re-litigating a decision this project already made.

---

## How to work this backlog

1. Take the **topmost unblocked, unchecked ticket**. Don't cherry-pick interesting ones — the order
   encodes dependencies and the test harness exists so later work is cheap.
2. One ticket = one commit (or a short stack). Commit message starts with the ticket ID.
3. **Check the box in this file as part of that same commit.** This file is the state.
4. If a ticket can't be finished, leave it unchecked, add a `> BLOCKED:` line under it saying exactly
   what's in the way, and move to the next unblocked ticket. Do not half-finish and check the box.
5. If a ticket turns out to be wrong or to need splitting, edit it and say so in the commit.
6. **On completion, move the row to `BACKLOG_ARCHIVE.md`** with an indented annotation recording any
   amendment, deviation or falsified premise. The annotation is the point; the checkmark is not.
7. **New tickets** slot in by dependency, not by number. A ticket split after the fact takes a letter
   suffix (`T-005a`/`T-005b`, `A-002`/`A-002b`); a genuinely new one takes the next free number in its
   series even if that puts it out of numeric order (`A-015`).

### Definition of done — applies to every ticket

- Code compiles with no warnings. `cargo clippy -- -D warnings` clean.
- **`RUSTDOCFLAGS=-D warnings cargo doc --workspace --no-deps` clean.** A third of the lint job, and
  the third that clippy and fmt do not cover — it is what catches a doc link pointing at nothing, or
  at a private item. Added at A-002, which shipped one and found out from CI.
- `cargo test -p isomesh` green. `cargo fmt` applied.
- **`grep -ri bevy crates/` returns nothing.** Non-negotiable — see `CLAUDE.md` rule 2.
- Any algorithm ticket also passes the T-001 validity suite. No exceptions, no "I'll add tests later."
- Any perf claim has a committed benchmark that produced it.
- Public items have doc comments. Anything with a sign convention, a coordinate order, or a winding
  order says so **in the doc comment**, not in a code comment.
- **`FINDINGS.md` updated in the same commit** if the ticket measured something, contradicted
  something written down, or earned a method rule. A measurement that only exists in a commit message
  is not retrievable six weeks later.

**Size key:** `S` ≈ one sitting · `M` ≈ a day · `L` ≈ multi-day, consider splitting.

---

## Phase 0 — Foundation and the test harness ✅ complete

All eleven tickets archived (I-001..I-004, T-001..T-008). The bet paid: every algorithm since has
been cheap to validate because the harness predated it.

---

## Phase 1 — The usual suspects

Each algorithm ticket is done when: T-001 reports **no unexplained violations** on all seven test
fields at three resolutions; T-004 determinism passes; T-005 covers it; and a benchmark exists.

> **Amended 2026-08-12.** This originally said "zero violations on all seven fields." M-4 falsified
> that as a universal gate: Surface Nets is *legitimately* non-manifold where one cell carries two
> sheets (48 edges on capped gyroid, 15 on fbm_terrain), and those counts are pinned as **non-zero
> assertions** precisely so they can't drift silently. A known defect with a pinned number and a
> ticket that owns it satisfies this gate. An unexplained one does not.
>
> **A-010 has since landed and closed that owner.** Surface Nets' and Dual Contouring's counts stay
> pinned as non-zero — they are properties of one-vertex-per-cell, not bugs — and
> `manifold_dual_contouring` is the entry that takes the zero. Its own residue is M-59's parallel-edge
> collapse, pinned at one edge on the ✗15 fixture and zero everywhere else, owned by O-16.

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **A-014d** | **Subgrid MT — simplicial embedding (§3.2.3).** Split out of A-014b rather than dropped, because it is a distinct guarantee with its own acceptance. After §3.2.2 the mesh *"has manifold connectivity, but may be an immersed Δ-complex rather than an embedded simplicial complex"* — two adjacent tets sharing a face can each emit an oppositely-oriented copy of a polygon bound by the same non-normal loop. Three polygon types (quad, hexagon, corner pentagon); the pairs form a manifold tube or a punctured sphere, so **connectivity is already right and only *embedding* is at stake.** The fix pushes polygons into the tet interior by inserting midpoints. The paper is explicit that it is conditional — *"needed only when taking the union of the two polygons would yield a nonmanifold edge"* — which makes it a separate ticket rather than a branch inside A-014b. **Re-aimed by M-99, and the target is narrower than it looked.** Appendix A's Theorem A.1 guarantees manifold connectivity in the even-sum case, and Lemma A.2 says the polygons *before* §3.2.3 already form an edge-manifold cell complex. The precondition was checked: **0 of 98,304 tetrahedron faces violate even-sum on any of the seven reference fields**, so the guarantee is in force. Measured accordingly — **unwelded output has 0 non-manifold edges and 0 non-manifold vertices on every field**; the counts below appear only after a weld by position, which cannot tell two coincident-but-distinct polygons apart. So this ticket is **not** "make the output manifold" — it already is, as a complex. It is *separate the coincident geometry far enough that an indexed, welded mesh can still represent it*, which is precisely what the inset does. M-59 is the precedent: a manifold complex an index buffer cannot express, now seen in a second algorithm. **It owns three measured defects rather than a hypothesis (M-97).** At 17³ the subgrid extractor is clean on `sphere`, `torus`, `box_exact` and `thin_plate`, and reports `csg_difference (3 non-manifold edges, 6 non-manifold vertices, 6 inconsistently oriented)`, `fbm_terrain (4, 6, 19)` and `gyroid (0, 0, 138)`. The first two are the pinching §3.2.3 names. **`gyroid`'s is the same cause wearing a different face**: 12 of its triangles have zero area and 24 have the gradient lying *exactly in the triangle's plane*, so A-014e's orientation vote is undecided rather than wrong — insetting gives those polygons area and a normal transverse to the gradient, which fixes both symptoms at once. Those counts are pinned in `the_validity_suite_over_every_reference_field`, so this ticket's real acceptance is **driving all three rows to zero**. **Acceptance:** T-002 reports zero self-intersections on a two-tet fixture built from a non-normal loop that A-014b leaves immersed — but note M-83 first: the counter is blind to folds inside a Steiner fan, so the fixture has to be built from loops whose triangles do *not* all share an apex, or the zero means nothing. **The construction was retrieved in full on 2026-08-12** and is no longer blocked: *"we 'push' polygons into the tet interior: we insert the midpoints of all polygon edges contained in any edge of `f`, and move them a small distance in the inward normal direction. For pentagons, we also insert the midpoint of the edge opposite the distinguished corner. The triangulation patterns in Figure 15, right ensure that inset regions stay within the tetrahedron."* **Two implementation attempts are on record and both were reverted (M-101).** The point-insertion prose is complete, so it was applied first to every boundary-disk region — `box_exact` went from `(0,0,0)` to `(84, 84, 180)` — and then only to loops confined to a single face, the per-tet reading of "two adjacent tetrahedra sharing a face `f`": still `(42, 84, 180)`. **What the failures point at:** a polygon edge lying along an edge of `f` is shared with whatever else meets at that tet edge, so inserting a midpoint detaches the disk from every neighbour that did not also insert one. Which polygons are duplicated is a property of a **pair** of tetrahedra, and this implementation is strictly per-tet. So the first open question is no longer "what are Figure 15's diagonals" but **"which polygons does the inset apply to, and does answering that need neighbour information the per-tet architecture does not carry"** — and if it does, this is an architectural change rather than a triangulation table. Beyond that, Figure 15's patterns themselves are still unread, being a picture rather than prose — so the quad/pentagon/hexagon triangulations need deriving or the figure needs looking at, and rule 5 applies to that step alone. | M | A-014b |
> BLOCKED: **which polygons the inset applies to.** The point-insertion prose is complete and was implemented twice; both attempts made the measured result *worse* and both were reverted (M-101). Applied to every boundary-disk region, `box_exact` goes from a clean `(0,0,0)` to `(84, 84, 180)`; restricted to loops confined to one face — the per-tet reading of "two adjacent tetrahedra sharing a face `f`" — it is still `(42, 84, 180)`. A polygon edge lying along an edge of `f` is shared with whatever else meets at that tet edge, so inserting a midpoint detaches the disk from every neighbour that did not also insert one, and **which polygons are duplicated is a property of a pair of tetrahedra** while this implementation is strictly per-tet. Unblocking it needs one measurement, not another attempt: instrument the extractor to report which polygons actually coincide across a shared face on `csg_difference`, and see whether that condition is expressible without neighbour information. If it is not, this is an architectural change rather than a triangulation table. **The three defects it owns are pinned and harmless meanwhile** (M-97, M-99): connectivity is provably manifold under Theorem A.1, and the counts only appear after a weld.
| ☐ | **A-002b** | **Marching Cubes 33 interior ambiguity — the trilinear body saddle.** Deliberately deferred at A-002, on evidence, not forgotten. Three reasons. (1) `catalog-v2.md:107` is explicit: *"Skip the interior test; spend the budget on chunk seams"* — a game needs topological *consistency*, which A-001 already has, over *correctness*. (2) **There is no correct published table to transcribe.** Custodio et al. 2013 (`10.1016/j.cag.2013.04.004` §5.1) prove Chernyaev's interior test tracks a quadratic where the true saddle trajectory is hyperbolic with an asymptote, so case 13.5.2 is misread as 13.5.1 — counterexample values in their Appendix A — and Lewiner's reference implementation omits disambiguation for cases 10 and 12 entirely. Rule 5 forbids inventing the missing one. (3) The v1 catalog prices it: the decider is *"~free"*, the guaranteed version is **730 subcases in the LUT**. Also needs cell-interior vertices for tunnels, which the grid-edge-keyed vertex cache has no slot for. **Acceptance:** a cell where the body saddle says "tunnel", meshed as a tunnel, with the sign tracked by Custodio's correction rather than Chernyaev's `F(t)`. | L | A-002 |
> BLOCKED: **there is no correct published table to transcribe**, which is stated on the row and is a rule-5 stop rather than a difficulty. Custodio et al. prove Chernyaev's interior test tracks a quadratic where the true saddle trajectory is hyperbolic, and Lewiner's reference implementation omits cases 10 and 12 entirely. Unblocking it means deriving the corrected decider from Custodio §5.1 and its Appendix A counterexamples — real work with a real source, not a transcription — and `catalog-v2.md:107` prices it as *"skip the interior test; spend the budget on chunk seams"* for a game.

---

## Phase 2 — Game-shaped infrastructure ✅ complete

All seven tickets archived (G-001..G-007): chunk coordinates, dirty-set re-meshing, brush operations,
field-derived LOD, collider export, the frame budget, and streaming. Still zero Bevy — every one of
them lives in the core crate, so the CAD side gets them too.

The bet that paid here was **G-006's**: the ticket asked for `mesh_within_budget(ms)` and a `no_std`
crate cannot read a clock, so the budget became a predicate the caller owns. That made streaming a
composition rather than a rewrite — `ChunkStream` decides residency, `DirtySet` orders the work
nearest-first, and the caller's own clock decides when to stop.

---

## Phase 3 — `bevy_isomesh`

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|

---

## Phase 4 — Examples

Two groups. The algorithm demos are quick and prove correctness visually. **The game-shaped ones are
the point** — they're how someone decides whether this crate is usable.

### 4a — Algorithm demos

| | ID | Example | Blocked by |
|---|---|---|---|
| ☐ | **E-110** | `qef_clamp` — clamp toggle, live self-intersections/1k, offending triangles in red | A-009 |
| ☐ | **E-112** | `precision_f32_vs_f64` — same field at ~1e6 offsets; f32 cracks, f64 doesn't. Condition number in the HUD. | A-007 |
| ☐ | **E-114** | `hermite_debug` — crossings, normals, solved vertex, cell box as gizmos. The view you debug A-007 in. | A-006 |

### 4b — Game-shaped

These use the algorithms the way a game does: chunked, edited, budgeted, collided against.

| | ID | Example | What it has to prove | Blocked by |
|---|---|---|---|---|
| ☐ | **E-206b** | `game_capsule_walk` — a real character controller on streamed terrain, not a ray sweep | **What E-203 does not test.** Its probe sweep finds holes and lips by casting rays down at 400 points a frame, which answers *"do the chunks meet"* (M-106: 495 crossings, 0 holes, seam lip 0.412 cells against 0.539 of terrain roughness). It does not answer *"can a body move through here"* — nothing in it slides along a surface, so an invisible wall that only a moving capsule reveals would go unseen. Wants an actual controller with a capsule, a step height and contact resolution. **This is where a physics engine earns its place** rather than `parry3d` raycasts; see V-22 in `FINDINGS.md` for the Avian note. | M | E-203 |
| ☐ | **E-204** | `game_destruction` — shoot a wall, it craters, debris becomes rigid bodies | Runtime fragments are correct physics bodies, not pre-fractured props. Carve a spiral and a hollow shell — that's where decomposition fails. | G-005, G-003 |
| ☐ | **E-205** | `game_lod_flyover` — fly out and back across LOD transitions | No popping, no cracks, no hitching. Transvoxel doing its job at speed. | G-004, A-011b |
| ☐ | **E-206** | `game_budget` — a deliberately overloaded edit queue under a frame budget | Frame time stays flat while the backlog drains. **Amortized cost per frame is the number no paper measures and the only one a game cares about.** | G-006 |
| ☐ | **E-207** | `game_editor` — sculpt with brushes, undo/redo over an op log | The CAD/editor use case. Undo is a re-fold of the log, not a snapshot. | G-003 |
| ☐ | **E-208** | `game_paint` — spray colour on a wall, then blow a hole through it | Paint on the remaining wall is exactly where you sprayed it. Row 4 of the opportunities table. | G-003, B-002 |
| ☐ | **E-209** | `game_csg_props` — place and boolean CSG primitives into the world live | Re-mesh per frame under moving primitives; concave sharp edges hold up. | G-003, A-007 |

---

## Phase 5 — Measurement

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **M-002** | **`bench_resolution_sweep` — the live plot only.** ~~fits `t = a + b·n³`, **prints `a`**~~ **The numeric half is done and its premise is falsified (M-62).** The bench had been printing `NaN` since the algorithm names were spelled out; fixed, and the answer is the opposite of what this ticket expected — Marching Cubes' `a` is **0.5118 ms, 0.64% of the largest run**, so there is *no* large fixed cost, and Surface Nets and Dual Contouring fit `a < 0`, meaning the model does not describe them. What is left is the **live plot**, which is a Bevy example and belongs in Phase 4. | S | T-006 |
| ☐ | **M-003** | **`bench_stage_breakdown`** — stacked bar: contour / normals / weld / collider / upload. Published comparison: contouring 68 ms vs halfedge construction 58 ms — **the contour was 54% of a usable mesh.** Find your ratio before optimizing anything. | M | G-005, M-001a |
| ☐ | **M-004** | **Write up M-001a..M-003** as `docs/research/YYYY-MM-DD-measured-comparison.md`. Numbers, method, hardware, and what surprised you. This is publishable on its own. | S | M-003 |

---

## Phase 6 — GPU (do not start before Phase 5)

The speed analysis is explicit that stage placement dominates the extraction algorithm by roughly an
order of magnitude. Which means GPU work is worth doing — and worth doing *after* you know your own
numbers, or you won't be able to tell what the port bought you.

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **GPU-001** | `isomesh-gpu` skeleton, `wgpu 29.0.3`. Public API takes `&wgpu::Device` / `&Queue` / `&mut CommandEncoder`. **Never a Bevy type.** | M | M-001a |
| ☐ | **GPU-002** | Shader composition: `include_str!` + ~40-line `#include`/`#ifdef` preprocessor. **Not `naga_oil`** — see `CLAUDE.md`. | S | GPU-001 |
| ☐ | **GPU-003** | naga CI validation of every shader permutation. No GPU required. ~30 lines, highest value-per-line in the repo. | S | GPU-002 |
| ☐ | **GPU-004** | Compute-shader Marching Cubes + readback. **Headless harness first, no Bevy in the room** — if it can't run against raw wgpu, the abstraction leaked. | L | GPU-003 |
| ☐ | **GPU-005** | `E-301 gpu_compute_mc` — asserts **bit-identical** to CPU, or documents the exact divergence. "Looks the same" is not an acceptance criterion. | M | GPU-004, B-003 |
| ☐ | **GPU-006** | `E-302 gpu_vs_cpu` — both live, timing HUD, resolution slider. Watch the gap **close** at small grids: launch overhead made visible. | M | GPU-005 |
| ☐ | **GPU-007** | **Mesh shader capability probe.** Print what this adapter reports for `EXPERIMENTAL_MESH_SHADER` and stop. **macOS/Metal is the unverified case** — wgpu's spec table says MSL is *planned*, the tracking issue says the Metal backend merged. Report the truth before writing a line of shader. | S | GPU-004 |
| ☐ | **GPU-008** | `E-303 gpu_mesh_shader` — feature-gated, off by default, graceful fallback, never panics on an unsupported adapter. | L | GPU-007 |

---

## Phase 7 — Experiment infrastructure

**Added 2026-08-13 after a re-evaluation against three questions: is the crate ready for novel
experimentation, is it usable from the Bevy ecosystem, and will the harness make experiments
iterative?** Phases 0–6 built algorithms and proved them correct. Nothing in them was built for
*swapping a rule and measuring the difference*, which is the entire shape of the work the research
docs now point at. This phase is the cost of that gap, paid deliberately.

The evidence it is real: `benches/shootout.rs` and `src/property/extraction.rs` both hand-enumerate
every algorithm by name. There is no `Extractor` trait — the public traits are `MeshSink`, `Real`,
`Sdf`, `Shape3`, `ReferenceField`, and each extractor is an unrelated struct. **Adding algorithm #9
costs an O(N) edit across benches, property tests and examples instead of O(1).**

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **X-001** | **`Extractor` trait and a single registry.** One trait covering the common shape (field + shape + grid → `MeshSink`), with algorithm-specific configuration as an associated type so Dual Contouring's Hermite data, subgrid's `samples`, and Transvoxel's LOD each stay expressible. Then **one** `ALL_EXTRACTORS` list that `benches/shootout.rs`, `property/extraction.rs`, `the_validity_suite_over_every_reference_field` and the examples all enumerate from. **Acceptance:** adding a ninth algorithm touches exactly one list, and a test asserts the registry's length equals the number of `Extractor` impls. **Do not force a trait that does not fit** — if two algorithms genuinely cannot share a signature, say so on this row and cover the rest. | L | — |
| ☐ | **X-002** | **An ablation seam that does not create a second execution path.** The next experiments are *not* new algorithms; they are the same algorithm with one rule swapped — Probabilistic Quadrics against Tikhonov `λ`, persistence-thresholded ambiguity against the asymptotic decider, curvature-aware placement against planar. `property/extraction.rs` currently argues, correctly, that a swappable table parameter *"would be a second execution path in production code, and the crate's rule is one."* **That rule protects production and blocks experimentation, and the tension must be resolved deliberately rather than by accident.** Proposed resolution: the variant is a *type* parameter with a default, so production monomorphises to exactly one path and the experiment instantiates another. Zero runtime branches, one source of truth. **Acceptance:** two vertex rules measured against each other on all seven fields, in one bench run, with no `if` in the hot loop — verified by reading the generated assembly or by a codegen test. | L | X-001 |
| ☐ | **X-003** | **An `experimental` feature and module.** Speculative algorithms currently have nowhere to live but the stable public API. Gate them: `#[cfg(feature = "experimental")] pub mod experimental`, off by default, exempt from semver, and **exempt from nothing else** — T-001 validity, T-004 determinism and the property suite still apply. **Acceptance:** `cargo tree -p isomesh` is unchanged with the feature off; an experimental algorithm passes the full validity suite. | S | X-001 |
| ☐ | **X-004** | **First ablation: Probabilistic Quadrics.** Trettner & Kobbelt `10.1111/cgf.13933` — **already in the corpus and invisible to `distill_search`**, found via `catalog_read`. It states outright that quadric minimisation *"is in many cases not robust and requires an SVD or some ad-hoc regularization"*, then derives a closed form solvable by a plain linear system, **50× faster than SVD**, demonstrated on isosurface extraction. **This supersedes the `λ ≈ 0.01` regularizer** taken from the adjacent-math audit. Measure against the current solve on all seven fields: Hausdorff, self-intersections per 1k, non-manifold edges, condition number, ms. **This is X-002's proof that the seam works, and a real improvement either way — a null result is also a finding.** | M | X-002 |

---

## Phase 8 — Usable by someone who is not us

Every example in `bevy_isomesh/examples/` demonstrates an *algorithm*. **None of them shows a person
how to put a meshed SDF into their own Bevy app.** That is the first thing a prospective user looks
for, and it does not exist. Neither crate is reserved on crates.io.

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **I-005** | **Reserve `isomesh` and `bevy_isomesh` on crates.io.** A `0.0.0` placeholder. **Overdue, not deferred** — this was raised on 2026-08-12, the repo is public and points at the name, and `megamesh` was taken 48 hours before we checked it. Ten minutes, unbounded downside. | S | — |
| ☐ | **B-005** | **`bevy_isomesh/README.md` and a quickstart example.** The README the crate does not have (it has no `readme` key either). Plus `examples/quickstart.rs`: **under 30 lines**, add the plugin, spawn one SDF volume, see a mesh. No HUD, no toggles, no comparison — the 16 existing examples all teach the library's internals, and none teaches its use. **Acceptance:** someone who has never seen the crate gets a sphere on screen by copying one file. | M | — |
| ☐ | **B-006** | **Publishing metadata on both crates.** `[package.metadata.docs.rs] all-features = true`, crate-level `//!` docs that open with a usage example rather than a design rationale, `readme` keys, `keywords`/`categories` reviewed. **Plus the compatibility matrix the architecture doc specified and nobody wrote:** `isomesh ↔ isomesh-gpu ↔ wgpu ↔ bevy`, in the README, as a table. Bevy consumers check that first and its absence reads as unmaintained. | M | B-005 |
| ☐ | **B-007** | **Bevy-ecosystem conventions.** A Bevy Assets listing entry, a version-support table (`bevy 0.19 → bevy_isomesh 0.1`), and a `CHANGELOG.md`. Cheap, and their absence is what makes an ecosystem crate look abandoned regardless of commit rate. | S | B-006 |

---

## Phase 9 — Keeping the harness honest as it grows

`FINDINGS.md` is **166 KB / 730 lines / 107 measurements / 17 falsified claims / 16 open questions**,
and it is the most valuable artefact in the repo. It is also approaching the size at which nobody
reads it. Two structural gaps beside that.

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **T-009** | **Committed metric baselines and a regression diff.** `docs/measurements/*.csv` is rewritten fresh each run, so nothing answers *"did this change make anything worse?"*. T-007's golden hashes catch a changed **mesh**; they are blind to a change that keeps the mesh bit-identical and doubles the runtime, or that worsens Hausdorff on one field. Commit a baseline CSV per machine; add `cargo run --bin regress` that diffs the current run against it and **fails on a regression beyond a stated tolerance**, printing the row. **Acceptance:** a deliberately slowed extractor fails the check and names the field and the metric. | M | — |
| ☐ | **T-010** | **`FINDINGS.md` index and split policy.** Add the same index treatment `BACKLOG_ARCHIVE.md` got — a table of every `M-`, `✗`, `O-` and `P-` entry with its one-line claim — and a stated split rule for when it exceeds a size (`FINDINGS.md` as index + `findings/` by axis). **The index is what keeps it a lookup table rather than a diary.** | S | — |
| ☐ | **T-011** | **An experiment record format.** `FINDINGS.md` has slots for measurements (`M-`), falsified beliefs (`✗`), open questions (`O-`) and pre-registered predictions (`P-`). It has **no slot for an experiment that was run and reverted** — and with Phase 7, most experiments will be. Add an `E×-` section: hypothesis, the ablation, the numbers both ways, verdict, **kept or reverted and why**. A reverted experiment with recorded numbers is what stops it being re-run in six weeks. | S | T-010 |
| ☐ | **T-012** | **Cross-machine measurement protocol.** `resolution_sweep-ryzen9-5900x.csv` exists beside `resolution_sweep.csv` and that second machine already produced a finding (M-45: Surface Nets' superlinearity is the algorithm, not one cache hierarchy). Make it systematic — a documented procedure, consistent filenames, and the machine's specs recorded in the CSV header rather than the filename. | S | T-009 |

---

## Phase 10 — The field contract

**Added 2026-08-15 from the SDF corpus build-out.** The crate has an *input contract it never wrote
down*, and one reference field already violates it in the exact region where its defects live.

`csg_difference` declares `is_exact_distance() -> bool { true } // away from the seam` — a comment
admitting the invariant is false, on a function returning true. Marschner, Sellán, Liu & Jacobson 2023
(`10.1145/3610548.3618170`) name this object: a **Pseudo-SDF**, *eikonal almost everywhere yet not a
distance function*, with error **concentrated at seams** — the union's medial axis. That is exactly
where A-014d located `csg_difference`'s coincident polygons.

**And the error is one-signed.** `min` (union) never overestimates — a conservative lower bound, safe.
`max` (intersection, subtraction) **overestimates near concave seams** — the direction that lets a
tracer step through a surface and mis-places an interpolated vertex. `csg_difference` is
`max(box, −sphere)`. **It is wrong in the dangerous direction.**

**The load-bearing distinction for everything below:** `min`/`max` of 1-Lipschitz functions is
1-Lipschitz. **The Lipschitz bound survives arbitrary CSG; exactness does not.** So a field stays a
valid conservative bound forever, no matter how many brush strokes — which is what makes Phase 11
provably correct under unlimited player editing.

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **F-001** | **Replace `is_exact_distance() -> bool` with a declared bound.** Bálint, Valasek & Gergó (`10.14232/actacyb.24.1.2019.3`, in corpus) prove every true SDF is Lipschitz with **smallest constant exactly 1**, and their follow-up gives a `q ∈ (0,1)` **underestimate ratio** that composes through a CSG tree. That is the type this should have been — `// away from the seam` is a `q < 1` field declared `q = 1`. Proposed: `fn bound(&self) -> FieldBound` with `Exact`, `Lipschitz { l }`, `Underestimate { q }`, `Unbounded`. **Acceptance:** every reference field declares honestly; `csg_difference` is no longer `Exact`; `validate/accuracy.rs` and `benches/shootout.rs` refuse to report Hausdorff for anything that is not `Exact`, rather than reporting it and annotating the caveat. **This ticket gates the whole phase — nothing below can assume what it does not know.** | M | — |
| ☐ | **F-002** | **A Lipschitz-bound validator, in the harness.** Sample `‖∇f‖` over a dense grid per field and **assert the declared bound actually holds**. A declared bound nobody checks is the same class of defect as the one this phase exists to fix. **Acceptance:** the validator fails on a field whose declaration is deliberately tightened by one step. Reports the measured `sup‖∇f‖` and the fraction of samples within tolerance of the eikonal condition, both recorded. | M | F-001 |
| ☐ | **F-003** | **CSG combinators that propagate the bound.** `Union`, `Intersection`, `Difference`, `SmoothUnion` each computing their result's `FieldBound` from their operands' — the algebra from F-001's source. Ricci 1973 (`10.1093/comjnl/16.2.157`) is the origin of min/max CSG and is on the hand-acquisition list. **Acceptance:** composing two `Exact` fields with `min` yields a declared bound that F-002 confirms; the same with `max` yields a strictly weaker one, and the test asserts the asymmetry rather than treating the two as equivalent. | M | F-002 |
| ☐ | **F-004** | **Measure how fast the distance property degrades under repeated CSG. Original contribution — no paper measures this.** Apply *N* random sphere subtractions to an analytic box; sample `‖∇f‖` over a grid; plot the distribution against *N*. Sits alongside the manifold and self-intersection metrics as a recorded field-quality number. **Acceptance:** a curve, in `docs/measurements/`, and a plain answer to *"after how many brush strokes is the field no longer usable as a distance?"* — which is the question a destructible game actually needs answered and nobody has. | M | F-003 |

---

## Phase 11 — Exploiting the bound

Everything here rests on Phase 10's finding that **the Lipschitz bound survives editing.** These are
correct under unlimited player carving; nothing here assumes exactness.

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **F-005** | **Empty-cell rejection by sphere tracing — the attack on M-98's 70×.** Hart (`10.1007/s003710050084`, in corpus): with `L = 1`, a **single** evaluation at a cell centre with `\|f\| >` half the cell diagonal proves the entire cell empty. M-98 measured subgrid at `6 tets × 6 edges × 16 samples = 576` field evaluations per cell against Marching Cubes' 8, and predicted ~72× against a measured 70× — **the constant is the whole story, and this deletes it for every empty cell.** Kalra & Barr (`10.1145/74334.74364`, hand-acquisition) give the `q`-bounded version when the field is not 1-Lipschitz. **Acceptance:** identical output to the unrejected path, bit-for-bit, on all seven fields; re-measure M-98's ratio and record both numbers. | M | F-001 |
| ☐ | **F-006** | **Segment tracing / enhanced sphere tracing.** `10.1111/cgf.13951` (HAL PDF already in the catalog entry). Uses a *directional* Lipschitz bound along a ray rather than the global one, which is strictly tighter. Relevant to F-005's cell test and to any future ray-vs-field query for physics. **Acceptance:** measured step-count reduction against plain sphere tracing on the reference fields; a null result is a finding. | M | F-005 |
| ☐ | **F-007** | **Kink-aware edge interpolation — possibly the whole fix, and much narrower than redistancing.** `min`/`max` preserve the **sign** exactly: `{min(f,g) ≤ 0}` *is* the union. What they break is (a) the interpolated crossing position, because the field is kinked and linear interpolation across a kink is wrong, and (b) gradient-based normals. **You know where the kinks are — you built the CSG tree.** So: detect the kink on an edge (a gradient discontinuity between endpoints), and solve for the crossing piecewise instead of linearly. Pujol & Chica (`10.1016/j.cag.2023.06.020`, in corpus) treat exactly this problem. **Acceptance:** `csg_difference`'s Hausdorff improves, and O-9's `0.0833` forward error is decomposed into seam-bias and harness-bias with a number on each. | L | F-003 |

---

## Phase 12 — SDF construction

The crate consumes fields and has never built one. Every ticket here also gives the harness a *second
source of truth* to check the first against.

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **S-001** | **Exact Euclidean distance transform** — Felzenszwalb & Huttenlocher, separable, `O(n)` per dimension. The **ground truth** every other constructor is measured against, and the cheapest to get exactly right. **Acceptance:** on a sampled sphere, agrees with the analytic distance to within one sample spacing everywhere; a brute-force `O(n²)` reference agrees exactly on a small grid. | M | — |
| ☐ | **S-002** | **Fast sweeping** — Zhao. `O(N)`, a few Gauss–Seidel passes, no heap, trivially parallel. The pragmatic default. **Acceptance:** converges to S-001's answer within a stated tolerance; sweep count to convergence recorded per field. | M | S-001 |
| ☐ | **S-003** | **Fast marching** — Sethian (`10.1073/pnas.93.4.1591`, free on pnas.org, hand-acquisition). Heap-based, `O(N log N)`, single pass. Worth having because it is the citation everyone knows and because its error characteristics differ from sweeping. **Acceptance:** measured against S-001 and S-002 — accuracy *and* wall clock, in `docs/measurements/`. | M | S-002 |
| ☐ | **S-004** | **Narrow-band reinitialization** — Peng et al. (in corpus). **The best structural match to a brush stroke**, because cost scales with edited *surface area* rather than chunk volume. Carry Sussman & Fatemi's warning explicitly: naive reinitialisation **moves the zero set**, which in a destructible game means geometry creeping after every edit. **Acceptance:** measure the zero-set drift per reinitialisation and assert it below a stated bound — that assertion is the ticket. | L | S-002, F-004 |
| ☐ | **S-005** | **Jump flooding**, GPU. Approximate, `O(log n)` passes, the standard GPU answer. Lives in `isomesh-gpu`. **Acceptance:** error against S-001 quantified rather than assumed; "approximate" is a measurement, not an adjective. | M | S-001, GPU-001 |
| ☐ | **S-006** | **Mesh → SDF by angle-weighted pseudonormal** — Bærentzen & Aanæs (in corpus). **This is a proof, not a heuristic**, and it is the right tool for geometry isomesh produced itself, which already carries a `V−E+F == 2` guard. **Acceptance:** round-trip — mesh a sphere, convert back to a field, re-mesh, and compare against the original. That round-trip is a strong end-to-end test the crate does not currently have. | M | S-001 |
| ☐ | **S-007** | **Mesh → SDF by generalized winding number**, for imported or damaged input. **Do not cite Barill 2018 as state of the art** — the 2026 Antipodal paper (`10.1145/3811323`) states its order-0/order-1 expansions are *"very imprecise… not useful for applications."* Use Antipodal or Xie, Hafner & Wojtan (`10.1145/3811339`), both in corpus, both exact and faster: the winding number reduces to one ray-surface intersection plus a sum over **boundary** edges, so **cost scales with holes, not triangles** — a nearly-closed mesh is nearly free. **Use GWN to classify points, never to repair meshes**: Takayama et al. 2014 (in corpus) is the GWN authors' own paper explaining that the orientation-repair application is *"fundamentally flawed."* **Acceptance:** classifies correctly on a deliberately hole-punched mesh where S-006 fails. | L | S-006 |

---

## Phase 13 — Certificates and field harness

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **T-013** | **Per-cell normal-variation isotopy certificate.** **Hausdorff error does not certify topological correctness — provably.** Two surfaces can be arbitrarily Hausdorff-close and not homeomorphic. Every real theorem adds a second hypothesis, and the isosurface-specific ones — **Plantinga & Vegter** (`10.1145/1057432.1057465`) and **Boissonnat–Cohen-Steiner–Vegter** (`10.1007/s00454-007-9011-4`), both already in the corpus — certify **isotopy from a per-cell normal-variation condition**. Local, cheap, checkable *during* extraction, and a natural fit for a marching pipeline. **This upgrades the crate's claim from "we report Hausdorff" to "we certify topology," which nothing else in this space does.** **Acceptance:** the predicate is evaluated per cell and its pass rate reported per field; a field engineered to violate it is correctly flagged. | L | F-001 |
| ☐ | **T-014** | **Downsampling operator comparison. Original — the comparison does not exist.** Mean vs min vs re-evaluate vs wavelet, measured on all seven fields across LOD 0–3. **The literature predicts your answer:** you do not downsample, you *re-sample* — every level built by evaluating the field at that spacing (Frisken's ADF, Koschier's hp-adaptive). Under re-sampling, a plate thinner than a coarse cell gives all-positive corners and correctly disappears; under box-filter averaging the straddling ± set survives and Marching Cubes keeps emitting triangles — **which is exactly M-72's measured 4,088 → 1,016 → 248 → 56.** So this ticket's first job is to confirm that M-72's aliasing is the predicted failure of an operator the literature already rejects, and its second is to publish the head-to-head nobody has. | M | F-001 |
| ☐ | **T-015** | **Field-quality metrics as first-class recorded numbers.** `sup‖∇f‖`, eikonal residual distribution, declared-vs-measured bound gap, and F-004's degradation curve — reported per field beside the mesh metrics, and wired into T-009's regression baseline so a field that silently degrades fails CI. **The crate measures its output exhaustively and its input not at all.** | M | F-002, T-009 |
| ☐ | **T-016** | **Constructor accuracy harness.** One place that runs S-001..S-007 against analytic ground truth on the reference fields and reports accuracy, wall clock and memory. The `M-001a` shootout for the *input* half of the pipeline. **Acceptance:** a CSV in `docs/measurements/` and a stated recommendation for which constructor a consumer should default to, with the number behind it. | M | S-003, S-006 |

---

## Phase 14 — Research tickets

**Added 2026-08-15 after a full pass over `FINDINGS.md` (107 measurements, 17 falsified claims, 16
open questions) followed by a literature check on the patterns.** Three of the 107 recur as
**mechanisms** rather than incidents. Those are the research directions; the rest are history.

### The experimental protocol — mandatory for every ticket in this phase

A ticket here is an **experiment**, not a feature. It is done when the question is answered, including
when the answer is "no." Each one must carry all five fields, and **the hypothesis must be written
into `FINDINGS.md` as a `P-` entry before the measurement runs** — a prediction that first appears
after the number is known is not evidence, and this project has already caught itself writing
expectations into docs that measurement then disproved (✗1, ✗3, ✗14, O-14).

| Field | Requirement |
|---|---|
| **H** — Hypothesis | Falsifiable, numeric where possible, pre-registered as `P-n` in `FINDINGS.md` **in the commit before** the measuring commit |
| **Harness** | Committed code. Runs in CI or by one documented command. No throwaway probes — M-89's census had to be re-run because the first one wasn't committed |
| **Records** | Named metrics, to `docs/measurements/*.csv`, wired into T-009's regression baseline |
| **Falsified by** | The specific observation that kills H. **A ticket with no falsifier is not an experiment** |
| **FINDINGS obligation** | `M-` if measured, `✗` if a written claim died, `E×-` if the change was reverted (T-011's format). **Same commit.** A result only in a commit message is not retrievable in six weeks |

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **R-000** | **Mechanise the protocol.** A `#[experiment]` harness: registers the `P-` id, refuses to run if no pre-registration exists, emits a CSV row with git SHA + machine + timestamp, and prints the FINDINGS stanza ready to paste. **The feedback loop is currently a discipline; make it a compile error.** **Acceptance:** an experiment without a pre-registered `P-` fails to build. | M | T-011 |

---

### 14a — Welding is a topology-destroying operation, and the predicate exists

**The strongest pattern in `FINDINGS.md`.** Five measurements, two independent algorithms, one
mechanism:

- **M-59** — *"The dual of a manifold surface is a manifold complex; the index buffer is where it stops being a manifold mesh."*
- **M-99** — *"provably manifold and my weld is what breaks it — the same mechanism as M-59, in a second algorithm. Unwelded: 0 non-manifold."*
- **M-96** — unwelded output has no topology to check at all (2,240 boundary edges / 896 triangles). **The weld is a precondition, not a tidy-up.**
- **M-93** — 30 reported self-intersections were *all* vertex-duplication artefacts.
- **M-48** — the edge cache "does not share everything."

So the weld is simultaneously **required** and **destructive**, and the crate has no theory of when a
merge is safe. **The literature has one, it is already in the corpus, and it is cheap.**

Dey, Fan & Wang (`10.48550/arXiv.1208.5018`, in corpus) give the **link condition**:
`Lk u ∩ Lk v = Lk{u,v}`. For two *non-adjacent* coincident vertices `Lk{u,v} = ∅`, so on a triangle
surface it reduces to a one-ring test:

> **merge (u,v) is safe ⟺ `Lk u ∩ Lk v = ∅`** — their one-rings share no vertex. **O(deg u + deg v).**

They also prove a k-way merge decomposes into **k−1 pairwise merges evaluated in the intermediate
complex** — so a bucket of ≥3 coincident vertices is **not atomic**, which is what R-002 is about.
Guéziec et al. 1998 (`10.1145/280953.281628`, acquired) state M-59's framing verbatim 28 years early:
*"Several manifolds can be mapped to the original non-manifold by identifying vertices."*

**What is unclaimed:** nobody states this predicate for **index-buffer welding of coincident vertices
emitted by an isosurface extractor**, and nobody publishes the measured rejection rate. That is the
contribution — modest, real, and a paragraph rather than a paper.

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **R-001** | **Gate the weld on the one-ring predicate.** **H:** a weld gated on `Lk u ∩ Lk v = ∅`, leaving rejected pairs split, yields **exactly 0 non-manifold edges and 0 non-manifold vertices** on all seven fields × all extractors, where the unconditional weld yields N > 0. **Harness:** both welds run on the same meshes in one pass. **Records:** non-manifold edges/vertices both ways, rejected-merge count R, Δ vertex count, weld wall-clock both ways. **Falsified by:** the gated weld still producing non-manifold output — **which would be the more interesting result**, proving the surface link condition insufficient for index-buffer realisation. **FINDINGS:** `M-` either way; `✗` against M-59's and M-99's framing if the predicate fully explains them. | L | R-000 |
| ☐ | **R-002** | **k-way welds may be order-dependent — this threatens the determinism guarantee.** Dey/Fan/Wang decompose a k-way merge into k−1 pairwise merges *in the intermediate complex*, so bucket order can matter. **H:** for buckets of ≥3 coincident vertices, at least one reference field yields **≥2 distinct outputs** across P seeded permutations of within-bucket merge order. **Harness:** permute, re-weld, compare byte-identity. **Records:** distinct-output count per field, vertex count spread. **Falsified by:** all P permutations byte-identical on every field — meaning k-way weld is confluent and no canonical order is needed. **If H holds, `CLAUDE.md`'s byte-identical guarantee is violated the moment gating lands**, and a canonical merge order must be pinned in the same commit. **Run this before R-001 ships.** | M | R-001 |
| ☐ | **R-003** | **Is splitting the unsafe merges free?** **H:** vertex inflation from gated-weld-plus-split is **< 1%**, and self-intersections per 1k are **unchanged** from the unconditional weld. **Falsified by:** inflation > 1% (a real merge/split trade-off exists and needs a stated policy) **or** self-intersections rising — which would mean M-93's duplication artefact returns and the metric must be defined on welded output only. **FINDINGS:** `M-`, and an `E×-` entry if the gating is reverted on cost. | M | R-001 |

---

### 14b — Coordinate reconstruction is the crack source, not the algorithm

**Second recurring mechanism. Three measurements, three different subsystems, one cause:**

- **M-32** — *"Chunk seams are bit-exact only when the cell size is a power of two."*
- **M-49** — *"`ChunkLayout::cell_of` inverts `world_of_sample` inside a cell and not reliably on its corner — M-32 in a second place."*
- **M-73** — *"a transition cell that computes its sample positions by offsetting from a face origin puts a hairline crack in the seam, and no weld can close it."*

Every one is floating-point coordinate reconstruction, not extraction. **Nobody has published what
fraction of "seam cracking" in shipped voxel engines is this rather than algorithmic.**

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **R-004** | **Quantify the crack budget: arithmetic vs algorithm.** **H:** with exact/canonical coordinate reconstruction — one canonical `world_of_sample`, never an offset-and-add — seam cracks fall to **0 for all cell sizes**, not only powers of two, and M-73's hairline disappears without any change to the transition-cell construction. **Harness:** sweep non-power-of-two cell sizes × LOD pairs, count unmatched boundary edges and max vertical discontinuity (M-106's metric, which already found a margin across 495 seam crossings). **Records:** crack count and max discontinuity per (cell size, LOD pair), both arithmetic paths. **Falsified by:** cracks surviving canonical reconstruction — which localises the defect back in Transvoxel and is a different ticket. Consider Attene's **indirect predicates** (`10.1016/j.cad.2020.102856`, in corpus): treat a crossing as a *construction* (line, plane) rather than a computed point, and get exact sign tests at near-float cost. **FINDINGS:** `M-`, and `✗` against M-32's power-of-two framing if it turns out to be an artefact of one reconstruction choice rather than a floating-point law. | L | R-000 |

---

### 14c — Two mechanisms nobody has explained

| | ID | Ticket | Size | Blocked by |
|---|---|---|---|---|
| ☐ | **R-005** | **Why does the dual go superlinear where Marching Cubes does not?** (O-11, half-answered.) M-21: Surface Nets is not `O(n³)` over the range; Marching Cubes is. M-45: it reproduces on Zen 3 and gets *worse* there, so it is not one cache hierarchy — **the mechanism is still unknown**, and both machines show a per-sample **spike at 128³** specifically, which is a clue nobody has followed. **H:** the cost is the four-cells-around-a-crossed-edge gather at stride `n²`; cache-miss count per sample rises with `n` for Surface Nets and stays flat for Marching Cubes. **Harness:** hardware counters at 96³/128³/192³/256³ on both machines. **Falsified by:** flat miss rates — pointing at branch misprediction or allocation instead. **FINDINGS:** `M-`, and closing O-11 either way. | M | R-000 |
| ☐ | **R-006** | **A non-convergent error, which should not exist.** M-66: *"On a sharp field the geometry and the field disagree by an angle that does not fall with resolution."* Every other error in this crate falls with `h` — M-12's `h²`, M-65's `h²` on normals. **An error that does not converge is either a real property of sharp features or a bug, and both are worth knowing.** **H:** the angle is bounded below by the dihedral angle of the feature and is therefore a property of sharp edges rather than of resolution — so it should be *predictable from the field*, not merely observed. **Harness:** sweep dihedral angle on a wedge field × resolution; plot measured disagreement against predicted. **Records:** angle vs (dihedral, h). **Falsified by:** the angle failing to track the dihedral prediction — which makes it a defect with a location. **FINDINGS:** `M-`; if it is a bug, `✗` against M-66's framing as a property. | M | R-000 |

---

### Corpus hygiene found during this pass

`10.1142_s0218195912600060` (Attali, Lieutier & Salinas) — `paper_download` reported success and
fetched the **HAL landing page**; the markdown is French UI chrome plus an abstract. **A third
producer of the landing-page signature.** Purge and re-fetch from `hal.science/hal-00785082/document`.

One identifier to protect: **Dey, Edelsbrunner, Guha & Nekhayev, *Topology preserving edge
contraction* (1999) — the primary source for the link condition — has no DOI at all.** Semantic
Scholar carries it with `doi: null`. It is free at emis.de. **Do not let an agent invent one.**

---

## Deliberately not in scope yet

Recorded so they don't get picked up early, and so it's clear they weren't forgotten.

- **`O-17` — how much does a grid-edge root cache buy?** M-98 measured subgrid Marching Tetrahedra at 70× classic MT, and the constant is field evaluations: 576 per cell at 16 samples per edge, against 8 shared corner samples for Marching Cubes. Every cell currently re-finds the roots on edges its neighbours already found, deliberately — identical endpoints through a deterministic root finder is what makes conformity hold without a cache. A cache keyed on the grid edge is the obvious optimisation and the redundancy is large, but it has a correctness precondition and the saving is **unmeasured**. Settle it by caching and re-running `cargo bench --bench extract`, with the golden hashes as the guard that the mesh did not change.
- Nanite-style mesh-space cluster simplification — the research concludes it can't be repaired
  edit-proportionally (no local validity certificate). Field-derived LOD is the bet instead.
- Networked/concurrent editing — **closed out, not deferred. The verdict is in and it is bounded.**
  O-4 asked whether brush operations commute. They do, conditionally: a run of same-kind *hard* edits
  reorders bit-for-bit — one result from all 40,320 orderings, all `Add` and again all `Subtract`
  (M-36) — because `min`/`max` select an argument rather than computing a value. Across an add/subtract
  boundary they do **not**: 11 distinct results, and the difference is *semantic*, so no storage format
  or arithmetic repairs it (M-37). Smooth union is worse still — 40,317 distinct results from 40,320,
  smooth-min being neither associative nor bit-commutative (M-38).

  So the coordination-free story survives inside a run and dies at every boundary, and **that is a
  protocol's problem, not this crate's.** isomesh's whole obligation was to make the truth available,
  and `BrushOp::commutes_with` already returns the honest answer rather than the optimistic one. A
  networking layer needs sockets, clocks and a session model — none of which belong in a `no_std` crate
  whose public API is `[f32; 3]`. Nothing further is owed here; reopen it as a real ticket only if a
  consumer turns up needing something the existing predicate cannot express.
- Neural / differentiable extraction (FlexiCubes, TetWeave) — different problem, different crate.
- Publishing real releases to crates.io. **`I-005 — reserve the name` is done: `isomesh 0.0.0` was
  published on 2026-08-12, and `0.0.1` on 2026-08-13 to carry the crate's README** and the name is held. `megamesh` was taken 48 hours before we checked it,
  which is what made this urgent rather than tidy. The placeholder is 82 files / 329 KiB compressed —
  source, benches, golden hashes and proptest regressions, nothing stray — and `0.0.0` is now burned
  permanently, which is the intent.

  What stays out of scope is a **real** release. That wants a `crates/isomesh/README.md` (the root one
  is outside the package directory and cannot be referenced from it, so the crates.io page currently
  shows only the one-line description), a version policy, and a decision about whether `isomesh-gpu`
  and `bevy_isomesh` publish alongside it. None of that is urgent now the name cannot be taken.
