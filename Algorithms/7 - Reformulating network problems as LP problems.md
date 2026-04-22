**Indicator variables** take the state of **0 or 1** (boolean) and indicate whether they are going to be used. 
# Dijkstra's algorithm
Example:
**Minimising** the distance travelled from (S-T) in the graph:
S-A = 8
S-B = 1
A-T = 5
A-B = 2
B-T = 7

## Dijkstra's LPs
### Minismise
We need to write out all of the arcs as indicator variables:
**Minimise:**
	$8SA+SB+2AB+2BA+5AT+7BT$

We can **ignore** $AS,AB$ as we wouldn't want to travel back to the start, likewise, we can ignore $TB,TA$ as we won't want to travel back once we have reached the destination node

### Subject to
To think about this, we have to think about **boolean logic**. When we are at node $S$, we can either choose to go to SA or SB, but not both (XOR). Hence, we can write the equation:
	$SA+SB=1$ 

For A, this isn't as simple. We must consider the routes to **go into** A and the routes **to leave** A.
Entering A: $SA,BA=0$ or $1$
Leaving A: $AB,AT=0$ or $1$
So, for A, it becomes:
	$SA+BA-AB-AT=0$
(This works as if we enter using one of the arcs, we must exit using an exit arc, meaning that the ones will cancel out)

Hence, all of them together are:
	$SA+SB=1$ 
	$SA+BA-AB-AT=0$
	$SB+AB-BA-BT=0$
	$AT+BT=0$
And, to prevent a loop of travelling multiple times through $BA/AB$, we use $BA+BC\le1$ 
# Flow problems
For the graph:
S-A = 8
S-B = 1
A-T = 10
A-B = 7
B-T = 16
## Flow LPs
In the flow LPs, these are **NOT** indicator variables. As flows are not binary.
### Maximising
To find the maximum flow, that is simple, we just want to maximise 
	$SA+SB$
### Subject to
Just like Dijkstra's LPs:
	$SA+BA-AB-AT=0$
	$SB+AB-BA-BT=0$
This is now because the flow in must equal the flow out.

We also need to add the constraints of the maximum possible flows allowed through each arc.
	$SA\le8,SB\le5,...$ and so on. 
(Remember that since $AB$ is not directed, flows can go both ways, so we write the flow constraint for $AB$ and $BA$)
# Critical Path Analysis
Critical Path Analysis for LP solvers 
(where start node is S and end node is T)
## Point based CPA LPs
**This is when the problem gives you the points in time as A,B, C ... (nodes) NOT the Activities (edges)**
### Objective function
The objective is to minimise the time taken for an activity network.
Hence, the objective function is to minimise the amount taken to reach the end node T:
Minimise:  T
### Constraints
We need to ensure that the time taken to reach each event is at least the time taken on the activity network. Hence, we need (for all arcs connected to the start node)

Subject to:
	$A \ge 4$
	$B \ge 3$

And for all nodes that are connected to other nodes, we need them to satisfy the fact that they must take at least the time taken to get to the previous node + the activities' time:
Subject to:
	$A \ge B + 13$ (A can be reached by completing B then A instead of through S)
	$T \ge B + 9$ 
	$T \ge A + 20$ 

Note that here,  (T can be reached to by either A or B)
### Normalising constraints
To feed them into an LP solver, we need to normalise them, however. This means that 
Subject to:
	$A \ge 4$ (No need to be normalised)
	...
	$A - B \ge 13$
	...
## Activity based CPA LPs
For questions where Activity is given/wanted, we need to take a different (but similar approach)
(as seen in Question 4 OCR MEI B MwA 2020)
### Objective function
The same as before.
Minimise: T
### Constraints
Since there are no nodes, we need to acknowledge that the time taken to each activity A will be at least the previous activities' time + the time taken to get there. For example:
Subject to:
	$T \ge 6 + I$ (This means that the time to complete T must be the time to complete activities up to I plus the time taken for I (6))
	$I \ge 10 + D$ 

Normalise!

# Bipartite Graph LPs
#todo: write up