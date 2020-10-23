# aspen
Passerine's package manager.

## Getting Started
We plan to add a script to install the passerine toolchain
and binaries on your `$PATH`.
However, for now, we recommend you just install `aspen` by hand.

To try out Passerine:

1. Clone this repository and [Passerine's repository](https://github.com/vrtbl/passerine) into the same directory.
2. Start a new Passerine project using Aspen:
```bash
cargo run aspen -q -- new example
```
3. This will create a project named `example` in the current directory.
   Open this project and look through it in your editor of choice.
4. To run the project:
```bash
cargo run apsen -q -- run example
```

> If your cwd is a Passerine project,`example` can be omitted.
> Note that in the future, once prebuilt binaries are installed and distributed,
> the above process will essentially be `aspen new example` and `aspen run`.

### Commands

> NOTE: Not all commands are implemented ATM.
> Commands in **bold** are partially or wholly implemented.

| Command   | Result                                                    |
| --------- | --------------------------------------------------------- |
| `update`  | Updates the Passerine toolchain.                          |
| **`new`** | Creates a new Passerine package.                          |
| `publish` | Publishes package to the registries in `Aspen.toml`.      |
| `pull`    | Pulls fresh packages from the registries in `Aspen.toml`. |
| `add`     | Adds a dependency to `Aspen.toml`.                        |
| **`run`** | Builds and runs the corresponding Passerine package.      |
| `test`    | Builds and runs the package's tests.                      |
| `bench`   | Builds and runs the package's benchmarks.                 |
| `doc`     | Builds the package's documentation.                       |
| `debug`   | Builds and runs the package in interactive debug mode.    |

An optional path to the project root may be provided.
