# 6.1 Simplex tableau
LPs are hard to solve graphically because:
1. They are constrained to 2 variables
2. Is it hard to represent the feasible region
A simplex tableau can work with these constraints.
## Setup
First, all inequalities must be **augmented**.

The simplex tableau has a **column** for the **objective variable** (P) followed by columns for the **variables** (x,y, ...) and the **slack variables** ($s_1, s_2$, ...). The final column is for the right hand side (RHS): this is the value of the augmented LPs.

For the first row, right the **coefficients** of the **objective function**. Next, for each row, right the coefficients of **each equation**, essentially creating a matrix of the coefficients.

E.g:
**Maximise**: 
	$P = 0.8x + y$
**Subject to**:
	$x+y+s_1=1000$ 
	$2x+y+s_2=1500$
	$3x+2y+s_3=2400$
**and**:
	$x \ge 0, y \ge 0, s_1 \ge 0, s_2 \ge 0, s_3 \ge 0$ 

| P   | x    | y   | $s_1$ | $s_2$ | $s_3$ | RHS  |
| --- | ---- | --- | ----- | ----- | ----- | ---- |
| 1   | -0.8 | -1  | 0     | 0     | 0     | 0    |
| 0   | 1    | 1   | 1     | 0     | 0     | 1000 |
| 0   | 2    | 1   | 0     | 1     | 0     | 1500 |
| 0   | 3    | 2   | 0     | 0     | 1     | 2400 |
Tip: remember that if $P = 0.8x + y$, then $P - 0.8x - y = 0$

**Non-basic** variables means that the values are set to **0**. Basic is the opposite, meaning it has a non-zero value.