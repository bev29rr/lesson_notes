# 4.1 Critical path analysis
Activities done by a group of people can have dependencies. For example, if person A wanted to do their morning routine, showering must precede leaving the house. They can be modelled into tables:

| Id  | Activity        | Duration /m | Immediate predecessors |
| --- | --------------- | ----------- | ---------------------- |
| A   | Shower          | 6           | -                      |
| B   | Dry hair        | 3           | A                      |
| C   | Eat breakfast   | 10          | -                      |
| D   | Brush teeth     | 4           | C                      |
| E   | Pack bag        | 7           | -                      |
| F   | Leave the house | 2           | B, D, E                |
## Activity networks
The same concept can be modelled using an activity network:
- **Arcs** represent **activities**
- **Vertices** represent **events**

Here the different lanes represent different people (assuming that someone else can dry person A's hair)

```text
          -----B-----
          |         |
*----A----*----C-------*----D----*----E----*
|                      |
-----E------------------
```
## Dummy activities
To turn this activity table into a simple digraph, we need to add dummy activities. 
- Dummy activities have an **activity duration** of **0**
- Dummy activities are shown as **dashed lines**
- Dummy activities are used to **show precedence(s) of an activity**

It is good habit to **have as few dummy variables** as possible.
## Identifying a critical path 
The minimum completion time is the **length of the longest path in the activity network** (there may be more than one). This path is called the critical path.
Any activity on the critical path is called **critical activity**.
### 1. Forward pass
First, a forward pass is made through the network, from **start to finish**, to find the earliest event times. (Remember that events are represented as vertices). (Pick the larger time)
- First event is set to 0
- If any events have dependencies, the forward pass must show the minimum time to complete **all the preceding events**
### 2. Backward pass
Next, a backwards pass is made through the network, from **finish to start** to find the latest event times. (Pick the smaller time)
- The latest time for the first event is 0
- The latest time for the finish event is the project completion time
- If any events have dependencies, then the latest event time is the latest time in which they can be completed to meet the requirements of the succeeding event(s)
If the earliest and the latest event times are the same, then the event is said to be a **critical event**

When an activity is **in between** critical events, it is a critical activities. The critical activities form a **critical path**.
## Float
Non-critical activities have **float**. Hence, the float for critical activities is 0.
### Total float
The maximum allowable delay for an activity that does **not** affect the _minimum project completion time_.

For an activity with time duration d (in between events i and j)
$f_\text{total} = (j_\text{latest} - i_\text{earliest}) - d$
### Independent float
The portion of the float that only exists to that activity (**only affects this activity**)
$f_\text{independent} = (j_\text{earliest} - i_\text{latest}) - d$
If the independent float is **negative**, set it to **zero**.
### Interfering float
The part of the total float that, if used, will reduce the float of the other activities.
$f_\text{interfering} = f_\text{total} - f_\text{independent}$
## Resourcing and scheduling
To analyse a critical path, a **cascade chart** is commonly used.
### Cascade chart (Gannt chart)
- **Time** on the **horizontal axis** (no vertical axis)
- Each bar represents an activity:
	- **Critical activities** take up the **bottom row**, joined up together
	- Non-critical activities start at their (first event's) **earliest possible start time** and have a width of their **duration**
		- Additionally, their **latest possible end time** (their float) is drawn with a **dashed box**
- Dummy activities are **omitted**

Note, for multiple critical paths, only draw one at the bottom, and stack the remaining activities on top.
### Resource histogram
Using the cascade chart, we can begin to think about resource (worker) allocation.
- **Time** on the **horizontal axis** and **workers** on the **vertical axis**
- **Flatten down all of the activities** (ignoring their float time) to use the minimum amount of workers possible (don't move them across their floats)

If an activity can be moved around its float, then it shows us that the problem can be done with less workers
# 4.2 Network flows
When continuous flow is required, the problem can be modelled as a **network flow**. The weights on the arcs show the capacities.
- All flows start at the **source** (vertex) -> S
- and end at the **sink** -> T
- The **maximum flow** of a **path** "ABCD" is the smallest **capacity** in the path
**Capacity graphs** and **feasible flow graphs** are different.
### Feasible flow graphs
- Shows the allowed movement (of liquid) that 
- When the capacity is the same as the feasible flow of an arc, it is saturated
- The **flow** of a **path** "ABCD" is the smallest **flow** in the path
### Checking flow validity
To check whether a flow is valid, create a arc-flow table (to check if the flow doesn't exceed the capacity):

| Arc | Flow | Capacity |
| --- | ---- | -------- |
|     |      |          |
Then, create a vertex-flow table (to check that the flow coming is is equal to the flow coming out):

| Vertex | Flow in | Flow out |
| ------ | ------- | -------- |
|        |         |          |
## Cuts
Notation:
```
{S, A}, {B, C, D, T}
```
where one set contains the source and the other set contains the sink.
Drawn in with a **dashed line** a separation of the vertices from the source set and the sink set.
#### Determining the cut value
The capacity (value) of the cut is the amount of arcs cut that contribute **from the source to the sink set**. Ignore the arcs cut that are directed towards the source set.
## Minimum cut theorem
Write out all of the possible cuts with their capacities. The **minimum capacity** is equal to the **maximum flow** through a network.
It also tells us that the arcs that are cut are **saturated**.
## Supersources & supersinks
Supersources (and supersinks) are a way to visualise all sources as one by extending the current network to make the sources all be derived from one. Make sure to balance the flows, where the flows in = flows out
