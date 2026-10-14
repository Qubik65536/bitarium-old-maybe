# Development environment

![Type: Guide](assets/badges/guide.svg)

[Documentation](README.md) · [Setup task](tasks/T0001-development-environment.md) · [Progress](progress.md)

Status: setup guide and proposed initial layout. No tool installation, builds, implementation, or host compatibility checks have been completed by this template.

The accepted design is sufficient to install tools, create component scaffolds, and begin byte-addressed memory operations. Concrete RAM/load/entry addresses, RAM initialization, fault state updates, stopping behavior, and the native interface still need specification before the first integrated program.

Start on the current computer's native architecture. Linux/macOS on x86_64/ARM64 remain full-MVP requirements; successful builds on one computer do not establish all four targets.

## 1. Copy the documentation template

Extract `bitarium-project-template.zip`. Rename its top-level `bitarium-project-template` directory to `bitarium`, or copy all its contents into an existing project root, preserving the template's directory structure.

All commands below use Bash or Zsh. From a fresh project root, initialize Git if there is no existing repository:

```sh
cd /your/path/to/bitarium
git init
```

No remote or publication is required. Do not initialize a second repository when using an existing checkout.

## 2. Install host prerequisites

The C3 compiler and Rust need a host linker and C development environment. Use the block for the current system.

### macOS

Install Apple's command-line development tools if missing. Complete the installation before continuing:

```sh
xcode-select --install
```

With [Homebrew](https://brew.sh/) installed, install opam and the stable packaged C3 compiler:

```sh
brew install opam c3c
```

Homebrew may lag the newest upstream C3 release. Use the installed version initially and record it. If comparing work across hosts, select the same C3 release for both; the [official download guide](https://c3-lang.org/getting-started/prebuilt-binaries/) provides alternatives. Prefer a native toolchain on Apple Silicon rather than mixing it with Intel binaries.

### Debian or Ubuntu

Install the build tools, opam, archive tools, and package configuration support:

```sh
sudo apt update
sudo apt install build-essential git curl unzip pkg-config opam
```

### Fedora

Install the corresponding tools in a normal Fedora development environment:

```sh
sudo dnf install gcc gcc-c++ make git curl unzip tar pkgconf-pkg-config opam
```

On Fedora Atomic, run the Fedora commands inside a development container such as Distrobox. Keep builds and tool discovery in that environment consistently. Flatpak editor integration is a separate setup step; compilation should work in the development shell first.

### C3 on Linux

Follow the [official prebuilt-binary guide](https://c3-lang.org/getting-started/prebuilt-binaries/). Select a stable release for the current architecture (`uname -m` shows it), and retain the executable together with its bundled standard library.

For example, place the extracted compiler folder at `$HOME/.local/opt/c3c`, with `c3c` directly inside that folder. Add it to the current shell's search path:

```sh
export PATH="$HOME/.local/opt/c3c:$PATH"
c3c --version
```

Once that works, add the same PATH line to the relevant shell startup file. If the compiler cannot locate its standard library, follow the guide's `C3C_LIB` instructions using the actual extracted library directory.

## 3. Install Rust

Use the [official rustup installer](https://rust-lang.org/tools/install/) to install stable Rust, then load its environment and add the formatter and linter:

```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
. "$HOME/.cargo/env"
rustup toolchain install stable
rustup default stable
rustup component add rustfmt clippy
```

If rustup is already installed, skip the installer. Stable Rust is a setup recommendation, not a recorded project version. Record the exact compiler version after installation and pin it for reproducible builds when the first build works.

## 4. Create a local OCaml environment

From the Bitarium root, initialize opam if needed and create a project-local switch. The explicit OCaml 5.5.1 version below is a starting recommendation, not a requirement for the eventual proof tooling:

```sh
opam init --bare
opam switch create . 5.5.1
eval "$(opam env --switch=. --set-switch)"
opam install dune ocaml-lsp-server ocamlformat
```

The local compiler and packages live in `_opam/`. Skip `opam init` when opam is already initialized, and skip switch creation when this project already has its chosen switch. After opening a new shell in the project, run the `eval` line again, or use `opam exec -- COMMAND`.

## 5. Create the three component scaffolds

Run these once from the Bitarium root. They create standard tool-generated scaffolds for the owner to replace with hand-written implementation:

```sh
cargo new runner --name bitarium --bin --vcs none
c3c init memory --template static-lib
dune init proj bitarium_tools toolchain
mkdir -p programs
```

| Path | Role |
| --- | --- |
| `runner/` | Rust CPU, execution controls, and eventual terminal interface |
| `memory/` | C3 static library for memory and devices |
| `toolchain/` | OCaml assembler, disassembler, and reference model |
| `programs/` | Hand-written guest assembly and known-result examples |
| `docs/` | Project context, decisions, research, and guides |
| `_opam/` | Local OCaml compiler and dependencies; generated, untracked |

Do not run scaffold commands over existing source directories. A library scaffold compiling successfully does not establish the eventual C ABI or Rust linkage. Start with the independent builds, then specify the interface and integrate it.

## 6. Build each component independently

From the root, compile and run the generated Rust executable:

```sh
cargo run --manifest-path runner/Cargo.toml
```

Build the C3 static library from its directory:

```sh
cd memory
c3c build
cd ..
```

Build and run the generated OCaml executable:

```sh
cd toolchain
opam exec -- dune build
opam exec -- dune exec ./bin/main.exe
cd ..
```

Completion criterion: both generated executables run, and the C3 library builds without unresolved compiler/linker errors. This checks local toolchain setup, not emulator behavior.

Record version information in a dated note under `docs/experiments/`, including the host OS and architecture:

```sh
uname -m
rustc --version
cargo --version
c3c --version
opam --version
opam exec -- ocamlc -version
opam exec -- dune --version
cc --version
```

Add generated build directories to the root `.gitignore`: `runner/target/`, `memory/build/`, `toolchain/_build/`, and `_opam/`. Inspect scaffold output for any additional generated files. Keep `runner/Cargo.lock` tracked for the Rust application. Later, record the OCaml dependencies and compiler constraint in the Dune/opam metadata and lock the chosen dependency set.

## 7. Editor setup

In Zed, enable Rust support and install the OCaml extension. Ensure the editor uses the project's active opam environment; the [Zed OCaml guide](https://zed.dev/docs/languages/ocaml) recommends launching from a terminal:

```sh
eval "$(opam env --switch=. --set-switch)"
zed .
```

This command assumes the Zed CLI is installed and available in the same environment. A Flatpak editor or a compiler inside Distrobox may need additional environment bridging; verify `cargo`, `c3c`, and `ocamllsp` in the editor's terminal before relying on language-server diagnostics. C3 editor integration is optional initially; `c3c build` is the compilation check.

## 8. First implementation session

Start with one checked byte read and one checked byte write in the C3 memory module. Use offsets within a RAM buffer so a final guest base address is not needed for this increment.

Write the access contract before implementing it: valid offsets, ownership/lifetime, initial byte contents for the experiment, and the failure result for an invalid offset. Check the first and last valid bytes, an offset equal to capacity, and that a rejected write leaves memory unchanged. These are behavior checks, not proofs.

Next add little-endian halfword/word operations and alignment/range handling. Specify the native interface before Rust calls those functions. Then implement Rust reset/register behavior and one ADDI step.

Before the first integrated guest program, select concrete RAM/load/entry addresses and fault/stopping conventions in `first-machine.md`. A base, load address, and entry PC of zero remain the current proposal, not an accepted decision.

Proof tooling can be evaluated once there is a small function worth proving. Check Cameleer/Why3 or Kani compatibility at that point; use a separate opam switch for proof experiments if needed. Proof-tool installation is not a prerequisite for this environment setup.

## References

- [Rust installation](https://rust-lang.org/tools/install/)
- [C3 installation](https://c3-lang.org/getting-started/prebuilt-binaries/)
- [C3 build commands](https://c3-lang.org/build-your-project/build-commands/)
- [Homebrew C3 formula](https://formulae.brew.sh/formula/c3c)
- [opam installation](https://opam.ocaml.org/doc/Install.html)
- [opam environments and switches](https://opam.ocaml.org/doc/Usage.html)
- [OCaml 5.5.1 package](https://opam.ocaml.org/packages/ocaml-base-compiler/ocaml-base-compiler.5.5.1/)
- [Dune command-line guide](https://dune.readthedocs.io/en/stable/usage.html)
- [Zed OCaml setup](https://zed.dev/docs/languages/ocaml)
