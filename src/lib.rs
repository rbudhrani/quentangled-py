use num_complex::Complex64;
use pyo3::exceptions::{PyRuntimeError, PyValueError};
use pyo3::prelude::*;
use quentangled::{Circuit, ConfigError, SimulationResult, Simulator, SimulatorConfig};
use std::path::PathBuf;

#[pyclass(name = "Circuit")]
/// A validated quantum circuit built from indexed qubit gates.
struct PyCircuit {
    inner: Circuit,
}

#[pymethods]
impl PyCircuit {
    #[new]
    /// Create an empty circuit containing the requested number of qubits.
    ///
    /// Args:
    ///     num_qubits: Number of qubits in the circuit. Qubits are indexed
    ///         from zero through ``num_qubits - 1``.
    ///
    /// Returns:
    ///     A new empty :class:`Circuit`.
    ///
    /// Raises:
    ///     ValueError: If ``num_qubits`` is negative.
    fn new(#[pyo3(from_py_with = nonnegative_usize)] num_qubits: usize) -> Self {
        Self {
            inner: Circuit::new(num_qubits),
        }
    }

    #[getter]
    /// Get the number of qubits in this circuit.
    ///
    /// Returns:
    ///     The number of qubits as an integer.
    fn num_qubits(&self) -> usize {
        self.inner.num_qubits()
    }

    /// Apply an X (bit-flip) gate.
    ///
    /// Args:
    ///     target: Index of the qubit to operate on.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If ``target`` is outside the circuit.
    fn x(&mut self, #[pyo3(from_py_with = nonnegative_usize)] target: usize) -> PyResult<()> {
        self.inner.x(target).map(|_| ()).map_err(circuit_error)
    }

    /// Apply a Y gate.
    ///
    /// Args:
    ///     target: Index of the qubit to operate on.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If ``target`` is outside the circuit.
    fn y(&mut self, #[pyo3(from_py_with = nonnegative_usize)] target: usize) -> PyResult<()> {
        self.inner.y(target).map(|_| ()).map_err(circuit_error)
    }

    /// Apply a Z (phase-flip) gate.
    ///
    /// Args:
    ///     target: Index of the qubit to operate on.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If ``target`` is outside the circuit.
    fn z(&mut self, #[pyo3(from_py_with = nonnegative_usize)] target: usize) -> PyResult<()> {
        self.inner.z(target).map(|_| ()).map_err(circuit_error)
    }

    /// Apply a Hadamard gate.
    ///
    /// Args:
    ///     target: Index of the qubit to operate on.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If ``target`` is outside the circuit.
    fn h(&mut self, #[pyo3(from_py_with = nonnegative_usize)] target: usize) -> PyResult<()> {
        self.inner.h(target).map(|_| ()).map_err(circuit_error)
    }

    /// Apply a phase gate.
    ///
    /// Args:
    ///     target: Index of the qubit to operate on.
    ///     theta: Phase angle in radians.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If ``target`` is outside the circuit or ``theta`` is
    ///         not finite.
    fn phase(
        &mut self,
        #[pyo3(from_py_with = nonnegative_usize)] target: usize,
        theta: f64,
    ) -> PyResult<()> {
        self.inner
            .phase(target, theta)
            .map(|_| ())
            .map_err(circuit_error)
    }

    #[pyo3(signature = (target, matrix, error_matrix=None, duration_ns=0.0))]
    /// Apply a custom 2x2 gate to one qubit.
    ///
    /// Matrix entries are supplied in row-major order. If ``error_matrix`` is
    /// supplied, it is applied as the gate's error model. ``duration_ns`` is
    /// recorded for simulator duration estimates.
    ///
    /// Args:
    ///     target: Index of the qubit to operate on.
    ///     matrix: Four complex values representing the 2x2 gate matrix in
    ///         row-major order.
    ///     error_matrix: Optional four complex values representing a 2x2
    ///         error matrix in row-major order. Defaults to ``None``.
    ///     duration_ns: Gate duration in nanoseconds. Defaults to ``0.0``.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If either matrix does not contain exactly four complex
    ///         values, if ``target`` is outside the circuit, or if the gate
    ///         parameters are invalid.
    fn unitary(
        &mut self,
        #[pyo3(from_py_with = nonnegative_usize)] target: usize,
        matrix: &Bound<'_, PyAny>,
        error_matrix: Option<&Bound<'_, PyAny>>,
        duration_ns: f64,
    ) -> PyResult<()> {
        let matrix = matrix_array(matrix)?;
        let error_matrix = error_matrix.map(matrix_array).transpose()?;
        self.inner
            .unitary(target, matrix, error_matrix, duration_ns)
            .map(|_| ())
            .map_err(circuit_error)
    }

    /// Apply a controlled-NOT (CNOT) gate.
    ///
    /// Args:
    ///     control: Index of the control qubit.
    ///     target: Index of the target qubit.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If either qubit is outside the circuit or the control
    ///         and target are the same qubit.
    fn cnot(
        &mut self,
        #[pyo3(from_py_with = nonnegative_usize)] control: usize,
        #[pyo3(from_py_with = nonnegative_usize)] target: usize,
    ) -> PyResult<()> {
        self.inner
            .cnot(control, target)
            .map(|_| ())
            .map_err(circuit_error)
    }

    /// Apply a controlled-Z (CZ) gate.
    ///
    /// Args:
    ///     control: Index of the control qubit.
    ///     target: Index of the target qubit.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If either qubit is outside the circuit or the control
    ///         and target are the same qubit.
    fn cz(
        &mut self,
        #[pyo3(from_py_with = nonnegative_usize)] control: usize,
        #[pyo3(from_py_with = nonnegative_usize)] target: usize,
    ) -> PyResult<()> {
        self.inner
            .cz(control, target)
            .map(|_| ())
            .map_err(circuit_error)
    }

    /// Swap the states of two qubits.
    ///
    /// Args:
    ///     q1: Index of the first qubit.
    ///     q2: Index of the second qubit.
    ///
    /// Returns:
    ///     ``None``.
    ///
    /// Raises:
    ///     ValueError: If either qubit is outside the circuit or both indices
    ///         refer to the same qubit.
    fn swap(
        &mut self,
        #[pyo3(from_py_with = nonnegative_usize)] q1: usize,
        #[pyo3(from_py_with = nonnegative_usize)] q2: usize,
    ) -> PyResult<()> {
        self.inner.swap(q1, q2).map(|_| ()).map_err(circuit_error)
    }
}

#[pyclass(name = "SimulationResult")]
/// Final state amplitudes and measurement probabilities from a simulation.
struct PySimulationResult {
    inner: SimulationResult,
}

#[pymethods]
impl PySimulationResult {
    #[getter]
    /// Get the number of qubits represented by this result.
    ///
    /// Returns:
    ///     The number of qubits as an integer.
    fn num_qubits(&self) -> usize {
        self.inner.num_qubits()
    }

    /// Return the state-vector amplitudes.
    ///
    /// Amplitudes are ordered by basis-state index, from zero to
    /// ``2**num_qubits - 1``.
    ///
    /// Returns:
    ///     A list of complex amplitudes.
    fn amplitudes(&self) -> Vec<Complex64> {
        self.inner.amplitudes().to_vec()
    }

    /// Return measurement probabilities for every basis state.
    ///
    /// Probabilities are ordered by basis-state index, from zero to
    /// ``2**num_qubits - 1``.
    ///
    /// Returns:
    ///     A list of floating-point probabilities.
    fn probabilities(&self) -> Vec<f64> {
        self.inner.probabilities()
    }

    /// Return the probability of one basis state.
    ///
    /// Args:
    ///     basis_state: Integer index of the computational basis state.
    ///
    /// Returns:
    ///     The state's probability, or ``None`` if ``basis_state`` is outside
    ///     the range represented by this result.
    fn probability(
        &self,
        #[pyo3(from_py_with = probability_index)] basis_state: Option<usize>,
    ) -> Option<f64> {
        basis_state.and_then(|index| self.inner.probability(index))
    }
}

#[pyclass(name = "Simulator")]
/// Simulator configured for one execution engine and set of gate parameters.
struct PySimulator {
    inner: Simulator,
}

#[pymethods]
impl PySimulator {
    #[new]
    #[pyo3(signature = (config_path=None))]
    /// Create a simulator, optionally loading settings from a TOML file.
    ///
    /// With no path, the simulator uses the default configuration.
    ///
    /// Args:
    ///     config_path: Optional path to a TOML configuration file. Accepts a
    ///         string or path-like object. Defaults to ``None``.
    ///
    /// Returns:
    ///     A configured :class:`Simulator`.
    ///
    /// Raises:
    ///     ValueError: If the configuration contains invalid settings or the
    ///         simulator cannot be created from them.
    ///     RuntimeError: If the file cannot be read or parsed.
    fn new(config_path: Option<PathBuf>) -> PyResult<Self> {
        let config = match config_path {
            Some(path) => SimulatorConfig::from_file(path).map_err(|error| {
                if error.downcast_ref::<ConfigError>().is_some() {
                    PyValueError::new_err(error.to_string())
                } else {
                    PyRuntimeError::new_err(format!("could not load simulator config: {error}"))
                }
            })?,
            None => SimulatorConfig::default(),
        };
        let inner =
            Simulator::new(config).map_err(|error| PyValueError::new_err(error.to_string()))?;
        Ok(Self { inner })
    }

    /// Execute a circuit and return its final state.
    ///
    /// Args:
    ///     circuit: Circuit to simulate.
    ///
    /// Returns:
    ///     A :class:`SimulationResult` containing the final state amplitudes
    ///     and probabilities.
    ///
    /// The Rust simulation runs without holding Python's GIL, allowing other
    /// Python threads to run while the circuit is executing.
    fn execute(&self, py: Python<'_>, circuit: PyRef<'_, PyCircuit>) -> PySimulationResult {
        let simulator = self.inner.clone();
        let circuit_snapshot = circuit.inner.clone();
        drop(circuit);

        PySimulationResult {
            inner: py.detach(move || simulator.execute(&circuit_snapshot)),
        }
    }

    /// Estimate a circuit's total duration.
    ///
    /// The estimate is based on the configured durations of the gates in the
    /// circuit; it does not measure wall-clock execution time.
    ///
    /// Args:
    ///     circuit: Circuit whose duration should be estimated.
    ///
    /// Returns:
    ///     Estimated duration in nanoseconds as a floating-point number.
    fn estimated_duration_ns(&self, circuit: PyRef<'_, PyCircuit>) -> f64 {
        self.inner.estimated_duration_ns(&circuit.inner)
    }
}

fn matrix_array(matrix: &Bound<'_, PyAny>) -> PyResult<[Complex64; 4]> {
    // Extract explicitly because PyO3 does not accept Vec<Complex64> as a direct argument.
    let matrix: Vec<Complex64> = matrix.extract()?;
    matrix.try_into().map_err(|matrix: Vec<Complex64>| {
        PyValueError::new_err(format!(
            "a 2x2 matrix must contain exactly 4 complex values, got {}",
            matrix.len()
        ))
    })
}

fn nonnegative_usize(value: &Bound<'_, PyAny>) -> PyResult<usize> {
    match value.extract::<usize>() {
        Ok(index) => Ok(index),
        Err(error) => {
            if value.is_instance_of::<pyo3::types::PyInt>() && value.lt(0)? {
                Err(PyValueError::new_err("value must be non-negative"))
            } else {
                Err(error)
            }
        }
    }
}

fn probability_index(value: &Bound<'_, PyAny>) -> PyResult<Option<usize>> {
    if value.is_instance_of::<pyo3::types::PyInt>() && value.lt(0)? {
        return Ok(None);
    }
    value.extract::<usize>().map(Some)
}

fn circuit_error(error: quentangled::CircuitError) -> PyErr {
    PyValueError::new_err(error.to_string())
}

#[pymodule]
fn _quentangled(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<PyCircuit>()?;
    module.add_class::<PySimulationResult>()?;
    module.add_class::<PySimulator>()?;
    Ok(())
}
