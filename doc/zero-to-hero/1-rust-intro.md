
## Installation

### Windows
The class went to rustup and installed. 

I used Winget
```
winget install -e --id Rustlang.Rustup
```

```
PS C:\Users\Steve> rustc --version
rustc 1.100.0 (0ef1bc386 2026-11-09)
PS C:\Users\Steve> cargo --version
cargo 1.100.0 (3acc79c0c 2026-11-05)
```


### Linux

I'm using Debian on WSL2.  
```
sudo apt update && sudo apt install -y build-essential
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"
rustc --version && cargo --version
```

WSL commits failed with 

```
remote: error: GH007: Your push would publish a private email address.
```
Find your noreply email, replace your git email, 
```
git config user.email 1711544+steveaf@users.noreply.github.com
```


### Extensions

rust-analyzer

## Fundamentals

Rust refuses the old trade-off. The historical choice was speed (C, C++) or memory safety (Java, Python, JavaScript, Go). Rust argues that the trade-off is avoidable: performance close to C++ with classes of memory bugs ruled out at compile time rather than at runtime by a garbage collector. Reflect on what it means that "fast" and "safe" are not antagonistic in the same project.

Production adoption is load-bearing, not promotional. Rust ships in the Linux kernel, Android system components, AWS Firecracker, parts of the Windows kernel, and Cloudflare's edge. These are systems where a memory bug is a CVE or an outage — which is exactly the proof that compile-time safety holds up under real load.

rustup installs a toolchain, not a compiler. One command wires up rustc (the compiler), cargo (the build tool and package manager), and rustup itself (the toolchain manager that handles updates, version pinning, and switching between stable/beta/nightly channels). This unification is a deliberate contrast to the fragmented setup in older systems languages.

The editor must agree with the shell. rust-analyzer is the official language server of the Rust Project. When you install the rust-analyzer VS Code extension (publisher: rust-lang), your inline diagnostics and type hints come from the same compiler front-end that cargo check uses — so what you see while typing matches what you see when you build.


### Types

Scalar types (i32, u64, f64, bool, char): Rust's primitive types.

Integers — signed (i8, i16, i32, i64, i128, isize) and unsigned (u8, …, usize). i32 is the default.

Floats — f32 and f64. f64 is the default.

Booleans — bool, with values true and false.

Characters — char, a single Unicode scalar value (four bytes). 'A', 'é', and '🦀' are all valid char literals. This is fundamentally different from C's byte-wide char and from Python's "any string of length 1."


### Binding

let binding and immutability: The let keyword introduces a variable binding. By default, bindings are immutable — reassigning a let variable is a compile error: cannot assign twice to an immutable variable. To opt into mutation, write let mut. This is the single most surprising default for developers coming from Python, JavaScript, Java, or Go, and it is deliberate: it makes "which values can change" visible at a glance.



