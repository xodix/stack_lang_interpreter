# Stack lang

This is an interpreted programming language based on a concept of a stack.

It takes heavy inspiration from postscript and forth.

Creating a completely new programing language may seem like a daunting task. However it is quite simple.

In case of interpreted languages (ones not compiled to machine code) it is simply a process of:

1. Parsing the source code into AST (Asynchronous Syntax Tree).
2. Executing the code based on AST created in step 1.

# Goals

My goal with creating this project is to expand my horizons on new type of programming paradigms. This project also serves as a reminder that programming languages are just programs that turn text into instructions on a cpu.

Creating your own programming language helps with rapidly learning others.

# How to run

You will need:

1. [cargo + rustc](https://www.rust-lang.org/tools/install)
2. Visual Studio C++ Build tools / GCC

First clone and enter the repository:

```sh
git clone "https://github.com/xodix/stack_lang_compiler.git"
cd stack_lang_compiler
```

Then enter:

```sh
cargo run --release test.sl
```

'test.sl' can be replaced with other files.

# Examples:

```
1 2 + print
```

Prints 3 to standard output.

```
1 2 + 3 * print_debug
```

Prints Int(9) to standard output because 1 + 2 = 3 and 3 \* 3 = 9.

The same operation can be written as:

```
3 2 1 + *
```

How?

The stack contains [3, 2, 1]. When the interpreter encounters a '+' it takes **two last elements** adds them together. Next it sees '\*' and multiplies [3, 3]. That leaves us with [9], that is 'print'-ed
