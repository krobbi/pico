# Pico
_A PNG to ICO packer written in Rust._  
__Copyright &copy; 2023 Chris Roberts__ (Krobbizoid).

# Contents
1. [Usage](#usage)
   * [Arguments](#arguments)
   * [Options](#options)
      * [Optimize Flag](#optimize-flag)
   * [Examples](#examples)
2. [Libraries](#libraries)
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
| Argument     | Usage                       |
| :----------- | :-------------------------- |
| `<input>...` | One or more PNG input files |

Pico must be given one or more unnamed arguments representing paths to PNG
input files. The order of the arguments matches the order of the data in the
ICO output file.

## Options
| Short | Long         | Arguments | Usage                              |
| :---- | :----------- | :-------- | :--------------------------------- |
| `-o`  | `--output`   | `<path>`  | ICO output file                    |
| `-f`  | `--force`    | _(None)_  | Overwrite existing ICO output file |
| `-z`  | `--optimize` | `...`     | PNG optimization level             |
| `-h`  | `--help`     | _(None)_  | Print help                         |
| `-V`  | `--version`  | _(None)_  | Print version                      |

If the `--output` option is not set, the path to the ICO output file will be
the same as the first input path, but with a `.ico` file extension.

Pico will fail if a file already exists at the output path, unless the
`--force` flag is set.

If the `--help` or `--version` flags are set, Pico will print information but
not perform any action.

### Optimize Flag
If the `--optimize` flag is set, all PNG input will be optimized to take up
less space in the output file. The input files will not be modified.

The more times the flag appears, the higher the optimization level will be:
| Count | Example  | Level  | Equivalent oxipng options                       |
| :---- | :------- | :----- | :---------------------------------------------- |
| 0     | _(None)_ | None   | _(None)_                                        |
| 1     | `-z`     | Low    | `--opt max --strip all --alpha`                 |
| 2     | `-zz`    | Medium | `--opt max --strip all --alpha --fast --zopfli` |
| 3+    | `-zzz`   | High   | `--opt max --strip all --alpha --zopfli`        |

The highest optimization level may be unreasonably slow for larger images.
Higher optimization levels do not guarantee a smaller or equal output size to
lower optimization levels.

## Examples
Pack `icon.png` into `icon.ico`:
```shell
pico icon.png
```

Pack `input.png` into `output.ico`:
```shell
pico input.png -o output.ico
```

Pack `big.png` into `micro.ico` with compression:
```shell
pico big.png -z -o micro.ico
```

Pack `big.png` into `nano.ico` with increased compression:
```shell
pico big.png -zz -o nano.ico
```

Pack `big.png` into `pico.ico` with maximum compression:
```shell
pico big.png -zzz -o pico.ico
```

Pack `foo.png` and `bar.png` into `foo.ico`:
```shell
pico foo.png bar.png
```

Pack `icon_64x.png` and `icon_32x.png` into `icon.ico` with maximum
compression, overwriting an existing file:
```shell
pico icon_64x.png icon_32x.png -zzz -o icon.ico -f
```

# Libraries
Pico is made possible with the following fantastic libraries:
* [clap](https://crates.io/crates/clap) - Command line argument parsing.
* [oxipng](https://crates.io/crates/oxipng) - PNG optimization.
* [png](https://crates.io/crates/png) - PNG parsing and validation.

# License
Pico is released under the MIT License:  
https://krobbi.github.io/license/2023/mit.txt

See [LICENSE.txt](./LICENSE.txt) for a full copy of the license text.
