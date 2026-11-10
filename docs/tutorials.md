# Tutorials

## Build a Bell state

Apply a Hadamard gate to the first qubit, then a controlled-NOT:

```python
from quentangled import Circuit, Simulator

circuit = Circuit(2)
circuit.h(0)
circuit.cnot(control=0, target=1)
result = Simulator().execute(circuit)

for index, probability in enumerate(result.probabilities()):
    print(index, probability)
```

The resulting state has equal probability for basis states `00` and `11`.
The simulator returns the full state vector; it does not sample measurement
shots.

`Simulator.execute()` releases Python's GIL while the Rust backend simulates
the circuit. This lets other Python threads make progress during longer
simulations; the execution uses the circuit state as it was when the call
started.

## Compose gates

`Circuit` supports `x`, `y`, `z`, `h`, `phase`, `cnot`, `cz`, `swap`, and
`unitary` gates. A circuit can be executed more than once, and a `Simulator`
can execute multiple circuits:

```python
circuit = Circuit(2)
circuit.x(0)
circuit.swap(0, 1)

simulator = Simulator()
result = simulator.execute(circuit)
assert result.probability(2) == 1.0
```

## Add a custom one-qubit gate

`unitary` accepts four complex values representing a 2x2 matrix:

```python
circuit = Circuit(1)
circuit.unitary(
    0,
    [0j, 1 + 0j,
     1 + 0j, 0j],
    duration_ns=12.0,
)

result = Simulator().execute(circuit)
print(result.probabilities())
```

An optional `error_matrix` can model a gate error. Invalid matrix lengths,
qubit indices, and durations raise `ValueError`.
