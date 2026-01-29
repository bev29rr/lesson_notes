# 1.1 Algorithms
## Conditions for an algorithm
- **Unambiguous** -> so that the person doesn't have to make choices
- **Deterministic** (no chance of random)
- **Finite** -> so it stops
# 1.2 Complexity
With a general polynomial of degree $n$:
$a_n​x^n+a_\text{n−1}​x^\text{n−1}+...+a_1x+a_0$ 
## Classic approach
Evaluating all terms for a known value of $x$, gives time complexity $O(n^2)$ as we need to calculate
$term_1+term_2+...+term_n$ gives $n(n+1)/2$ terms, which is $O(n^2)$.
## Horner's method
Instead of solving the polynomial as is, we can factor out terms of $x$, heavily reducing the amount of calculations required:
$a_n​x^n+a_\text{n-1}​x^\text{n-1}+...+a_1x+a_0$
$=(((..((a_n​x^n+a_\text{n-1})x+a_\text{n-2})x+a_\text{n-3})x+...)+a_1)x+a_0$ 
This nested form only requires $O(n)$ calculations.
# 1.3 Packing
## Common packing algorithms
- **First-fit algorithm**
- **First-fit decreasing algorithm**
- **Full-bin strategy**
# 1.4 Sorting 
## Quick sort
1. Pick pivot (usually first)
2. Compare (and sort):
	**itemFromLeft**: greatest item on the left hand side (of the pivot)
	**itemFromRight**: smallest item from the right hand side
3. Repeat until the list is sorted