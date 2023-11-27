# Pico
_A PNG to ICO packer written in Rust._  
__Copyright &copy; 2023 Chris Roberts__ (Krobbizoid).

# Contents
1. [Usage](#usage)
2. [License](#license)

# Usage
Pico aims to convert source PNG images to target ICO files by packing the PNG
data directly into the output file. This feature has been supported since
Windows Vista and typically results in smaller ICO files.

Build Pico with `cargo build --release` and move the target executable to a
directory with environment access. After this you can use `pico` in the command
line.

Pico is in the process of being ported from Python to Rust so that it can be
used as a native executable. I have minimal Rust experience and the Rust
edition is not yet suitable for real-world use.

# License
Pico is released under the MIT License:  
https://krobbi.github.io/license/2023/mit.txt

See [license.txt](/license.txt) for a full copy of the license text.
