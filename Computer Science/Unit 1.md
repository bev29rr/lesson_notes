Sections 1 - 2
# 1
// TODO: catch up

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
