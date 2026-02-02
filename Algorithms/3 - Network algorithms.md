# 3.1 Algorithms for minimal connector problems
### Note:
Arcs -> Edges
Nodes -> Vertices
## Kruskal's algorithm
A **greedy** algorithm used to find the **MST** (Minimum Spanning Tree) of a connected, weighted, undirected graph 
### How to do:
1. Sort arcs in **ascending** order (of weight)
2. Choose the next arc and try and place it, discard it if it **creates a cycle**
3. Repeat until **n-1** arcs are chosen (minimum amount)
## Prim's algorithm
Same objectives as Kruskal's algorithm
### How to do:
1. Choose a random starting node
2. **Order** the arcs connecting new nodes in **ascending order** and **add the smallest one**
3. Repeat until **n-1** arcs have been chosen

We always choose **n-1** nodes because for a tree of **n** nodes, it contains **n-1** nodes is the minimum amount of nodes 
### How to do (in tabular form):
more practice on this before you can write it up #todo 

# 3.2 Finding the shortest path
## Dijkstra's Algorithm
Finds the _shortest path_ from a starting node to all other nodes. Only works with **connected graphs**, whose **weights are all ≥ 0**.

For this course, you must give each node this table:

| Order              | Distance from origin |
| ------------------ | -------------------- |
| Temporary Distance |                      |

### How to do:
1. Start at original node
2. Explore new nodes, adding their distance to the temporary distance
3. The smallest temporary distance should be assigned the next order
4. Repeat step 2, exploring new nodes **that have their order assigned** until the final node has its order assigned 

# 3.3 Calculating algorithmic complexities
## Kruskal's and Prim's algorithm
**Kruskal's algorithm needs the edges sorted**, and since the maximum amount of edges for a $K_n$ graph is $\frac{n(n-1)}{2} = \frac{1}{2}(n^2 - n)$ so sorting them would require $O(n^2)$ time complexity (**quadratic**).

Similarly, in **Prim's algorithm**, each iteration involves comparisons that are roughly linear -> "$O(n)$", the entire algorithm has a **quadratic** time complexity $O(n^2)$.

## Dijkstra's algorithm
Dijkstra's algorithm has also a two stage operation:
- Select **n** nodes -> $O(n)$
	- For each node, new node scanning can be up to **n** -> $O(n)$
Hence Dijkstra's algorithm also has a $O(n^2)$ time complexity.