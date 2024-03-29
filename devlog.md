# Notes
[8086 manual](https://ia801302.us.archive.org/2/items/bitsavers_intel80869lyUsersManualOct79_62967963/9800722-03_The_8086_Family_Users_Manual_Oct79.pdf)
Instruction encoding info starts at page 257 (section 4-18).

# Development log
## 2024-01-16
- Implemented `time` option for simulator that estimates execution cycles.
- Pending to have `time` option finished: implement CMP/ADD/SUB to memory.
- Implementd CMP/ADD/SUB to memory.
- Decided not to implement data transfer estimations.

## 2023-07-11
- Identified that the decoder is failing with `pub const MOV_REG_MEM_REG: u8 = 0b100010;`.
- Bug had to do with the decoding of CMP/ADD/SUB immediate to register memory, regarding the `data if s:w=01` condition.
- Fix bug with word flag not being set correctly when creating Operators.

