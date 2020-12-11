all:
	export LD_LIBRARY_PATH=/mnt/c/Users/aetel/IdeaProjects/enclib/target/debug/test/
	swig -csharp example.i
	gcc -c -fpic example.c example_wrap.c
	gcc -shared example.o  example_wrap.o -o example.dll
	mono-csc -out:runme.exe *.cs

run:
	mono ./runme.exe
