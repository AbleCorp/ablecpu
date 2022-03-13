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
The first bit is responsible for the OPCode, signing information and error checking policies.
The remaining 16 bits are split into two 8 bit numbers which we will call "arguments" from now on.
Depending on the OPCode, the arguments can function slightly differently.

As stated previously, the first byte is split into 3 parts like so:

| Halt on Error | Store debug info on error | First Argument Signed | Second Argument Signed | OPCode |
| --- | --- | --- | --- | --- |
| First bit | Second bit | Third bit | Fourth bit | Last 4 bits |
| If this bit is set to true it will stop execution upon any kind of error | If a error is detected store some info in a special location | Defines if the first argument should be treated as a signed number| Defines if the second argument should be treated a sa signed number| What to do lol|

### OpCodes

| OpCode | Description | Pseudo Code |
| --- | --- | --- |
| NoOP | Do nothing | `nop` |
| And | Bitwise AND | `a = a & b` |
| Or | Bitwise OR | `a = a | b` |
| Not | Bitwise NOT | `a = ~a` |
| Add | Addition | `a = a + b` |
| Sub | Subtraction | `a = a - b` |
| Mul | Multiplication | `a = a * b` |
| Div | Division | `a = a / b` |
| SL | Shift Left | `a = a << b` |
| SR | Shift Right | `a = a >> b` |
| RL | Rotate Left | `a = a << b | a >> (8 - b)` |
| RR | Rotate Right | `a = a >> b | a << (8 - b)` |
| CompEq | Compare Equal | `a = a == b` |
| CompGt | Compare Greater Than | `a = a > b` |
| CompLt | Compare Less Than | `a = a < b` |

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