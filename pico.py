#!/usr/bin/env python

"""
Pico
A PNG to ICO packer written in Python.
Copyright (c) 2023 Chris Roberts (Krobbizoid).

Released under the MIT License:
https://krobbi.github.io/license/2023/mit.txt
"""

import sys

def main(args: list[str]) -> int:
    """ Run Pico from arguments and return an exit code. """
    
    print("Hello, Pico!")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
