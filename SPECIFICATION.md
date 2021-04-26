# U-RISC Specification
## Concepts
Light, easy to understand
## Registers
U-RISC has two general purpose registers; A and B. Additionally there is a X register, holding the position in the instruction blob. \
The S register is used for dynamic parameters in instructions, and is not intended as a buffer.
Depending on the settings the first 8-64 addresses of the cache are also implemented as registers.
## Bus Design
The bus is equipped with a total of 2^64 addresses. The addresses from 0 to 131071 are per core (also referred to as "Privat Bus").\
The addresses from 0 to 65,535 hold 64d values representing data that can be used by the processor. \
Addresses from 65536 to 131071 store 8 bit values representing instructions and their arguments. \
The address 131072 is used for maybe existing RAM, technically its seen as a device, but it wil always be at address 171072. \
Any address above may or may not be populated with devices.
## Miscellaneous
- Instructions are 8 bits long (5 bits currently used) and all data and addresses are 64 bits long
- Data is represented as "64d" in the instruction specification
- Addresses are represented as "64a"

## Instructions
Instruction - Name - Instruction signature
```
0x00 - No-OP - 0x00
0x01 - Load bus address into register a - 0x01+64a
0x02 - Load bus address into register b - 0x02+64a
0x03 - Add a to b and push result to a - 0x03
0x04 - Subtract a from b and push result to a - 0x04
0x05 - Multiply a and b and push result to a - 0x05
0x06 - Divide a by b and push result into a and rest to b - 0x06
0x07 - Copy a to b - 0x07
0x08 - Copy b to a - 0x08
0x09 - Swap a and b - 0x09
0x0A - Push a to bus address - 0x0A+64a
0x0B - Push b to bus address - 0x0B+64a
0x0C - Load value into a - 0x0C+64d
0x0D - Load value into b - 0x0D+64d
0x0E - Load bus adress into register x - 0x0E + 64a
0x0F - Copy a to x - 0x0F
0x10 - Copy b to x - 0x10
```