# Section 7 - Data structures
## 82 - Collection data types
### These include:
- Arrays
- Records
- Lists
- Tuples
Note: all of these are **ordered collections of items**
### Data types vs data structures
- **Data type:** 
	- what a variable can contain
	- a class of concrete items that share a property
- **Data structure:** 
	- a collection of types/forms of data
	- a way of organising/accessing data
	- an abstraction on top of data types

| Feature            | Array   | Record  | Lists   | Tuples    |
| ------------------ | ------- | ------- | ------- | --------- |
| **Static/dynamic** | Static  | Static  | Dynamic | Static    |
| **Mutability**     | Mutable | Mutable | Mutable | Immutable |
| **Item type**      | Same    | Any     | Any     | Any       |

Notes:
- Mutability means that the individual items can be 
- Static means known at **compile-time**, dynamic means known at **run-time**.

### Keywords
- Arrays: **contiguous** memory
- All: have a **single identifier**.
## 83, 84, 85 - Complex data structures
### How to implement the list Abstract Data Type
 - Using a static array with fixed-length item
	 - Fixed length
	 - Fixed length items
 - Using a static array with pointers
	 - Fixed length
	 - Pointers for each element
 - Using a dynamic array
	 - Arrays are fixed length, but when maximum is reaches, array can be copied to a larger one
	 - Can use fixed length or pointer items
 - Using a dynamic linked list
	 - See below
#### Methods:
- isEmpty()
- append(item)
- remove(item)
- search(item)
- length()
- index(item)
- insert(pos, item)
- pop()

// For each: methods, (basic) functionality and pointers of each
### (Dynamic) Linked Lists
#### Description: 
- Data is stored (non-contiguously) and has a pointer
- Pointer gives location of next node
#### Pointers
- headPointer
- nextFree
### Graphs
#### Description:
- Consists of edges and vertices
#### Methods #todo
#### Pointers
### Stack
#### Description
#### Methods
#### Pointers
### Queue
#### Description
#### Methods
#### Pointers
### Trees
#### Description
#### Methods
#### Pointers
### BST
#### Description
#### Methods
#### Pointers
### Hash Table
#### Description
#### Methods
#### Pointers
### Priority Queue
// More basic overview of priority queue
