# Reliable Stream Reasoning

**Exploring temporal and probabilistic reasoning under evolving evidence**

[![Python reference tests](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/workflows/reference-tests.yml/badge.svg)](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/workflows/reference-tests.yml)
[![Kolibrie smoke test](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/workflows/kolibrie-smoke.yml/badge.svg)](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/workflows/kolibrie-smoke.yml)

An independent, reproducible research prototype investigating how temporal and probabilistic stream-reasoning systems behave when evidence arrives out of order, changes, or expires. Offshore cognitive digital twins motivate the work; the first experiments use synthetic observations to isolate system semantics.

> **Status:** Exploratory. Reference tests and a Kolibrie smoke test have passed; controlled comparison results are pending. No novel method or Kolibrie defect is claimed.

## Research focus

Operational systems combine sensor observations, learned predictions, knowledge-graph context, and temporal rules. Their outputs depend on how information is timestamped, revised, and interpreted. The investigation centres on three questions:

1. **Temporal validity:** Under which event-time and window semantics should derived conclusions change when observations arrive late, are corrected, or are retracted?
2. **Probabilistic reliability:** How do dependencies between sensor observations and neural predictions affect the calibration of inferred conclusions?
3. **Practical cost:** How do inference accuracy, conclusion staleness, latency, and memory use interact under constrained edge-computing budgets?

The project builds on established work. **Kolibrie v0.2.0** already includes DatalogMTL, incremental cross-window reasoning, neural relations, probabilistic provenance, SDD-based weighted model counting, and SPARQL updates. These are existing upstream capabilities—not claimed contributions of this repository.

## Progress

| Milestone | Status | Evidence |
| --- | --- | --- |
| Python event-time reference model | Complete | [Implementation](src/reference_probe.py) |
| Python regression tests | Passed in GitHub Actions | [Workflow](.github/workflows/reference-tests.yml) |
| Kolibrie v0.2.0 upstream probabilistic tests | Passed | [Verified run](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/runs/39619121102) |
| Initial late-observation probe | Executed; produced zero derived results under initial configuration | [Research log](docs/kolibrie-integration.md) |
| Matched ordered / reversed-arrival comparison | **Passed — [0.8, 0.9] in both conditions** | [Verified run](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/runs/39631296715) · [Rust test](experiments/late_observation_probe.rs) |
| Correction and retraction comparison | Not implemented | [Experiment protocol](docs/experiment_protocol.md) |
| Real turbine-data case study | Not started | [Research concept](docs/research_concept.md) |

## Preliminary experimental observations

### Python reference model

The dependency-free reference model defines the event-time window as `cutoff - width < event_time <= cutoff`. Corrections replace facts by stable ID; retractions remove them. An **independent noisy-OR** is used solely for illustrative probability calculations. For example, independent inputs of 0.6 and 0.5 yield 0.8. These synthetic probabilities are **not calibrated fault risks**.

### Kolibrie integration — 21 October 2026

The first exploratory probe executed, but returned zero derived results in its initial configuration. We subsequently introduced matched **ordered** and **reversed-arrival** controls with assertions requiring nonempty outputs and matching probability vectors.

The revised Kolibrie v0.2.0 test passed on GitHub Actions:

```text
reverse=false: exact probabilities=[0.8, 0.9]
reverse=true: exact probabilities=[0.8, 0.9]
test external_probe_late_probabilistic_observation ... ok
```

The maximum absolute difference between the **sorted** probability vectors was **0.00**. This demonstrates consistency for these observations under the tested rule and window configuration, not general late-event correctness: fact identity, provenance, and intermediate states were not compared. Correction and retraction experiments remain future work.

See the [experiment log](docs/kolibrie-integration.md) and [verified Actions run](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/runs/39631296715).

## Reproduce

### Python reference

Requires Python 3.10+; no external Python dependencies.

```bash
python -m unittest discover -s tests -v
python -m src.reference_probe
```

### Kolibrie v0.2.0

Use [Actions → Kolibrie v0.2.0 smoke test](https://github.com/rohaniyer27/Reliable-stream-reasoning/actions/workflows/kolibrie-smoke.yml) and choose **Run workflow** on `main`. The workflow checks out the upstream tagged release, installs Rust, builds the relevant tests, and appends the [exploratory Rust probe](experiments/late_observation_probe.rs) to the upstream integration test module within the temporary CI workspace. It does **not** modify the upstream Kolibrie repository.

The Rust build can take substantially longer than the Python test job. Consult the workflow log for the exact upstream commit and toolchain versions.

## Repository layout

```text
.github/workflows/
  reference-tests.yml          Python unit tests
  kolibrie-smoke.yml           Pinned upstream build and exploratory test
src/reference_probe.py         Synthetic event-time reference model
tests/test_reference_probe.py  Python regression tests
experiments/
  late_observation_probe.rs    Custom Kolibrie integration probe
docs/
  research_concept.md          Motivation and research questions
  experiment_protocol.md       Planned controlled comparisons
  kolibrie-integration.md      Reproducibility notes and observed run
data/                           Empty; external data excluded
results/                        Empty; no benchmark results published
```

## Future evaluation and limitations

The proposed offshore case study may draw on the [CARE to Compare](https://zenodo.org/records/14006163) wind-turbine SCADA dataset. These are anonymised historical summaries—not raw sensor-message arrival logs—so delays, corrections, outages, and shared-cause dependencies would need to be simulated and explicitly labelled. Data quality, licensing, ground-truth fault horizons, small-event uncertainty, and leakage prevention must be resolved before making predictive-performance claims. No CARE data have been analysed or bundled here.

## References

- [Stream Intelligence Lab — Kolibrie](https://github.com/StreamIntelligenceLab/Kolibrie)
- [Kolibrie v0.2.0 release notes](https://github.com/StreamIntelligenceLab/Kolibrie/releases/tag/v0.2.0)
- [Research on uncertainty and provenance for RDF streams](https://ceur-ws.org/Vol-4293/paper18.pdf)
- [CARE to Compare — wind-turbine data](https://zenodo.org/records/14006163)

---

Independent project; not affiliated with KU Leuven or its Stream Intelligence Lab.
