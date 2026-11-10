import math
import tempfile
import threading
import unittest
from pathlib import Path

from quentangled import Circuit, Simulator


class CircuitApiTests(unittest.TestCase):
    def assert_probability_almost_equal(self, result, basis_state, expected):
        probability = result.probability(basis_state)
        if probability is None:
            self.fail(f"basis state {basis_state} is out of range")
        self.assertAlmostEqual(probability, expected)

    def test_bell_state_results_in_entanglement(self):
        circuit = Circuit(2)
        circuit.h(0)
        circuit.cnot(0, 1)

        result = Simulator().execute(circuit)

        self.assertEqual(result.num_qubits, 2)
        self.assert_probability_almost_equal(result, 0, 0.5)
        self.assert_probability_almost_equal(result, 3, 0.5)
        self.assert_probability_almost_equal(result, 1, 0.0)
        self.assertEqual(len(result.amplitudes()), 4)

    def test_execute_releases_gil_for_other_python_threads(self):
        circuit = Circuit(22)
        for qubit in range(circuit.num_qubits):
            circuit.h(qubit)

        other_thread_ran = threading.Event()
        worker = threading.Timer(0.01, other_thread_ran.set)
        worker.start()
        result = Simulator().execute(circuit)
        ran_during_execution = other_thread_ran.is_set()
        worker.join()

        self.assertTrue(ran_during_execution)
        self.assertEqual(result.num_qubits, 22)

    def test_gate_validation_is_reported(self):
        circuit = Circuit(1)

        with self.assertRaisesRegex(ValueError, "out of bounds"):
            circuit.x(1)

        with self.assertRaisesRegex(ValueError, "cannot fill multiple"):
            circuit.cnot(0, 0)

    def test_negative_qubit_counts_and_gate_indices_raise_value_error(self):
        with self.assertRaisesRegex(ValueError, "non-negative"):
            Circuit(-1)

        with self.assertRaisesRegex(ValueError, "non-negative"):
            Circuit(1).x(-(1 << 200))

        with self.assertRaisesRegex(ValueError, "non-negative"):
            Circuit(2).cnot(-1, 0)

    def test_negative_probability_index_returns_none(self):
        result = Simulator().execute(Circuit(1))

        self.assertIsNone(result.probability(-1))
        self.assertIsNone(result.probability(-(1 << 200)))

    def test_remaining_standard_gates(self):
        simulator = Simulator()

        y_circuit = Circuit(1)
        y_circuit.y(0)
        self.assert_probability_almost_equal(simulator.execute(y_circuit), 1, 1.0)

        z_circuit = Circuit(1)
        z_circuit.x(0)
        z_circuit.z(0)
        self.assert_probability_almost_equal(simulator.execute(z_circuit), 1, 1.0)

        cz_circuit = Circuit(2)
        cz_circuit.x(0)
        cz_circuit.x(1)
        cz_circuit.cz(0, 1)
        self.assertAlmostEqual(simulator.execute(cz_circuit).amplitudes()[3].real, -1.0)

        swap_circuit = Circuit(2)
        swap_circuit.x(0)
        swap_circuit.swap(0, 1)
        self.assert_probability_almost_equal(simulator.execute(swap_circuit), 2, 1.0)

    def test_phase_rejects_non_finite_angle(self):
        with self.assertRaisesRegex(ValueError, "must be finite"):
            Circuit(1).phase(0, math.nan)

    def test_custom_unitary_validates_matrix_size_and_duration(self):
        circuit = Circuit(1)

        with self.assertRaisesRegex(ValueError, "exactly 4 complex values"):
            circuit.unitary(0, [1 + 0j])

        with self.assertRaisesRegex(ValueError, "duration"):
            circuit.unitary(0, [1 + 0j, 0j, 0j, 1 + 0j], duration_ns=-1)

    def test_custom_unitary_executes(self):
        circuit = Circuit(1)
        circuit.unitary(0, [0j, 1 + 0j, 1 + 0j, 0j], duration_ns=3.0)
        simulator = Simulator()

        self.assert_probability_almost_equal(simulator.execute(circuit), 1, 1.0)
        self.assertEqual(simulator.estimated_duration_ns(circuit), 3.0)

    def test_custom_unitary_accepts_optional_error_matrix(self):
        circuit = Circuit(1)
        circuit.unitary(
            0,
            [0j, 1 + 0j, 1 + 0j, 0j],
            error_matrix=[1 + 0j, 0j, 0j, -1 + 0j],
        )

        result = Simulator().execute(circuit)

        self.assert_probability_almost_equal(result, 1, 1.0)
        self.assertAlmostEqual(result.amplitudes()[1].real, -1.0)

    def test_simulator_loads_configuration(self):
        with tempfile.TemporaryDirectory() as directory:
            config_path = Path(directory) / "quentangled.toml"
            config_path.write_text(
                "[engine]\nparallel_threshold = 1\n[gates.x]\ntime_ns = 7.0\n",
                encoding="utf-8",
            )
            circuit = Circuit(1)
            circuit.x(0)

            simulator = Simulator(config_path)

            self.assertEqual(simulator.estimated_duration_ns(circuit), 7.0)
            self.assert_probability_almost_equal(simulator.execute(circuit), 1, 1.0)

    def test_invalid_simulator_configuration_is_reported(self):
        with tempfile.TemporaryDirectory() as directory:
            config_path = Path(directory) / "quentangled.toml"
            config_path.write_text("[engine]\nchunk_size = 0\n", encoding="utf-8")

            with self.assertRaisesRegex(ValueError, "chunk_size"):
                Simulator(config_path)

    def test_missing_simulator_configuration_is_reported(self):
        with tempfile.TemporaryDirectory() as directory:
            config_path = Path(directory) / "missing.toml"

            with self.assertRaisesRegex(
                RuntimeError, "could not load simulator config"
            ):
                Simulator(config_path)


if __name__ == "__main__":
    unittest.main()
