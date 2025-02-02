# meta-build

## Introduction

`meta-build` is my attempt at writing a simple meta-build system that uses the Ninja build system.

## Features

- [x] Recursive directory traversal for finding source (`.c`) and header (`.h`) files.
- [x] Uses most popular and recommended compiler-flags.
- [x] Uses most popular and recommended linker-flags.

## Examples

Running `meta-build --executable` using Lua's source code produces the following `build.ninja` file:

```text
cflags = -O2 -Wall -Wextra -Wformat -Wformat=2 -Wconversion -Wsign-conversion -Wtrampolines -Wimplicit-fallthrough -Wbidi-chars=any -Werror=format-security -Werror=implicit -Werror=incompatible-pointer-types -Werror=int-conversion -fstrict-flex-arrays=3 -fstack-clash-protection -fstack-protector-strong -fcf-protection=full -fno-delete-null-pointer-checks -fno-strict-overflow -fno-strict-aliasing -ftrivial-auto-var-init=zero -fexceptions -fPIE -pie -I/home/omega/Git/meta-build
lflags = -Wl,-z,nodlopen -Wl,-z,noexecstack -Wl,-z,relro -Wl,-z,now -Wl,--as-needed -Wl,--no-copy-dt-needed-entries
rule cc
  depfile = $out.d
  command = gcc -MD -MF $out.d $cflags -c $in -o $out
rule ld
  command = gcc $in -o $out $lflags -lm
build ldo.o: cc ldo.c
build lmathlib.o: cc lmathlib.c
build lgc.o: cc lgc.c
build lcode.o: cc lcode.c
build ltable.o: cc ltable.c
build lapi.o: cc lapi.c
build lzio.o: cc lzio.c
build lctype.o: cc lctype.c
build lobject.o: cc lobject.c
build lfunc.o: cc lfunc.c
build lmem.o: cc lmem.c
build ltm.o: cc ltm.c
build lvm.o: cc lvm.c
build lutf8lib.o: cc lutf8lib.c
build lbaselib.o: cc lbaselib.c
build lstrlib.o: cc lstrlib.c
build ldebug.o: cc ldebug.c
build ldblib.o: cc ldblib.c
build lopcodes.o: cc lopcodes.c
build loadlib.o: cc loadlib.c
build llex.o: cc llex.c
build lua.o: cc lua.c
build lparser.o: cc lparser.c
build lstring.o: cc lstring.c
build ldump.o: cc ldump.c
build liolib.o: cc liolib.c
build loslib.o: cc loslib.c
build lstate.o: cc lstate.c
build lcorolib.o: cc lcorolib.c
build lundump.o: cc lundump.c
build lauxlib.o: cc lauxlib.c
build linit.o: cc linit.c
build ltablib.o: cc ltablib.c
build main: ld ldo.o lmathlib.o lgc.o lcode.o ltable.o lapi.o lzio.o lctype.o lobject.o lfunc.o lmem.o ltm.o lvm.o lutf8lib.o lbaselib.o lstrlib.o ldebug.o ldblib.o lopcodes.o loadlib.o llex.o lua.o lparser.o lstring.o ldump.o liolib.o loslib.o lstate.o lcorolib.o lundump.o lauxlib.o linit.o ltablib.o
```

