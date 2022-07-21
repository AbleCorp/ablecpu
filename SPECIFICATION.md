# DISCLAIMER

## THIS CODEBASE SHOULD IN NO CASE BE USED IN ANY KIND OF PRODUCTION ENVIRONMENT. IT IS UNSTABLE AND NO SUPPORT IS PROVIDED.

# DISCLAIMER END

## Introduction

AbleCPU aims to provide a simple and reduced ISA, while also maintaining ease of use and flexibility.
One of the core concepts is, that *everything* is memory mapped.
So there are no registers, no device bus and no special instructions to deal with them.
NOTE: This version specifies a 8-bit CPU, but it is possible to extend it to 16-bit and 32-bit.

### Instructions

Instructions consist of 24 bits.
The first byte is responsible for the OPCode, signing information and error checking policies.
The remaining 16 bits are split into two 8 bit numbers which we will call "arguments" from now on.
Depending on the OPCode, the arguments can function slightly differently.

As stated previously, the first byte is split into 3 parts like so:

| Halt on Error | Store debug info on error | First Argument Signed | Second Argument Signed | OPCode |
| --- | --- | --- | --- | --- |
| First bit | Second bit | Third bit | Fourth bit | Last 4 bits |
| If this bit is set to true it will stop execution upon any kind of error | If a error is detected store some info in a special location | Defines if the first argument should be treated as a signed number| Defines if the second argument should be treated a sa signed number| What to do lol|

The instruction will always load the arguments from memory and store the result in memory.

### OpCodes

OpCodes are designed to be quite simple. They are just a number which defines what to do.

| Binary | OpCode | Description | Pseudo Code |
| --- | --- | --- | --- |
| 0000 | NoOP | Do nothing | `nop` |
| 0001 | And | Bitwise AND | `a = a & b` |
| 0010 | Or | Bitwise OR | `a = a \| b` |
| 0011 | Not | Bitwise NOT | `a = ~a` |
| 0100 | Add | Addition | `a = a + b` |
| 0101 | Sub | Subtraction | `a = a - b` |
| 0110 | Mul | Multiplication | `a = a * b` |
| 0111 | Div | Division | `a = a / b` |
| 1000 | SL | Shift Left | `a = a << 1` |
| 1001 | SR | Shift Right | `a = a >> 1` |
| 1010 | RL | Rotate Left | `a = a <<< 1` |
| 1011 | RR | Rotate Right | `a = a >>> 1` |
| 1100 | Copy | Copy value | `b = a` |
| 1101 | CompEq | Compare Equal | `a == b` |
| 1110 | CompGt | Compare Greater Than | `a > b` |
| 1111 | CompLt | Compare Less Than | `a < b` |

### Comparison

If a comparison is false, the next instruction is skipped.

### Memory Map

| Address | Description |
| --- | --- |
| 0 | Program Counter |
| 1-127 | Instruction Memory |
| 128-191 | Data Memory |
| 192-255 | Devices |

### Device Ideas

| Address | Name | Description |
| --- | --- | --- |
| 192 | Goto Page (Instructions) | Set the page to X (If you want paging)
| 193 | Goto Page (Data) | Set the page to X (If you want paging) |
| 194 | ASCII Output | Output a character to the terminal |
| 195 | ASCII Input | Read a character from the terminal |