Sections 3 - 4
# 36
- Waterfall
- Spiral
- Agile
	- RAD
	- XP
### Waterfall
- Gathers all requirements upfront
- Design everything carefully
- Focus on delivering initial requirements
- Then everything built
### Spiral
- Gathers some requirements upfront
- Iterative design
- Focus on managing/reducing **risk**
- Client feedback on each iteration
### Agile
- Similar to spiral but with a focus on releasing often
- Continuous delivery
- **Expects** requirements to change
#### XP
- Focus on high quality code released frequently
- Regular automated tests
- **Pair programming** and **code reviews**
#### RAD
- Focus on prototypes
- Regular feedback from client
- Less focus on **planning** and more on **faster prototypes** (for feedback)
# 37 & 38
Careful with scopes:
```vbscript
global string = "foo"
```

Iterative loops:
```vbscript
Do 
	success = function()
Until success == false
```

By value and by reference:
```vbscript
procedure foobar(x: byVal, y: byRef) 
	...
endprocedure
```
Using values by reference modifies the data in the reference, not the actual data passed in

Arrays are of fixed size:
```vbscript
array names[5]
names[0] = "Abraham"
names[1] = "Ben"
names[2] = "Charlie"
names[3] = "Delta"
names.push("Emanuel")
names.push("Francisco") 'error here
```

# 39
Low-level languages:
- Machine code
- Assembly code
High-level languages:
- Compiled languages
- Interpreted languages

For exams, even system level languages like *Rust* and *C++* are considered high-level.

Programming paradigms:
- Procedural
- OOP
	- Inheritance
	- Encapsulation
	- Polymorphism
- Functional
- Declarative (vs Imperative)
# 40
Constants (mutating references):
```js
let x = 5;
x = 6; // allowed

const y = 7;
y = 9; // error flagged as y is a constant

const z = ["Hello"];
z.push("World"); // allowed as the array is stored as a reference
```

File handling:
```vbscript
myFile = openRead("sample.txt") 'Creates file handler
x = myFile.readLine()
myFile.close() 'File handler MUST be closed
```
# 41
| Mnemonic | Instruction        |
| -------- | ------------------ |
| ADD      | Add                |
| SUB      | Subtract           |
| STA      | Store              |
| LDA      | Load               |
| BRA      | Branch always      |
| BRP      | Branch if positive |
| BRZ      | Branch if zero     |
| INP      | Input              |
| OUT      | Output             |
| HLT      | End program        |
| DAT      | Data location      |
Remember that for LMC assembly, left hand side stores the reference to that line, and the right hand side is the input to  the parameter.
```asm
	INP
	ADD two
	HLT
two DAT 2
```
Also remember that *INP* and *OUT* store the value into the accumulator and outputs the value of the accumulator respectively. They **DO NOT ACCEPT A PARAMETER!**
# 42

### Addressing modes
#### Immediate
- Holds the value directly
#### Direct (What LMC uses)
- Points to a memory address
	- That holds the value 
#### Indirect
- Points to a memory address 
	- that holds another memory address
	-  that holds the value
#### Indexed
- Index register **(IR)** stores a value
- Index register is incremented (by the value provided) 
- and the value in the memory address is the value