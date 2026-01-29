# 2.1 The language of networks
**Graph** -> set of nodes/vertices
## Features of a graph
- **Loops** -> An edge with the same vertex at both ends
- **Order/degree** -> The number of edges incident on it. If there is a loop, then the arc is counted as two
## Types of a graph
- **Connected** -> every vertex is joined
- **Simple** -> no loops and no repeated edges
- **Simply connected** -> simple and connected
	- **Tree** -> a simply connected graph with the **minimum** number of arcs
	- **Complete graph** -> a simply connected graph with the **maximum** number of arcs
- **Bipartite graph** -> 
- A graph with $n$ vertices is denoted as $K_n$ 
- **Digraph** -> A graph with directed arcs
- **Undirected** -> Opposite of a digraph
## Incidence matrices 
Shows the number of edges between each pair of vertices.
E.g:

|       | A   | B   | C   |
| ----- | --- | --- | --- |
| **A** | 0   | 1   | 1   |
| **B** | 1   | 2   | 1   |
| **C** | 1   | 1   | 0   |
Notice how the leading diagonal ($y=-x$), the number of edges must be a multiple of 2.

### Isomorphic
Two graphs are said to be **isomorphic** if they have the same **incidence matrix**.

# 2.3 Modelling with networks
### Common Problems
- Travelling salesperson -> Start and end at the same point visiting all nodes
- Route Inspection -> Take every edge