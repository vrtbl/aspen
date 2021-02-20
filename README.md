# aspen
Passerine's package manager.

## Getting Started
To install the `aspen` command, run this in the shell of your choice:

```zsh
sh <(curl -sSf https://www.slightknack.dev/static/install.sh)
```

This requires git and a recent version of Cargo to work.
You can inspect the contents of `install.sh` first if you want,
we're not trying to play any tricks on you.

2. Start a new Passerine project using Aspen:
```bash
aspen new example
```
3. This will create a project named `example` in the current directory.
   Open this project and look through it in your editor of choice.
4. To run the project:
```bash
cd example
aspen run
```

> If your cwd is a Passerine project,`example` can be omitted.
> Note that in the future, once prebuilt binaries are installed and distributed,
> the above process will essentially be `aspen new example` and `aspen run`.

### Commands

> NOTE: Not all commands are implemented ATM.
> Commands in **bold** are partially or wholly implemented.

| Command    | Result                                                    |
| ---------- | --------------------------------------------------------- |
| `update`   | Updates the Passerine toolchain.                          |
| **`new`**  | Creates a new Passerine package.                          |
| `publish`  | Publishes package to the registries in `Aspen.toml`.      |
| `pull`     | Pulls fresh packages from the registries in `Aspen.toml`. |
| `add`      | Adds a dependency to `Aspen.toml`.                        |
| **`run`**  | Builds and runs the corresponding Passerine package.      |
| **`repl`** | Opens a fresh repl session.                               |
| `test`     | Builds and runs the package's tests.                      |
| `bench`    | Builds and runs the package's benchmarks.                 |
| `doc`      | Builds the package's documentation.                       |
| `debug`    | Builds and runs the package in interactive debug mode.    |

An optional path to the project root may be provided.
