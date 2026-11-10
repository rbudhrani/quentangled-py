# Quickstart

## Install

Quentangled requires Python 3.14 or newer. To keep its dependencies separate
from other projects, create and activate a virtual environment:

```console
python -m venv .venv
```

Activate it on macOS or Linux:

```console
source .venv/bin/activate
```

On Windows Command Prompt:

```console
.venv\Scripts\activate.bat
```

On Windows PowerShell:

```powershell
.venv\Scripts\Activate.ps1
```

Then install Quentangled into the active environment:

```console
python -m pip install quentangled
```

If you manage your project with uv, add Quentangled as a project dependency
instead; uv creates and manages the project environment for you:

```console
uv add quentangled
```

## Create and run a circuit

This example prepares a Bell state, an entangled two-qubit state:

```python
from quentangled import Circuit, Simulator

circuit = Circuit(2)
circuit.h(0)
circuit.cnot(0, 1)

result = Simulator().execute(circuit)
print(result.probabilities())
```

The probabilities for basis states `00` and `11` are each `0.5`; the other
two basis states have probability zero. `probabilities()` and `amplitudes()`
are ordered by basis-state index. For two qubits, the entries correspond to
`00`, `01`, `10`, and `11`.

Qubits are indexed from zero. Gates that refer to an invalid qubit or otherwise
violate circuit constraints raise `ValueError`.

The Rust simulation runs without holding Python's global interpreter lock
(GIL), so other Python threads can continue while `execute()` is computing.
Execution uses a snapshot of the circuit taken when `execute()` is called.

## Inspect a result

```python
result.num_qubits
result.amplitudes()
result.probability(0)
```

`probability(index)` returns `None` if the basis-state index is out of range.
