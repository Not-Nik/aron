# aron

An incomplete, x86_64 assembler

![aron example](imgs/aron.png?raw=true)

## Usage

```
Usage: aron [options] filename
 Options:
   --help       Prints this message
   -f format    Set the output binary format
      elf           ELF (64-bit)
      macho         Mach-O
   -o filename  Set output filename
```

## Performance

This is not empirical by any means, but the benchmarking on the `test.macos.s`, comparing with macOS' assembler `as`
shows:

![aron is 12.94 ± 2.34 times faster than as](imgs/aron-bm.png?raw=true)
