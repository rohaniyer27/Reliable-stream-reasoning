# Experimental protocol — v0.1

## Purpose

Test well-defined semantics before making an algorithmic claim. The local Python reference is **independent of Kolibrie**. Experiments against Kolibrie v0.2.0 remain pending.

## Hypotheses to examine (not established results)

- H1: Under identical event-time semantics and exact evaluation, maintained and recomputed probabilities should agree within floating-point tolerance.
- H2: Different window/arrival policies can produce different intermediate conclusions without either implementation being incorrect.
- H3: Approximate or dependence-agnostic inference may trade accuracy for compute resources; magnitude is empirical.

## Synthetic test cases

| Case | Input manipulation | Required observation |
| --- | --- | --- |
| Ordered | Two observations at event times 10 and 12 | Baseline derived probability |
| Late | Reverse arrival order | Result under stated watermark/window policy |
| Corrected | Replace stable event ID at time 12 with new probability | Revised conclusion |
| Retracted | Delete observation by ID | Withdrawal/change of supported conclusion |
| Expired | Advance cutoff to 20 with width 5 | No active observations |

For the reference noisy-OR with probabilities 0.6 and 0.5: ordered 0.80; correction from 0.5 to 0.2 gives 0.68; removal of the 0.6 fact after that gives 0.20; expiry yields 0.00. These are **reference calculations, not Kolibrie measurements**.

## Reproduction protocol

1. Pin the Kolibrie tag **v0.2.0** and record its resolved commit SHA, Rust version, host OS and enabled features.
2. Run its own tests before applying the experiment harness.
3. Specify the semantics of arrival time, event time, watermark (if any), inclusive/exclusive window bounds, fact identity, and update/retraction propagation.
4. Implement equivalent rules and data in Kolibrie; retain source code and exact commands.
5. Compare against fresh recomputation from the same corrected and in-window evidence.
6. Log processing latency (including p50/p95), maximum resident memory, differences in conclusions and version/configuration.
7. Label failures caused by incompatible semantics separately from mismatches under equivalent semantics.

## Measures

- **Absolute probability difference:** `abs(p_incremental - p_reference)`, only where both results represent the same event and temporal semantics.
- **Staleness:** elapsed processing time between an evidence revision and corresponding conclusion revision.
- **Calibration:** Brier score and carefully uncertainty-qualified calibration plots versus a predefined fault-within-horizon target; report neural and derived calibration separately.
- **Resources:** median and tail latency, throughput and peak memory.

No predictive calibration measure is meaningful without a clearly specified label and prediction horizon. Do not use the synthetic noisy-OR probabilities as ground-truth fault forecasts.

## Limitations and research integrity

A correctly implemented exact incremental reasoner is expected to match fresh recomputation. Agreement does not by itself prove novel research, and mismatch does not necessarily prove a scientific limitation. Correlated-sensor analysis requires either known simulated dependencies or defensible empirical assumptions. Disruptions added to CARE summaries are synthetic, not recorded communication events. Check data licences before any public data derivative.
