Linear programming (LP) is a method of optimisation. It is used when we want to maximise or minimise a quantity while working under certain constraints.
# Feasible region
This is the region that all the inequalities can be satisfied in.
When representing the constraints graphically, shade in the **non-desired** areas
# Maximising linear equations
**Maximising** an expression $x+y$ in a feasible region:
- Turn expression into an equation $x+y=P$  (**objective function**)
- Find the greatest value of k that satisfies the equation $y=-x+P$ and the feasible region
	- This can be done by moving a ruler parallel to the line (from the top-right to the bottom-left)
To **minimise**, you do the same, but you move to the top-right

This can also be done by testing each vertex of the feasible region. Each vertex is called a **basic feasible solution**.
For integer problems (**ILP**), the solution will be a grid point within a feasible solution.
# Augmented form
This involves turning LPs into equalities with **slack variables**. For example:
$$
\begin{aligned}
x+y \le 6 \\
x \le 4 \\
x,y \ge 0
\end{aligned}
$$
becomes (in augmented form):
$$
\begin{aligned}
x+y+s_1 = 6 \\
x+s_2 = 4 \\
x,y, s_1, s_2 \ge 0
\end{aligned}
$$
### Slack Variables
Slack variables are **non-negative**.
The boundaries of the feasible region each correspond to a slack variable (or a variable) being zero.
