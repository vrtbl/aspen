# aspen
Passerine's package manager.

## Usage
### Commands

> NOTE: All commands are planned ATM and not implemented yet.

| Command   | Result                                                    |
| --------- | --------------------------------------------------------- |
| `update`  | Updates the Passerine toolchain.                          |
| `new`     | Creates a new Passerine package.                          |
| `publish` | Publishes package to the registries in `Aspen.toml`.      |
| `pull`    | Pulls fresh packages from the registries in `Aspen.toml`. |
| `run`     | Builds and runs the corresponding Passerine package.      |
| `test`    | Builds and runs the package's tests.                      |
| `bench`   | Builds and runs the package's benchmarks.                 |
| `doc`     | Builds the package's documentation.                       |
| `debug`   | Builds and runs the package in interactive debug mode.    |

An optional path to the project root may be provided.
