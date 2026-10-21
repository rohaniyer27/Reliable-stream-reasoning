# Reliability-Aware Stream Reasoning for Offshore Digital Twins

An independent, reproducible research workspace exploring reliability in probabilistic and temporal stream reasoning for offshore digital twins. The focus is on **probabilistic and temporal stream reasoning**, evidence revision, and evaluation under constrained computing resources.

> [!IMPORTANT]
> **Research status: exploratory.** The reference experiments in this repository are written in Python and **do not run Kolibrie**. They demonstrate specified event-time semantics, not a defect in Kolibrie, a new inference algorithm, or results from offshore SCADA data.

## Research questions

1. Under precisely defined event-time and retraction semantics, how do evolving observations affect derived probabilistic conclusions?
2. How do dependence assumptions between neural predictions and sensors affect the calibration of symbolic conclusions?
3. What accuracy, staleness, latency, and memory trade-offs arise from approximate reasoning under an edge-processing budget?

Kolibrie v0.2.0 already supports DatalogMTL, cross-window incremental stream reasoning, probabilistic provenance, weighted model counting, neural relations, and SPARQL updates. The objective here is to study the **interactions and empirical boundaries** of established capabilities, not to claim their invention.

## Current scope

The first milestone is a small, dependency-free Python **reference evaluator** for timestamped observations. Its semantics are deliberate and simple:

- Event-time window: `cutoff - width < event_time <= cutoff`
- Correction: replace an observation using its stable event ID
- Retraction: remove an observation using its stable event ID
- Derived probability: independent noisy-OR across active observations, for illustration only

The reference probability is **not a calibrated real-world fault probability**. Tests exercise window expiry, out-of-order delivery, correction, and retraction. No Kolibrie output has yet been measured.

### Run locally

Requires Python 3.10+ and no third-party packages.

```bash
python -m unittest discover -s tests -v
python -m src.reference_probe
```

## Repository layout

```text
src/reference_probe.py        Independent event-time probability reference
tests/test_reference_probe.py Reference semantics and regression tests
docs/research_concept.md      Research motivation, questions and scope
docs/experiment_protocol.md   Controlled probe design and comparison metrics
data/.gitkeep                 Placeholder (raw SCADA data is excluded)
```

## Next milestone: comparison with Kolibrie

Pin Kolibrie to **v0.2.0** and record the commit SHA, commands and environment. Translate the reference scenarios into supported Kolibrie stream/rule APIs, confirm how event-time and window semantics map, and compare results against fresh recomputation under identical semantics. Any semantic mismatch must be documented before interpreting probability differences.

See [Experiment protocol](docs/experiment_protocol.md) for the planned procedure and validity safeguards.

## Data and integrity

The proposed CARE to Compare wind-turbine case study uses anonymised SCADA summaries, **not original message-arrival logs**. Delays, corrections, sensor outages, and shared-cause dependencies would be controlled synthetic interventions. Fault prediction horizons, train/test boundaries and label suitability must be established before reporting calibration. Verify reuse terms before downloading or redistributing source data. **No external dataset is included in this repository.**

## References

- [Kolibrie — repository](https://github.com/StreamIntelligenceLab/Kolibrie)
- [Kolibrie v0.2.0 release notes](https://github.com/StreamIntelligenceLab/Kolibrie/releases/tag/v0.2.0)
- [Uncertainty annotations and provenance in RDF streams (CEUR)](https://ceur-ws.org/Vol-4293/paper18.pdf)
- [CARE to Compare dataset (Zenodo)](https://zenodo.org/records/14006163)

Independent research prototype. Not affiliated with, endorsed by, or an official project of KU Leuven or the Stream Intelligence Lab.
