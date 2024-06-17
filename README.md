# atelier


## Ubuntu with Torch and MPS-GPU

- Lower Vulverabilities
- Lower Compressed size
- Lower Software Footprint
- High Stability

- optional: 
    - Exploration: Common dev tools, infrastructure agnostic, unoptimized
    - Development: Targeted or Flexible Infrastructure, Performant Dev Tools
    - Production: Minimalistic dev tools, high infrastructure-driven optimization

## Target: Development

- linux: debian:bullseye-slim
- gpu: MPS | CUDA
- python: 3.11


### Rust

**components**

- `rustc` which is the Rust compiler nad Rustdoc
- `cargo` package manager and build tool.
- `rustfmt` automatically formatting code.
- `rust-std` Rust standard library (with optional further target specification)
- `rust-docs` A local copy of Rust documentation
- `rust-analyzer` language server for editors
- `clippy` Linting tool
- `rust-src` local copy of the source code of the RUst standard library.
- `llvm-tools` A collection of LLVM tools for multi-architecture support.

**Unselected components**

- `rustc-codegen-cranelift` experimental version of alternative compiler to `llvm`
- `miri` experimental version of rust interpreter.
- `rustc-dev` a library version of the compiler, not used for similicity.
- `rls` deprecated language server
- `rust-analysis` Metadata generator used by `rls`. 
- `rust-mingw` tools to build for windows-based platforms.

### Specific compilation process

instead of `cargo` we use `rustc` with specifications tailored to optimize the compilation
to work optimally in specific hosts architectures and operative systems.

```shell
rustc src/main.rs --target=<target-specification-here>
```

Check for more details in the official doc for [Command-line arguments]("https://doc.rust-lang.org/nightly/rustc/command-line-arguments.html")

### `rustc` compiler Tiered support

**host**: The host platform is considered both from architecture and operating system. 

Currently, the Rust programming language is supported differently across different hosts 
according to their architecture. There are 3 tiers. Tier 1 is the "guaranteed to work", 
[Tier 2: with Host Tools](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2-with-host-tools) "guaranteed to build", [Tier: without Host Tools](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-2-without-host-tools) [Tier 3](https://doc.rust-lang.org/nightly/rustc/platform-support.html#tier-3) "no guarantees"

for 64-bit OS, linux and macOS, these are the preffered ones:

goal is to select a target build in order to have Rust running natively in the machine. 

Here at the [rustup-components](https://rust-lang.github.io/rustup-components-history/) the full compatibility list can be verified, depending on the selected target build, different tools that are going to be availale. 

- macOS (Tier 2 with Host Tools): The `aarch64-apple-darwin` is used so to take advantage of the apple silicon. 
- linux with x86 CPU (Tier 1): The `x86_64-unknown-linux-gnu` is selected, this will be the most widely used option.
- linux with ARM CPU (Tier 1): The `aarch64-unknown-linux-gnu` is selected for ARM-based architectures like AWS: Graviton Lambdas.

### Cross-compilation

Now, [cross-compilation]("https://rust-lang.github.io/rustup/cross-compilation.html#cross-compilation") can utilized for the cases where the Target host is different than the Comipiling host. e.g. you are using your macOS machine to build a project meant to be running on a Ubuntu Server with ARM64 (AWS: Graviton lambdas)



## rustc online installation

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s --target
```

for offline installation strictly in macOS.

using the `aarch64-apple-darwin` Mac installers (.pkg) hosted as [standalone files](https://forge.rust-lang.org/infra/other-installation-methods.html#standalone-installers), download them and install with:

```
sudo installer -pkg aarch64-apple-darwin.pkg -target "/"
```

These installers come with rustc, cargo, rustdoc, the standard library, and the standard documentation, but do not provide access to additional cross-targets like rustup does

## Target: Compilation

universal set of rustup tools

