# Pico
_A PNG to ICO packer written in Rust._  
__Copyright &copy; 2023-2024 Chris Roberts__ (Krobbizoid).

# Contents
1. [Usage](#usage)
   * [Arguments](#arguments)
   * [Options](#options)
   * [Examples](#examples)
2. [Dependencies](#Dependencies)
3. [License](#license)

# Usage
Pico aims to convert PNG images to ICO icons by packing the PNG data directly
into the output file. This feature has been supported since Windows Vista and
typically results in smaller ICO files.

Build Pico with `cargo build --release` and move the executable from
`target/release/pico(.exe)` to a directory with environment access. After this
you can use Pico from the command line:
```shell
pico [OPTIONS] <input>...
```

## Arguments
| Argument     | Usage                                      |
| :----------- | :----------------------------------------- |
| `<input>...` | One or more PNG input files or directories |

Pico must be given one or more unnamed arguments representing paths to PNG
input. The order of the arguments matches the order of the data in the ICO
output file.

If a directory path is given, it will be expanded to every file in that
directory with a `.png` file extension (case-insensitive). The order of the
files is system-dependent. Subdirectories will not be searched.

## Options
| Short | Long         | Arguments | Usage                              |
| :---- | :----------- | :-------- | :--------------------------------- |
| `-o`  | `--output`   | `<path>`  | ICO output file                    |
| `-f`  | `--force`    | _(None)_  | Overwrite existing ICO output file |
| `-h`  | `--help`     | _(None)_  | Print help                         |
| `-V`  | `--version`  | _(None)_  | Print version                      |

If the `--output` option is not set, the path to the ICO output file will be
the same as the first input path, but with a `.ico` file extension.

Pico will fail if a file already exists at the output path, unless the
`--force` flag is set.

If the `--help` or `--version` flags are set, Pico will print information but
not perform any action.

## Examples
Pack `icon.png` into `icon.ico`:
```shell
pico icon.png
```

Pack `input.png` into `output.ico`:
```shell
pico input.png -o output.ico
```

Pack `foo.png` and `bar.png` into `foo.ico`:
```shell
pico foo.png bar.png
```

Pack `icon_64x.png` and `icon_32x.png` into `icon.ico`, overwriting an existing
file:
```shell
pico icon_64x.png icon_32x.png -o icon.ico -f
```

# Dependencies
Pico uses the following libraries:
* [clap](https://crates.io/crates/clap) - Command line argument parsing.
* [png](https://crates.io/crates/png) - PNG parsing and validation.

# License
Pico is released under the MIT License:  
https://krobbi.github.io/license/2023/2024/mit.txt

See [LICENSE.txt](./LICENSE.txt) for a full copy of the license text.
