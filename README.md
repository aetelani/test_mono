# test_mono
Testing Mono and FFI.

Compiling shared library from Rust and Transpiling that to C#. Running the resulted code in Mono.

Check Makefile for commands. There is two targets, using library and compilation from C source. Swig and Mono was compliled from th HEAD outside the project.

There two different flavors of mono-ffi, other is compiled C and other is pre-compiled library. The exec are runmer for lib and runme for single lib, not two like in the runmer case. The LD_LIBRARY_PATH needs to be set when linking libraries together. The Mono wrapper uses direct association DLLImport to link library to source. See manual for more information.

The rust library includes cleanup stack for storing memory allocations for FFI strings. 
