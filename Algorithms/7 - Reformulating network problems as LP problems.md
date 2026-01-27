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
#todo Critical Path Analysis