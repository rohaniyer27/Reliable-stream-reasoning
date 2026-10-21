# Kolibrie integration — first milestone

## Status

**Pending execution.** The workflow `.github/workflows/kolibrie-smoke.yml` runs two **existing upstream Kolibrie v0.2.0 tests** on GitHub Actions. It does not yet test our own correction/retraction hypotheses.

## Why these tests?

The upstream source at `kolibrie/tests/rsp_engine_test.rs` includes `rsp_probabilistic_occurrences_keep_stable_distinct_seed_ids`. It uses `add_probabilistic_to_stream` and verifies how probabilistic event identities are handled across windowed stream ingestion. The upstream source at `kolibrie/tests/probabilistic_program_test.rs` contains `probabilistic_rules_combine_every_proof_in_any_order`, which exercises multiple probabilistic derivations. These are verified *existing tests*, not original project contributions.

## Run the workflow

1. Go to **Actions → Kolibrie v0.2.0 smoke test → Run workflow** on branch `main`.
2. Inspect the job log and record the result, upstream commit SHA and Rust versions.
3. If Cargo compilation fails, inspect whether the upstream workspace package or test names changed; do not interpret build failures as research findings.


## First recorded exploratory result — 21 October 2026

GitHub Actions [run 39619121102](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/runs/39619121102) succeeded and executed `external_probe_late_probabilistic_observation` against the pinned upstream Kolibrie checkout. The test ingested timestamp 2 (p=0.8) before timestamp 1 (p=0.9) and printed:

```text
latest_hybrid_results exists: true
derived result count: 0
test result: ok. 1 passed; 0 failed
```

**Interpretation:** This is a successful smoke/probe execution, **not** evidence that Kolibrie incorrectly handles late observations. The exploratory test does not assert a nonempty result or establish an ordered-arrival control; no probability difference or correctness claim is supported. The test also uses an existing rule with a 0.7 threshold. Next, add matched ordered and reversed-arrival controls, inspect window firing, and assert expected outputs only after establishing equivalent temporal semantics.

## Follow-on experiment (not implemented)

Implement a pinned Kolibrie harness that uses rules and probabilistic events equivalent to the Python reference, with explicit event-time windows, correction and retraction semantics. Compare only when the operations and inferred propositions represent the same semantics. Rebuilding a fresh engine from corrected events provides a baseline; differences under identical semantics are a correctness question, while deliberate approximation provides a research trade-off.

Only the bounded observation above is verified. Correction/retraction comparisons and accuracy measurements remain pending.

### Upstream references

- [Kolibrie v0.2.0 release](https://github.com/StreamIntelligenceLab/Kolibrie/releases/tag/v0.2.0)
- [RSP engine tests](https://github.com/StreamIntelligenceLab/Kolibrie/blob/v0.2.0/kolibrie/tests/rsp_engine_test.rs)
- [Probabilistic program tests](https://github.com/StreamIntelligenceLab/Kolibrie/blob/v0.2.0/kolibrie/tests/probabilistic_program_test.rs)
