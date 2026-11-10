# Simulator configuration

`Simulator` optionally accepts a path to a TOML file. The file can configure
the execution engine and per-gate duration and rotation-angle error:

```toml
[engine]
parallel_threshold = 1

[gates.x]
time_ns = 25.0
rot_angle_err_rad = 0.2

[gates.cnot]
time_ns = 150.0
```

Load the settings when creating a simulator:

```python
from quentangled import Circuit, Simulator

circuit = Circuit(2)
circuit.h(0)
circuit.cnot(0, 1)

simulator = Simulator("quentangled.toml")
result = simulator.execute(circuit)
print("Estimated duration (ns):", simulator.estimated_duration_ns(circuit))
```

The simulator uses default settings when no path is supplied. Invalid values
raise `ValueError`; file access or parsing failures raise `RuntimeError`.
Duration estimates are reported in nanoseconds.
