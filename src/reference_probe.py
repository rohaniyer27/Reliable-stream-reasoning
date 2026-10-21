"""Independent event-time reference; this module does NOT invoke Kolibrie.

An observation is active at cutoff T when T-width < event_time <= T.
A correction replaces an observation by its stable identifier; a retraction
removes that observation. The noisy-OR assumes independent input events.
Its result is illustrative, not a calibrated turbine fault probability.
"""
from dataclasses import dataclass
from typing import Iterable


@dataclass(frozen=True)
class Observation:
    event_id: str
    event_time: int
    probability: float

    def __post_init__(self) -> None:
        if not (0.0 <= self.probability <= 1.0):
            raise ValueError("probability must lie in [0, 1]")


def active_at(events: Iterable[Observation], cutoff: int, width: int) -> list[Observation]:
    """Return observations active in the half-open-left event-time window."""
    if width <= 0:
        raise ValueError("width must be positive")
    return [e for e in events if cutoff - width < e.event_time <= cutoff]


def independent_or_probability(events: Iterable[Observation]) -> float:
    """Illustrative P(any event) assuming independent Bernoulli facts."""
    complement = 1.0
    for event in events:
        complement *= 1.0 - event.probability
    return 1.0 - complement


def evaluate(events: Iterable[Observation], cutoff: int, width: int) -> float:
    return independent_or_probability(active_at(events, cutoff, width))


def main() -> None:
    observations = {
        "vibration": Observation("vibration", 10, 0.6),
        "temperature": Observation("temperature", 12, 0.5),
    }
    print(f"Initial: {evaluate(observations.values(), 12, 5):.4f}")
    observations["temperature"] = Observation("temperature", 12, 0.2)
    print(f"After correction: {evaluate(observations.values(), 12, 5):.4f}")
    observations.pop("vibration")
    print(f"After retraction: {evaluate(observations.values(), 12, 5):.4f}")
    print(f"After expiry: {evaluate(observations.values(), 20, 5):.4f}")


if __name__ == "__main__":
    main()
