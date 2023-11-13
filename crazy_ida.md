# I have this crazy idea

I could make my language compiled.
I could do this by having this type of array:

```
[
	rdi,
	rsi,
	r8-r15,
	mmx0-mmx7,
	xmm0-15,; for floating point numbers
	stack, ; for text and values that didn't make it into registers
]
```

rdx and rax registers will be used for mathematical operations
rcx is going to be a counter register used for if statements and loops

### This:

```
-1 -2 + println
```

### Would translate to:

```asm
section .text

; -1
mov rdi, -1; -1

; -2
mov rsi, -2; -2

; +
add rdi, rsi; rsi is implicitly abandoned and will be used for next push operation

; println
call println; println
```

### This:

```
1.0 2.0 / println
```

### Would translate to:

```asm
section .text

; -1
mov xmm0, 1; -1

; -2
mov rsi, -2; -2

; /
mov xmm0, 0x3ff0000000000000
mov xmm1, 0x4000000000000000
vdivsd xmm0, xmm0, xmm1

; println
call println; println
```

### This:

```
"Hell'o World!" println
```

### Would translate to:

```
section .text

mov rdi, STR123
call println

section .data
STR123: db "Hell'o World!"
```

# Stack only implementation

### This:

```
-8 8 + println
```

### Would translate to:

```
section .text

push -8
push 8
add QWORD [rsp-8], QWORD [rsp]
pop

call println
```
