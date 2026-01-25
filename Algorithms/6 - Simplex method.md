# 6.1 Simplex tableau
LPs are hard to solve graphically because:
1. They are constrained to 2 variables
2. Is it hard to represent the feasible region
A simplex tableau can work with these constraints.
## Setup
First, all inequalities must be **augmented**.

The simplex tableau has a **column** for the **objective variable** (P) followed by columns for the **variables** (x,y, ...) and the **slack variables** ($s_1, s_2$, ...). The final column is for the right hand side (RHS): this is the value of the augmented LPs.

For the first row, right the **coefficients** of the **objective function**. Next, for each row, right the coefficients of **each equation**, essentially creating a matrix of the coefficients.

E.g. :
**Maximise**: 
	$P = x + 0.8y$
**Subject to**:
	$x+y+s_1=1000$ 
	$2x+y+s_2=1500$
	$3x+2y+s_3=2400$
**and**:
	$x \ge 0, y \ge 0, s_1 \ge 0, s_2 \ge 0, s_3 \ge 0$ 

| P   | x   | y    | $s_1$ | $s_2$ | $s_3$ | RHS  |
| --- | --- | ---- | ----- | ----- | ----- | ---- |
| 1   | -1  | -0.8 | 0     | 0     | 0     | 0    |
| 0   | 1   | 1    | 1     | 0     | 0     | 1000 |
| 0   | 2   | 1    | 0     | 1     | 0     | 1500 |
| 0   | 3   | 2    | 0     | 0     | 1     | 2400 |
Tip: remember that if $P = x + 0.8y$, then $P - x - 0.8y = 0$
### Basic & non-basic variables
**Non-basic** variables means that the values are set to **zero**. **Basic** variables can **either** be **non-zero** or **zero**. The values of basic variables is assigned to the value of the RHS when the basic variables are 1. 

In the example above, $P, s_1, s_2, s_3$  are basic variables, having values of 0, 1000, 1500 and 2400, respectively. On the other hand, x and y are non-basic having the value of zero.
### Renaming linear equations
Recall that, for solving LPs, we used a graph. For the example above, the lines would be:
	$x+y=1000$ 
	$2x+y=1500$
	$3x+2y=2400$
Notice how these equations are the equivalent to the augmented lines, given that the respective slack variable was zero. Hence, we can rename each line to be:
	$s_1=0$ 
	$s_2=0$
	$s_3=0$
#intuitive
### Choosing a pivot
1. Analyse the objective function:
	$P=x+0.8y$ 
	 Notice how the $x$ variable increases at a faster rate than the $y$ value, hence, we first need to travel in the $x$
2. Find the equation that holds the maximum value for the variable **within the feasible region** ($x$ in this case):
	1. Find the values (of $x$) for each equation (**setting $y=0$**)
	2. Whichever one is the **smallest** is the going to be the **maximum value within the feasible region**
3. Select that coefficient as the pivot

E.g. : 
1. $P=x+0.8y \Rightarrow$ travel in $x$ direction (due to a faster rate of growth than $y$)
2. Given that $y=0$:
		$s_1=0:x+(0)=1000 \Rightarrow x=1000$ 
		$s_2=0:2x+(0)=1500 \Rightarrow x=750$ 
		$s_3=0:3x+2(0)=2400 \Rightarrow x=800$ 
3. Since $s_2=0$ has the smallest value for $x$, we select this pivot. Hence:

| P   | x     | y    | $s_1$ | $s_2$ | $s_3$ | RHS  |          |
| --- | ----- | ---- | ----- | ----- | ----- | ---- | -------- |
| 1   | -1    | -0.8 | 0     | 0     | 0     | 0    |          |
| 0   | 1     | 1    | 1     | 0     | 0     | 1000 |          |
| 0   | **2** | 1    | 0     | 1     | 0     | 1500 | <- Pivot |
| 0   | 3     | 2    | 0     | 0     | 1     | 2400 |          |
Graphically, it represents moving along the line $y=0$ and reaching the largest value for $x$ in the feasible region ($x=750$)
## Carrying out an iteration
Create a new table with these rows:
1. Start with the row that the pivot was on (the **pivot row**):
	This will contain the pivot equation in the form where the pivot is equal to one
2. Then, for each row, use the **new** pivot equation to eliminate the pivot variable ($x$)

E.g. :

| P   | x     | y               | $s_1$ | $s_2$          | $s_3$ | RHS |
| --- | ----- | --------------- | ----- | -------------- | ----- | --- |
| 1   | 0     | $-\frac{3}{10}$ | 0     | $\frac{1}{2}$  | 0     | 750 |
| 0   | 0     | $\frac{1}{2}$   | 1     | $-\frac{1}{2}$ | 0     | 250 |
| 0   | **1** | $\frac{1}{2}$   | 0     | $\frac{1}{2}$  | 0     | 750 |
| 0   | 0     | $\frac{1}{2}$   | 0     | $-\frac{3}{2}$ | 1     | 150 |
Example calculation (for the first row):
	**Original equation:**
	$P-0.8x-y=0$ 
	**New pivot equation:**
	$x+\frac{y}{2}+\frac{s_2}{2}=750$ 
	**Combined equations:**
	$P-0.3y+\frac{s_2}{2}=750$ 
### Iteration finalisation criteria
If an iteration has **non-negative basic variables**, then it corresponds to the **basic feasible solution**. In the example above:

**Non-basic variables:**
	$y=0$
	$s_2=0$
**Basic variables:**
	$P=750$
	$s_1=250$
	$x=750$
	$s_3=150$

This is where $y=0$ meets $s_2=0$