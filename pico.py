#!/usr/bin/env python

"""
Pico
A PNG to ICO packer written in Python.
Copyright (c) 2023 Chris Roberts (Krobbizoid).

Released under the MIT License:
https://krobbi.github.io/license/2023/mit.txt
"""

import sys

def pico(source_path: str, target_path: str) -> None:
    """ Run Pico from a source path and a target path. """
    
    print("Pico:")
    print(f" * source: '{source_path}'.")
    print(f" * target: '{target_path}'.")


def main(args: list[str]) -> int:
    """ Run Pico from arguments and return an exit code. """
    
    if len(args) == 3:
        pico(args[1], args[2])
        return 0
    else:
        print("Usage: 'pico.py <source> <target>'.", file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
