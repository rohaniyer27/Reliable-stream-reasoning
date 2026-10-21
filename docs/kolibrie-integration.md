# Kolibrie integration — first milestone

## Status

**Pending execution.** The workflow `.github/workflows/kolibrie-smoke.yml` runs two **existing upstream Kolibrie v0.2.0 tests** on GitHub Actions. It does not yet test our own correction/retraction hypotheses.

## Why these tests?

The upstream source at `kolibrie/tests/rsp_engine_test.rs` includes `rsp_probabilistic_occurrences_keep_stable_distinct_seed_ids`. It uses `add_probabilistic_to_stream` and verifies how probabilistic event identities are handled across windowed stream ingestion. The upstream source at `kolibrie/tests/probabilistic_program_test.rs` contains `probabilistic_rules_combine_every_proof_in_any_order`, which exercises multiple probabilistic derivations. These are verified *existing tests*, not original project contributions.

## Run the workflow

1. Go to **Actions → Kolibrie v0.2.0 smoke test → Run workflow** on branch `main`.
2. Inspect the job log and record the result, upstream commit SHA and Rust versions.
3. If Cargo compilation fails, inspect whether the upstream workspace package or test names changed; do not interpret build failures as research findings.

## Follow-on experiment (not implemented)

Implement a pinned Kolibrie harness that uses rules and probabilistic events equivalent to the Python reference, with explicit event-time windows, correction and retraction semantics. Compare only when the operations and inferred propositions represent the same semantics. Rebuilding a fresh engine from corrected events provides a baseline; differences under identical semantics are a correctness question, while deliberate approximation provides a research trade-off.

**No results from Kolibrie should be claimed until the smoke-test workflow has actually completed.**

### Upstream references

- [Kolibrie v0.2.0 release](https://github.com/StreamIntelligenceLab/Kolibrie/releases/tag/v0.2.0)
- [RSP engine tests](https://github.com/StreamIntelligenceLab/Kolibrie/blob/v0.2.0/kolibrie/tests/rsp_engine_test.rs)
- [Probabilistic program tests](https://github.com/StreamIntelligenceLab/Kolibrie/blob/v0.2.0/kolibrie/tests/probabilistic_program_test.rs)
