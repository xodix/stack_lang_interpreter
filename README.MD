# Stack lang

Stack lang is an interpreted programming language based on the concept of a stack.

It takes heavy inspiration from postscript and forth.

Creating a new programing language may seem like a daunting task. However, it is quite simple.

In the case of interpreted languages (ones not compiled to machine code), it is simply a process of:

1. Parsing the source code into AST (Asynchronous Syntax Tree).
2. Executing the code from it.

# Goals

My goal in creating this project is to expand my horizons on new types of programming paradigms. This project also serves as a reminder that programming languages are just programs that turn text into instructions on a CPU.

Creating your programming language helps with rapidly learning others.

# How to run

## Download:

You can use the binary file by downloading it from [here](https://github.com/xodix/stack_lang_compiler/releases/latest)

## OR

## Compile

You will need:

1. [cargo + rustc](https://www.rust-lang.org/tools/install)
2. Visual Studio C++ Build tools / GCC

Firstly clone and enter the repository from the console:

```sh
git clone "https://github.com/xodix/stack_lang_compiler.git"

cd stack_lang_compile
```

# Running

## When using released binary:

```sh
./stack_lang_compiler.exe examples/fizzbuzz.sl
```

'examples/fizzbuzz.sl' can be replaced with a path to a different file.

## When using source code enter:

```sh
cargo run --release examples/fizzbuzz.sl
```

'examples/fizzbuzz.sl' can be replaced with a path to a different file.

# Syntax with examples:

## Math and stdio

```
1 2 + print
```

Prints `3` to standard output.

```
1 2 + 3 * print_debug
```

Prints `Int(9)` to standard output because `1 + 2 = 3` and `3 \* 3 = 9`.

The same operation can be written as:

```
3 1 2 + *
```

How?

The stack contains [3, 2, 1]. When the interpreter encounters a '+' it takes **two top elements** and adds them together. Next, it sees '\*' and multiplies [3, 3]. That leaves us with [9], which is 'print'-ed.

## Comments

```
// This is invisible and is not interpreted.
```

Prints nothing because it is a comment.

```
1 2 /*This is invisible and is not interpreted.*/ + print
```

Prints `3`.

## If statements

```
2 3 {*} true if print
```

Prints `6` because the condition 'true' is truthy.

```
2.0 3.0 {/} 4 4 - if print
```

Prints `1.5` because `4 4 -` evaluates to `0` which is **not truthy**.

**Scopes** contain operations and values that can be executed.

**Truthy** values do not mean 0 in some way. For example "" (empty string) is **falsy** because its length is 0.

## Comparisons

```
2 3 < print
```

Prints false because `3` is smaller than `2`.

This language supports '<', '>', '==', '<=', '>='.

## Logical Operators

### AND

And operator requires two arguments on the stack to be truthy.

```
2 3 && print
```

Prints `true` because `2` and `3` are truthy.

```
0 3 && print
```

Prints `false` because `0` is falsy.

### OR

Or operator requires one argument on the stack to be truthy.

```
2 3 || print
```

Prints `true` because `2` and `3` are truthy.

```
0 3 || print
```

Prints `true` because `3` is truthy.

### NOT (!)

Not operator (!) makes the top element on the stack falsy if it is truthy and truthy if it is falsy.

```
3 ! print
```

Prints `0` because the falsy value of Int is `0`.

```
0 ! print
```

Prints `1` because the truthy value of Int is `1`.

## Loops

### FOR

For loop takes the top element (a number) and executes the second element on the stack a given amount of times.

```
2 {1 + println} 3 for
```

Executes 3 times and prints:

```
3
4
5
```

### WHILE

While loop takes the top element and runs the scope while the top element is truthy.

```
{-1 + println} 3 while
```

Prints:

```
2
1
0
```

- 3 is truthy so -1 is added to the top element.
- 2 is truthy so -1 is added to the top element.
- 1 is truthy so -1 is added to the top element.
- 0 is not truthy so the execution of the while loop stops.

## Operating on the stack

### SWITCH

Switch takes the top element and switches its value with the second.

```
2 1 switch - print
```

Prints `1` because `2 - 1 = 1`. Instead of `1 - 2` we get `2 - 1`.

### REVERSE

Reverse reverses the entire stack.

```
1 2 3 4 reverse + print
```

Prints `3` because `1 + 2 = 3`. Instead of `3 + 4` we get `1 + 2`.

### POP

Pop removes the top element on the stack.

```
1 2 3 pop + print
```

Prints `3` because `2 + 1 = 3`. Instead of `3 + 2` we get `2 + 1`.

### COPY

Copy copies the top element and put it on the stack.

```
1 2 3 copy + print
```

Prints `6` because `3 + 3 = 6`. Instead of `3 + 2` we get `3 + 3`.

## Macros (functions)

Macros make a scope accessible by its name.

```
5
{2 *} "double" macro
double print
```

Prints `10` because `5 * 2 = 10`.

The stack contains: number `5`, `scope` and `"double"`.

Then macros name and its scope is popped from the stack and saved in memory.

When the double macro is executed the top element on the stack is multiplied by two and printed.

## Constants

Constants give defined values names.

```
5 "LUCKY" const
LUCKY 2 * print
```

Prints `10` because `LUCKY` is now an alias for `5`.
