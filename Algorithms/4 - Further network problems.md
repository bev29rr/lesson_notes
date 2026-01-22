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

