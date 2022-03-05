# DISCLAIMER
## This is a hobby project I used to better understand and practise VM's. Do not use it in any production environment.

# AbleCpu Specification
## Core Concepts

There are no registers, everything is mapped onto a 1 dimensional memory space also containing the bus for connecting to external devices.
Standard design is made for 16-bit but it should be scalable from 32 bit to anything above 64.

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
011    - Comp - Compares the value from memory address A with value from memory address B and skips x instructions following the jumping rules
100    - Add  - Adds the value from memory address A to value from memory address B and stores the Result in memory at address A
101    - Sub  - Subtracts the value from memory address B from value from memory address A and stores the Result in memory at address A
110    - Mul  - Multiplies the value from memory address A with value from memory address B and stores the Result in memory at address A
111    - Div  - Divides the value from memory address A by value from memory address B and stores the Result in memory at address A

```

### Jumping Rules

```
A = B: Skip 0 instructions
A < B: Skip 1 instruction
A > B: Skip 2 instructions
```

### Instruction Flags

```
1-1-1 - 11
| | |   Instruction execution speed (00 highest, 11 lowest)
| | don't store debug info
| If 1: don't halt if error
If 1: don't do error handling
```

### Bus Sturcture

16-bit bus:
```
Address Space  - Name              - Explanation
0              - rego_zero         - Used to store the current instruction position
1 - 16383      - Data Cache        - Used to store data
16384 - 32767  - Instruction Cache - Used to store instructions
32768          - RAM               - Place for the RAM controller
32769+         - Devices           - Place for additional devices
```

32-bit bus (64-bit):
```
Address Space  - Name              - Explanation
0              - rego_zero         - Used to store the current instruction position
1 - 65535      - Data Cache        - Used to store data
65536 - 131071 - Instruction Cache - Used to store instructions
131072         - RAM               - Place for the RAM controller
131073+        - Devices           - Place for additional devices
```