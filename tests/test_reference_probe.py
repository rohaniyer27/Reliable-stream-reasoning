"""Regression tests for reference semantics only (not Kolibrie)."""
import unittest

from src.reference_probe import Observation, active_at, evaluate


class ReferenceProbeTests(unittest.TestCase):
    def setUp(self) -> None:
        self.events = {
            "v": Observation("v", 10, 0.6),
            "t": Observation("t", 12, 0.5),
        }

    def test_initial_probability(self) -> None:
        self.assertAlmostEqual(evaluate(self.events.values(), 12, 5), 0.8)

    def test_out_of_order_delivery_under_event_time_reference(self) -> None:
        reverse_delivery = list(reversed(list(self.events.values())))
        self.assertAlmostEqual(evaluate(reverse_delivery, 12, 5), 0.8)

    def test_correction(self) -> None:
        self.events["t"] = Observation("t", 12, 0.2)
        self.assertAlmostEqual(evaluate(self.events.values(), 12, 5), 0.68)

    def test_retraction(self) -> None:
        self.events.pop("v")
        self.assertAlmostEqual(evaluate(self.events.values(), 12, 5), 0.5)

    def test_expiration(self) -> None:
        self.assertAlmostEqual(evaluate(self.events.values(), 20, 5), 0.0)

    def test_lower_bound_exclusive(self) -> None:
        self.assertEqual(active_at(self.events.values(), 15, 5), [self.events["t"]])

    def test_invalid_probability(self) -> None:
        with self.assertRaises(ValueError):
            Observation("bad", 1, 1.2)

    def test_invalid_width(self) -> None:
        with self.assertRaises(ValueError):
            evaluate(self.events.values(), 12, 0)


if __name__ == "__main__":
    unittest.main()
