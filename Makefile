LD_LIBRARY_PATH=./
all:
	swig -csharp -dllimport example.dll example.i
	gcc -c -fpic example.c example_wrap.c
	ld -shared example.o  example_wrap.o -o example.dll
	mono-csc -out:runme.exe *.cs

run:
	mono --aot ./runme.exe
	mono ./runme.exe
