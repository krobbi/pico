# Pico
_A PNG to ICO packer written in Python._  
__Copyright &copy; 2023 Chris Roberts__ (Krobbizoid).

# Contents
1. [Usage](#usage)
   * [Source](#source)
   * [Target](#target)
2. [License](#license)

# Usage
Pico aims to convert a source directory of PNG images to a target ICO file.

Copy `pico.py` to a directory with environment access. The script is completely
standalone and has no dependencies other than a modern version of Python.

Pico can then be run with the following command:
```shell
pico.py <source> <target>
```

## Source
The first argument, `<source>`, is the directory path to read the PNG files
from. The method for selecting which PNG files are used is not yet documented.

## Target
The second argument, `<target>`, is simply the file path to write the ICO file
to.

# License
Pico is released under the MIT License:  
https://krobbi.github.io/license/2023/mit.txt

See [license.txt](/license.txt) for a full copy of the license text.
