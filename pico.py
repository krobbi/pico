#!/usr/bin/env python

"""
Pico
A PNG to ICO packer written in Python.
Copyright (c) 2023 Chris Roberts (Krobbizoid).

Released under the MIT License:
https://krobbi.github.io/license/2023/mit.txt
"""

import os
import sys

from dataclasses import dataclass
from typing import Self

class PicoError(Exception):
    """ A fatal exception handled by Pico. """
    
    def __init__(self: Self, message: str) -> None:
        """ Initialize the Pico error's message. """
        
        super().__init__(message)


@dataclass
class Image:
    """ An image that can be packed into an ICO file. """
    
    name: str
    """ The name of the image's source file. """
    
    size: int
    """ The size of the image in pixels. """


class BinaryInput:
    """ Binary data that can be read as a stream. """
    
    _position: int = 0
    """ The current position in the binary input's data. """
    
    _data: bytes
    """ The binary input's data. """
    
    def __init__(self: Self, data: bytes) -> None:
        """ Initialize the binary input's data. """
        
        self._data = data
    
    
    def has(self: Self, signature: bytes) -> bool:
        """
        Return whether the binary input has a signature and advance
        beyond it if it does.
        """
        
        if self._peek(len(signature)) == signature:
            self._position += len(signature)
            return True
        else:
            return False
    
    
    def _peek(self: Self, length: int) -> bytes:
        """ Peek and return a length of data from the binary input. """
        
        peeked: bytes = self._data[self._position : self._position + length]
        
        if len(peeked) == length:
            return peeked
        else:
            raise PicoError("Could not read from input stream.")


def decode_image(name: str, size: int, data: BinaryInput) -> Image:
    """ Decode and return an image from its name, size, and data. """
    
    if not data.has(b"\x89PNG\r\n\x1a\n"):
        raise PicoError(f"File '{name}' is not a PNG image.")
    
    return Image(name, size)


def load_image(entry: os.DirEntry, size: int) -> Image:
    """ Load and return an image from a directory entry and a size. """
    
    try:
        with open(entry, "rb") as file:
            data: BinaryInput = BinaryInput(file.read())
    except OSError:
        raise PicoError(f"Could not read '{entry.name}'.")
    
    return decode_image(entry.name, size, data)


def scan_dir_images(path: str) -> list[Image]:
    """ Scan and return a list of images from a directory path. """
    
    try:
        with os.scandir(path) as dir:
            sizes: list[int] = [256, 128, 96, 64, 48, 32, 16]
            images: list[Image] = []
            
            for entry in dir:
                if entry.is_file() and entry.name.endswith(".png"):
                    for size in sizes:
                        if f"{size}x" in entry.name:
                            sizes.remove(size)
                            index: int = 0
                            
                            while index < len(images):
                                if size > images[index].size:
                                    break
                                
                                index += 1
                            
                            images.insert(index, load_image(entry, size))
                            
                            if sizes:
                                break
                            else:
                                return images
            
            return images
    except OSError:
        raise PicoError(f"Could not scan images from '{path}'.")


def pico(source_path: str, target_path: str) -> None:
    """ Run Pico from a source path and a target path. """
    
    images: list[Image] = scan_dir_images(source_path)
    
    if not images:
        raise PicoError(f"No valid images in '{source_path}'.")
    
    print(f"Image(s):")
    
    for image in images:
        print(f" * '{image.name}' ({image.size}x{image.size})")


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
