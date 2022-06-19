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

## Download:

You can use the binary file by downloading it from: <PATH_TO_RELEASE>

## OR

## Compile

You will need:

1. [cargo + rustc](https://www.rust-lang.org/tools/install)
2. Visual Studio C++ Build tools / GCC

First clone and enter the repository from the console:

```sh
git clone "https://github.com/xodix/stack_lang_compiler.git"

cd stack_lang_compile
```

## Running

### When using released binary:

```sh
./stack_lang_compiler test.sl
```

'test.sl' can be replaced with path to a different file.

### When using source code enter:

```sh
cargo run --release test.sl
```

'test.sl' can be replaced with path to a different file.

# Examples:

## Math and stdio

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
3 1 2 + *
```

How?

The stack contains [3, 2, 1]. When the interpreter encounters a '+' it takes **two top elements** and adds them together. Next it sees '\*' and multiplies [3, 3]. That leaves us with [9], that is 'print'-ed.

## If statements

```
2 3 {*} true if print
```

Prints 6 because the condition 'true' is truthy.

```
2.0 3.0 {/} 4 4 - if print
```

Prints 1.5 because '4 4 -' evaluates to 0 which is **not truthy**.

**Scopes** contain operations and values that can be executed.

**Truthy** values are ones that do not mean 0 in some way. For example "" (empty string) is **falsy** because its length is 0.

## Comparisons

```
2 3 < print
```

Prints false because 3 is smaller than 2.

This language supports '<', '>', '==', '<=', '>='.

## Logical Operators

### AND

And operator requires two arguments on the stack to be truthy.

```
2 3 && print
```

Prints true because 2 and 3 are truthy.

```
0 3 && print
```

Prints false because 0 is falsy.

## OR

Or operator requires one argument on the stack to be truthy.

```
2 3 || print
```

Prints true because 2 and 3 are truthy.

```
0 3 || print
```

Prints true because 3 is truthy.

## NOT (!)

Not operator (!) makes the top element on the stack not truthy.

```
3 ! print
```

Prints 0 because falsy value of Int is 0.

# Loops

## FOR

For loop takes the top element (a number) and executes the second element on the stack given amount of times.

```
2 {1 + print} 3 for
```

Executes exactly 3 times and prints:

```
3
4
5
```

## WHILE

While loop takes the top element and runs the scope while top element is truthy.

```
{-1 + print} 3 while
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
- 0 is not truthy so execution of while loop stops.

# Operating on the stack

## SWITCH

Switch takes the top element and switches it's value with the second.

```
2 1 switch - print
```

Prints 1 because 2 - 1 = 1. Instead of 1 - 2 we get 2 - 1.

## REVERSE

Reverse reverses the entire stack.

```
1 2 3 4 reverse + print
```

Prints 3 because 1 + 2 = 3. Instead of 3 + 4 we get 1 + 2.

## POP

Pop removes the top element on the stack.

```
1 2 3 pop + print
```

Prints 3 because 2 + 1 = 3. Instead of 3 + 2 we get 2 + 1.

## COPY

Copy copies the top element and put it on the stack.

```
1 2 3 copy + print
```

Prints 6 because 3 + 3 = 6. Instead of 3 + 2 we get 3 + 3.
