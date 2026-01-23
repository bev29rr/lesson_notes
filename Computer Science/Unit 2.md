Sections 3 - 4
# Section 3
## 36 - Development Methodologies
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
## 37 & 38 - Pseudo-code
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

## 39 - Paradigms
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
## 40 -Procedural Programming
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
## 41 - Assembly

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
## 42 - Memory Addressing
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

## 43 - OOP
### Basic Terminology
```vbscript
class Dog 'class
	height 'attribute
	
	Sub Constructor(h) 'constructor (constructor is a procedure)
		 height = h
	End Sub
	
	Function GetHeight 'method
	End Function
End Class
```
## Encapsulation
Encapsulation means **hiding internal data** and only allowing access through **controlled methods**. This protects data.
```java
class BankAccount {
    private double balance;

    public void deposit(double amount) {
        balance += amount;
    }
    
    public double withdraw() {
	    if (amount - balance > 0) {
		    balance -= amount; // only allows withdraws of money if possible 
	    }
    }

    public double getBalance() {
        return balance;
    }
}
```
## Inheritance
```vb
Class Animal
	...
End Class

Class Dog 
	Inherits Animal
End Class
```
Allows a class to reuse and extend the behaviour of properties of another class.
## Polymorphism
**One interface, many behaviours**. This means that the same method call can behave differently on the object type.
```java
class Animal {
    void makeSound() {
        System.out.println("");
    }
}

class Dog extends Animal {
    void makeSound() {
        System.out.println("Bark");
    }
}

Animal a1 = new Animal();
Animal a2 = new Dog();

a1.makeSound(); // Nothing
a2.makeSound(); // Bark
```
# Section 4 - Exchanging Data
## 44 - Compression
**Lossy** looses data and **lossless** retains all of the data.
Data compression is important for **data transfers** via the **internet**
### Lossy
| Advantages                     | Disadvantages                                          |
| ------------------------------ | ------------------------------------------------------ |
| Reduces the size significantly | Permanent quality loss                                 |
| Adjustable compression levels  | Repeating re-encoding can reduce quality significantly |
### Lossless
| Advantages                                        | Disadvantages                                      |
| ------------------------------------------------- | -------------------------------------------------- |
| No data loss                                      | Smaller reduction in file size                     |
| Quality does not degrade after repeat re-encoding | May create overhead larger than original file size |
## 45 - RLE & Dictionary Coding
### Run Length Encoding
Groups consecutive repeated values.
### Dictionary Coding
Replaces **recurring patterns** of data with short references (codes).
## 46 - Encryption
**Symmetric** uses the same key to encrypt and decrypt, **asymmetric** uses a different key to encrypt and decrypt.

For asymmetric encryption, to encrypt a message, the message is **encrypted with the public key**. To **decrypt** the message, the **private key is used**.

For symmetric encryption, the key must be shared (privately).
## 47 & 48 - Hashing
Hashing is **one-way** (can't unhash) and returns a **fixed length output** every time. A good hashing algorithm is one that **rarely has collisions**.
## 49 - Databases
Conditions for a database:
- Structured
- Stores data
- Persistent ~~(most of the time)~~

### Terminology
- Tables/entities
	- Records/rows
		- Fields/attributes/columns
### Flat file vs relational databases
#### Flat file
Stores all data in a **single file** or **unlinked tables** with no **defined relationships**
#### Relational databases
Stores data across **multiple tables that are linked** using **primary and foreign keys**
## 50 - Normalisation
Normalisation is done in order, e.g.:
0NF -> 1NF -> 2NF -> 3NF -> ...
### Types
#### 1NF
- **Unique** field names
- "**Atomic**" data (single values, ~~arrays~~)
#### 2NF
- In 1NF
- If it has a composite key, all other fields are dependent on all parts of the key
#### 3NF
- In 2NF
- No transitive dependencies (fields cant be dependent on any non-primary-key fields)
#### Summary
0NF: any database
1NF: **no** records **within** records
2NF: dependent on the **whole key**
3NF: **only dependent** on the **key**

### Reasons for normalisation
- DRY
- Faster modifying/looking up
### Reasons against normalisation
- Harder to implement
- More complex queries
## 51 - Database keys and relationship types
Primary keys serve as unique identifiers for each row in a database. Foreign keys link the data from one table to other (references).
Relationship types:
- 1 : M (one to many)
- M : 1 (many to one)
- M : M (many to many)
Many to many relationships have to be done using a link table. This is where you add a buffer table which gets a many input and returns a singular output

## 52 & 53 - Data management & Querying
#### Capturing data
- OCR, OMR
- Forms
#### Querying data
- SQL queries (SELECT)
- Non-SQL queries
#### Managing data
- Inserting, updating
- Deleting
#### Exchanging data
- CSV/JSON
- API/Https
- USB Sticks/Discs
### Querying
#### Indexing
A **secondary key/index** can be added to a field in a table to make lookup times smaller.

E.g
```sql
SELECT * FROM this_table WHERE x = "...";
```
A separate table with a x -> val mapping can be created to reduce lookup times from $O(n) \rightarrow O(log(n))$

#### QBE
Haven't come up in exams (so far) but can.
It involves **filling in example values** instead of writing queries

## 54 - SQL

#TODO: finish off the last few slides

Statements that need to be known:
```sql
SELECT
FROM
WHERE
LIKE
AND
OR
DELETE
INSERT
DROP
JOIN 
-- Wildcards
```