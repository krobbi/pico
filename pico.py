#!/usr/bin/env python3

"""
Pico
A PNG to ICO packer written in Python.
Copyright (c) 2023 Chris Roberts (Krobbizoid).

Released under the MIT License:
https://krobbi.github.io/license/2023/mit.txt
"""

import os
import struct
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
    
    palette_size: int
    """ The number of colors in the image's palette, if applicable. """
    
    bits_per_pixel: int
    """ The number of bits per pixel in the image. """
    
    data: bytes
    """ The image's data. """


class BinaryInput:
    """ Binary data that can be read as a stream. """
    
    _position: int = 0
    """ The current position in the binary input's data. """
    
    _data: bytes
    """ The binary input's data. """
    
    def __init__(self: Self, data: bytes) -> None:
        """ Initialize the binary input's data. """
        
        self._data = data
    
    
    def get_data(self: Self) -> bytes:
        """ Return the binary input's data. """
        
        return self._data
    
    
    def get_position(self: Self) -> int:
        """ Return the binary input's position. """
        
        return self._position
    
    
    def seek(self: Self, position: int) -> None:
        """ Set the binary input's position. """
        
        self._position = position
    
    
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
    
    
    def get_u8(self: Self) -> int:
        """
        Get and return an 8-bit unsigned integer from the binary input.
        """
        
        return self._get_uint(1)
    
    
    def get_u32(self: Self) -> int:
        """
        Get and return a 32-bit unsigned integer from the binary input.
        """
        
        return self._get_uint(4)
    
    
    def _peek(self: Self, length: int) -> bytes:
        """ Peek and return a length of data from the binary input. """
        
        peeked: bytes = self._data[self._position : self._position + length]
        
        if len(peeked) == length:
            return peeked
        else:
            raise PicoError("Could not read from input stream.")
    
    
    def _get(self: Self, length: int) -> bytes:
        """ Get and return a length of data from the binary input. """
        
        peeked: bytes = self._peek(length)
        self._position += length
        return peeked
    
    
    def _get_uint(self: Self, length: int) -> int:
        """
        Get and return an unsigned integer from the binary input.
        """
        
        return int.from_bytes(self._get(length), "big", signed=False)


def decode_image(name: str, size: int, data: BinaryInput) -> Image:
    """ Decode and return an image from its name, size, and data. """
    
    if not data.has(b"\x89PNG\r\n\x1a\n"):
        raise PicoError(f"File '{name}' is not a PNG image.")
    
    palette_size: int = 0
    bits_per_pixel: int = 32
    
    while True:
        chunk_length: int = data.get_u32()
        next_chunk_position: int = data.get_position() + chunk_length + 8
        
        if data.has(b"IHDR"):
            width: int = data.get_u32()
            height: int = data.get_u32()
            
            if width < 1 or height < 1:
                raise PicoError(f"Image '{name}' has an invalid size.")
            elif width != height:
                raise PicoError(f"Image '{name}' is not square.")
            elif width != size:
                raise PicoError(f"Image '{name}' is not {size} pixels.")
            
            bit_depth: int = data.get_u8()
            color_type: int = data.get_u8()
            allowed_bit_depths: list[int]
            samples_per_pixel: int
            
            if color_type == 0:
                samples_per_pixel = 1 # Grayscale.
                allowed_bit_depths = [1, 2, 4, 8, 16]
            elif color_type == 2:
                samples_per_pixel = 3 # True color.
                allowed_bit_depths = [8, 16]
            elif color_type == 3:
                samples_per_pixel = 1 # Indexed.
                allowed_bit_depths = [1, 2, 4, 8]
            elif color_type == 4:
                samples_per_pixel = 2 # Grayscale with alpha.
                allowed_bit_depths = [8, 16]
            elif color_type == 6:
                samples_per_pixel = 4 # True color with alpha.
                allowed_bit_depths = [8, 16]
            else:
                raise PicoError(f"Image '{name}' has an invalid color type.")
            
            if bit_depth not in allowed_bit_depths:
                raise PicoError(f"Image '{name}' has an invalid bit depth.")
            
            bits_per_pixel = bit_depth * samples_per_pixel
        elif data.has(b"PLTE"):
            palette_size = chunk_length // 3
        elif data.has(b"IEND"):
            break
        
        data.seek(next_chunk_position)
    
    return Image(name, size, palette_size, bits_per_pixel, data.get_data())


def load_image(entry: os.DirEntry, size: int) -> Image:
    """ Load and return an image from a directory entry and a size. """
    
    try:
        with open(entry, "rb") as file:
            data: BinaryInput = BinaryInput(file.read())
    except OSError:
        raise PicoError(f"Could not read from '{entry.name}'.")
    
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


def encode_icon(images: list[Image]) -> bytes:
    """ Encode and return ICO data from a list of images. """
    
    def put_u8(value: int) -> None:
        """ Put an 8-bit unsigned integer to the index data. """
        
        nonlocal index_data
        index_data += bytearray(struct.pack("<B", value & 0xff))
    
    
    def put_u16(value: int) -> None:
        """ Put a 16-bit unsigned integer to the index data. """
        
        nonlocal index_data
        index_data += bytearray(struct.pack("<H", value & 0xffff))
    
    
    def put_u32(value: int) -> None:
        """ Put a 32-bit unsigned integer to the index data. """
        
        nonlocal index_data
        index_data += bytearray(struct.pack("<I", value & 0xffff_ffff))
    
    
    index_data: bytearray = bytearray()
    put_u16(0) # Reserved. Must always be 0.
    put_u16(1) # Image type. 1 for icon, 2 for cursor.
    put_u16(len(images)) # Number of images in the file.
    
    image_base: int = len(index_data) + len(images) * 16
    image_data: bytearray = bytearray()
    
    for image in images:
        put_u8(image.size) # Image width in pixels.
        put_u8(image.size) # Image height in pixels.
        put_u8(image.palette_size) # Number of colors.
        put_u8(0) # Reserved. Should be 0.
        put_u16(0) # Color planes. Should be 0 or 1.
        put_u16(image.bits_per_pixel) # Bits per pixel.
        put_u32(len(image.data)) # Image data size in bytes.
        put_u32(image_base + len(image_data)) # Image offset in bytes.
        image_data += bytearray(image.data) # Image data blob.
    
    return bytes(index_data + image_data)


def pico(source_path: str, target_path: str) -> None:
    """ Run Pico from a source path and a target path. """
    
    images: list[Image] = scan_dir_images(source_path)
    
    if not images:
        raise PicoError(f"No valid images in '{source_path}'.")
    
    try:
        with open(target_path, "wb") as file:
            file.write(encode_icon(images))
    except OSError:
        raise PicoError(f"Could not write to '{target_path}'.")
    
    print(f"Packed {len(images)} image(s) to '{target_path}':")
    
    for index, image in enumerate(images):
        print(f" * {index + 1}: '{image.name}'")


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
