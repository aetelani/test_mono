LD_LIBRARY_PATH=./
all:
	swig -csharp -dllimport example.dll example.i
	gcc -c -fpic example.c example_wrap.c
	ld -shared --whole-archive example.o  example_wrap.o -o example.dll
	ldd -r -d example.dll
	mono-csc -out:runme.exe example.cs  examplePINVOKE.cs  runtime.cs

enclib:
	swig -csharp -dllimport enclib.dll enclib.i
	gcc -c -fPIC enclib_wrap.c
	ld -shared -lc -L../../target/debug/libenclib.so  enclib_wrap.o -o enclib.dll
	ldd -r -d enclib.dll
	mono-csc -out:runmer.exe enclib.cs  enclibPINVOKE.cs  runtimer.cs
run:
	mono --aot ./runme.exe
	mono ./runme.exe
