#!/usr/bin/env python

"""
Pico
A PNG to ICO packer written in Python.
Copyright (c) 2023 Chris Roberts (Krobbizoid).

Released under the MIT License:
https://krobbi.github.io/license/2023/mit.txt
"""

import sys

from typing import Self

class PicoError(Exception):
    """ A fatal exception handled by Pico. """
    
    def __init__(self: Self, message: str) -> None:
        """ Initialize the Pico error's message. """
        
        super().__init__(message)


def pico(source_path: str, target_path: str) -> None:
    """ Run Pico from a source path and a target path. """
    
    print("Pico:")
    print(f" * source: '{source_path}'.")
    print(f" * target: '{target_path}'.")


def main(args: list[str]) -> int:
    """ Run Pico from arguments and return an exit code. """
    
    try:
        if len(args) != 3:
            raise PicoError("Usage: 'pico.py <source> <target>'.")
        
        pico(args[1], args[2])
        return 0
    except PicoError as e:
        print(e, file=sys.stderr)
        return 1


if __name__ == "__main__":
    sys.exit(main(sys.argv))
