# Research concept

**Working title:** Reliable Neuro-Symbolic Stream Reasoning for Offshore Cognitive Digital Twins  
**Status:** Exploratory research framing; not a validated novelty claim.

## Motivation

Offshore wind turbines and energy islands are monitored through heterogeneous, sometimes correlated sensors. A cognitive digital twin must interpret timestamped observations, neural predictions and operational context. Late evidence, corrected readings and temporary outages complicate the validity of an alert, particularly when processing resources or network connectivity are limited.

Kolibrie v0.2.0 already includes temporal DatalogMTL reasoning, incremental cross-window evaluation, neural learning, probabilistic provenance and SDD-based weighted model counting. The research therefore starts by **characterising the interaction** of these capabilities under explicitly stated assumptions.

## Research questions

1. How do event-time versus processing-time semantics, observation retractions and window expiry affect derived probabilistic conclusions? What quality is lost when approximations are intentionally introduced?
2. For neural predictions and sensor readings subject to shared underlying causes, how do dependency assumptions affect the calibration of derived alerts?
3. How do inference quality, stale-conclusion duration, runtime and memory requirements trade off under constrained edge processing?

Reliability refers to the **temporal validity** of conclusions, the **calibration** of derived probabilities relative to a declared prediction target, and how long invalid conclusions remain active.

## Staged methodology

**Stage 1 — Synthetic semantic probe.** Specify window and correction semantics explicitly. Compare independently recomputed reference results with results from a pinned version of Kolibrie. Separate implementation correctness checks from research claims. No Kolibrie probe result is currently reported.

**Stage 2 — Neural-symbolic case study.** Model turbine components, sensors and operational context in an RDF knowledge graph. Use an ML model to generate probabilistic observations. Measure neural calibration separately from symbolic calibration. Define a fault-within-horizon target and guard against temporal and turbine-level leakage.

**Stage 3 — Conditional extension.** If the results justify it, compare dependence-aware inference or approximate selective recomputation under fixed latency and memory budgets.

## Data limitations

CARE to Compare provides anonymised SCADA summaries and fault-related anomaly datasets, not raw arrival traces. Injected delays, corrections and outages would be **simulated** and clearly separated from observed faults. Prioritise screened average channels because the data documentation notes implausible minimum, maximum and standard-deviation values. Small numbers of fault events require uncertainty intervals and cautious calibration claims. Dataset licence and redistribution rights require verification.

## Expected contribution

A reproducible evaluation of when established reasoning mechanisms are sufficient, and when dependency or resource constraints justify targeted extensions. The initial code is a reference model only; no novel algorithm or validated Kolibrie deficiency is claimed.
