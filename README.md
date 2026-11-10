# Quentangled
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![CI](https://github.com/rbudhrani/quentangled-py/actions/workflows/ci.yml/badge.svg)](https://github.com/rbudhrani/quentangled-py/actions/workflows/ci.yml)
[![PyPI version](https://img.shields.io/pypi/v/quentangled)](https://pypi.org/project/quentangled/)
[![Python versions](https://img.shields.io/pypi/pyversions/quentangled)](https://pypi.org/project/quentangled/)
[![Documentation](https://img.shields.io/badge/docs-online-blue)](https://rbudhrani.github.io/quentangled-py/)

A high-performance, multithreaded state vector based quantum circuit simulator for Python, powered by a core Rust backend. Designed for speed, thread safety, and a clean, familiar developer experience.

## 🚀 Installation
This package is built with PyO3 and managed via `maturin`.

To build and install it locally for development:
```zsh
uv run maturin develop
```

## 💻 Usage
Here is how to build a simple Bell state generator and extract measurement probabilities:
```python
from quentangled import Circuit, Simulator

# Initialise a circuit with 2 qubits
circuit = Circuit(2)

# Apply a Hadamard gate to qubit 0 to create superposition
circuit.h(0)

# Apply a CNOT gate with control 0 and target 1 to entangle them
circuit.cnot(0, 1)

result = Simulator().execute(circuit)
print("State probabilities:", result.probabilities())
```

## Gate configuration
Gate durations and rotation-angle errors can be set in a TOML file:
```toml
[gates.x]
time_ns = 25.0
rot_angle_err_rad = 0.2

[gates.cnot]
time_ns = 150.0
```

Pass the file path when creating the simulator:
```python
simulator = Simulator("quentangled.toml")
result = simulator.execute(circuit)
print("Estimated duration (ns):", simulator.estimated_duration_ns(circuit))
print("State probabilities:", result.probabilities())
```

## 🛠️ Contributing
To contribute to this project, please open an issue and then create a pull request from it. To trigger the automated release job of the pipeline, the commits in the `main` branch must follow [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/). `release-please` parses commits using this specification and then decides whether the MAJOR, MINOR or PATCH version should be updated and updates the `CHANGELOG` using the commits as descriptions, so the commit message is very important. Before merging the PR ensure that the squashed commit message follows the conventional commit convention. `release-please` opens up a release pull request that is updated with the latest commits in `main`. Once this pull request is merged, a new tag is created along with a corresponding GitHub release.

The following commits will result in a version bump:
1. **fix:** a commit of the _type_ `fix` patches a bug in your codebase (this correlates with [`PATCH`](http://semver.org/#summary) in Semantic Versioning).
2. **feat:** a commit of the _type_ `feat` introduces a new feature to the codebase (this correlates with [`MINOR`](http://semver.org/#summary) in Semantic Versioning).
3. **BREAKING CHANGE:** a commit that has a footer `BREAKING CHANGE:`, or appends a `!` after the type/scope, introduces a breaking API change (correlating with [`MAJOR`](http://semver.org/#summary) in Semantic Versioning).
A BREAKING CHANGE can be part of commits of any _type_.
4. _types_ other than `fix:` and `feat:` are allowed, for example [@commitlint/config-conventional](https://github.com/conventional-changelog/commitlint/tree/master/%40commitlint/config-conventional) (based on the [Angular convention](https://github.com/angular/angular/blob/22b96b9/CONTRIBUTING.md#-commit-message-guidelines)) recommends `build:`, `chore:`,
  `ci:`, `docs:`, `style:`, `refactor:`, `perf:`, `test:`, and others..
5. _footers_ other than `BREAKING CHANGE: <description>` may be provided and follow a convention similar to
  [git trailer format](https://git-scm.com/docs/git-interpret-trailers).

Additional types are not mandated by the Conventional Commits specification, and have no implicit effect in Semantic Versioning (unless they include a BREAKING CHANGE).