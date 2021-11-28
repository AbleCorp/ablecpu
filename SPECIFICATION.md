# DISCLAIMER
## This is a hobby project I used to better understand and practise VM's. Do not use it in any production environment.

# AbleCpu Specification
## Core Concepts

There are no registers, everything is mapped onto a 1 dimensional memory space also containing the bus for connecting to external devices.
Standard design is made for 64-bit but it should be scalable from 32 bit to anything above 64.

## Instructions

Instructions consist of 4 parts:

```
InstructionType (3 bits)   - stores what instruction it is
InstructionFlags (5 bits)  - stores additional information about how the instruction should be executed
InstructionDataA (64 bits) - stores data to be used by the instruction
InstructionDataB (64 bits) - stores data to be used by the instruction
```
### InstructionType

```
Binary - Name - Explanation
000    - NoOP - Does nothing
001    - Load - Stores value A in memory on address B
010    - Copy - Copys the value from memory address A to memory address B
011    - Add  - Adds the value from memory address A to value from memory address B and stores the Result in memory at address A
100    - Sub  - Subtracts the value from memory address B from value from memory address A and stores the Result in memory at address A
101    - Mul  - Multiplies the value from memory address A with value from memory address B and stores the Result in memory at address A
110    - Div  - Divides the value from memory address A by value from memory address B and stores the Result in memory at address A
111    - Comp - Compares the value from memory address A with value from memory address B and skips x instructions following theese rules:
```

### Jumping Rules

```
A = B: Skip 0 instructions
A < B: Skip 1 instruction
A > B: Skip 2 instructions
```