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
| --- | --- | --- | --- | --- |
| First bit | Second bit | Third bit | Fourth bit | Last 4 bits |
| First bit | Second bit | Third bit | Fourth bit | Last 4 bits |
