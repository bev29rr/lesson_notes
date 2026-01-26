Sections 1 - 2
# 1 - Components of a computer
## 1 Core components of a processor
### ALU
- Performs basic arithmetic
- Incrementing/decrementing PC
- Logic and bit shifts
### CU
- Coordinates activities (directs flow of data)
## 2 CPU Registers
A register is a temporary storage location directly within the CPU. The registers are as follows:
- PC
- ACC
- MAR (**NOT MIR**)
- MDR
- CIR
## 3 Buses
A bus is a set of parallel lines connecting 2 or more components of a computer.

System bus connecting CPU to memory and other components (I/O):
- **Address bus**
- **Data bus**
- **Control bus**: control signals, **clk**
## 4 Assembly registers
**LMC** instructions **don't refer to registers** (they refer to memory addresses). However, instructions still use registers. E.g:
- OUT: ACC -> MDR (-> data bus)
- ADD/SUB -> MEM -> MDR, $\pm$  ACC -> ACC
## 5 FDE Cycle
**FDE** keeps going until a halt is reached. 
### Stages
#### Fetch
1. PC -> MAR -> **address bus**
2. Read signal on the **control bus**
3. MEM -> **data bus** -> MDR
4. MDR -> CIR
5. PC = PC + 1
#### Decode
6. Decodes instruction:
	- **Opcode** and **operand** are separated 
#todo Complete
## 14 (Parallel and Concurrency)
Concurrency means multiple things happening at the same time.

Ways to improve concurrent processing:
- Multiple cores (**parallel processing**)
- Distribute across multiple computers (**Distributed OS**)
- CPU scheduling

Better explained:

> “Parallel processing” ⊂ “Concurrency”

## 15 (IO Devices)
Obvious, but focus on sensors.
- OCR: Optical Character Recognition
- OMR: Optical Mark Recognition

## 16
Skipped

## 17 (RAM and ROM)
- **RAM**: main memory, primary storage

- **ROM**:
  - In general computers:
    - holds bootstrap loader
    - often refers to flash memory
  - Embedded systems:
    - may only have ROM

ROM can be considered a **security feature** since it is read-only.

## 18
**Virtual storage**: storage that is physically remote from the host PC (doesn’t have to be on the cloud).

# 2 - Systems software
## 19 (OS)
An **Operating System** provides:
- resource management
- security
- interrupt handling
- user interface

It provides an **abstraction** to the user.

**Utility software** aids system maintenance.

## 20 (Virtual Memory)
Two ways of achieving virtual memory:
- paging
- segmentation

**Memory management** is used for allocating and freeing memory.
- Page table maps virtual addresses to physical memory
  - **Paging**: dividing memory into fixed-size physical units
  - **Segmentation**: dividing memory into variable-sized logical units

**Fragmentation** occurs when memory has scattered “holes”: small unused segments of memory.
