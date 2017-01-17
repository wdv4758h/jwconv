#!/usr/bin/env python

from cffi import FFI

ffi = FFI()
lib = ffi.dlopen("../target/release/libjwconv.so")
ffi.cdef('''
char* const romaji_to_hiragana(char* const data);
void string_free(char* ptr);
''')


def romaji_to_hiragana(data):
    ptr = lib.romaji_to_hiragana(data.encode())
    ptr = ffi.gc(ptr, lib.string_free)
    return ffi.string(ptr).decode('utf-8')
