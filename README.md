# Pico
Pico converts PNG images to ICO icons by packing the PNG data directly into the
ICO file. This feature has been supported since Windows Vista and typically
results in smaller ICO files.

Build Pico with `cargo build --release` and move the executable from
`target/release/pico(.exe)` to a directory with environment access. After this,
you can use Pico from the command line:
```shell
pico [OPTIONS] <input>...
```

## Arguments
| Argument     | Usage                                      |
| :----------- | :----------------------------------------- |
| `<input>...` | One or more PNG input files or directories |

Pico must be given one or more unnamed arguments containing paths to PNG input
files. The order of the paths is 'stable' and will match the order of the data
in the ICO output file.

If a directory path is given, then it will be expanded into every file path in
the directory with a `.png` file extension (case-insensitive.) The expanded
file paths are inserted in-place where the directory path was, sorted into
alphabetical order. Subdirectories will not be searched.

## Options
| Short | Long         | Arguments | Usage                              |
| :---- | :----------- | :-------- | :--------------------------------- |
| `-o`  | `--output`   | `<path>`  | ICO output file                    |
| `-s`  | `--sort`     | _(None)_  | Sort ICO entries by resolution     |
| `-f`  | `--force`    | _(None)_  | Overwrite existing ICO output file |
| `-h`  | `--help`     | _(None)_  | Print help                         |
| `-V`  | `--version`  | _(None)_  | Print version                      |

If the `--output` option is not set, then the path to the ICO output file will
be the same as the first PNG input file path, but with a `.ico` file extension.

If the `--sort` flag is set, then Pico will sort the entries in the ICO output
file by resolution from largest to smallest.

Pico will fail if a file already exists at the ICO output file path, unless the
`--force` flag is set.

If the `--help` or `--version` flag is set, then Pico will print information
but not perform any action.

## Examples
Pack `icon.png` into `icon.ico`:
```shell
pico icon.png
```

Pack all PNG images in `icons/` into `icons.ico`:
```shell
pico icons/
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

Pack `icon_32x.png` and all PNG images in `icons/` into `icon.ico`, sorting
icon entries by descending resolution, and overwriting an existing file:
```shell
pico icon_32x.png icons/ -sf -o icon.ico
```

# Dependencies
Pico uses the following libraries:
* [clap](https://crates.io/crates/clap) - Command line argument parsing.
* [png](https://crates.io/crates/png) - PNG parsing and validation.

# License
Pico is released under the MIT License. See [LICENSE.txt](./LICENSE.txt) for a
full copy of the license text.
