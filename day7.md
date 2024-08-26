# Day 7 Explanation
The solution involves constructing a tree of wires (nodes) with wire data stored in 3 hashmaps.
```
struct Circuit<'a> {
    /// The source wires for each wire.
    wires_upstream: FxHashMap<&'a str, WireSource<'a>>,
    /// The downstream wires for each wire.
    wires_downstreams: FxHashMap<&'a str, Vec<&'a str>>,
    /// Mutable state data for each wire.
    wires_state: FxHashMap<&'a str, WireState>,
    /// A list of wire identifiers in the order they should be processed.
    queue: Vec<&'a str>,
}
```
Each line of input gives us 2 key types of information:
1. Which wire is being supplied a signal from another wire
2. The type of gate/logic that the receiving wire has

This results in 2 actions for each line
1. Each wire on the left has the wire on the right added as a downstream wire.
2. The wire on the right gets a new entry with information about the source type.

This is captured in `wires_downstreams` and `wires_upstream`

Source type is represented by this enum
```
enum WireSource<'a> {
    GateAndWire(&'a str, &'a str),
    GateAndLiteral(u16, &'a str),
    GateOr(&'a str, &'a str),
    GateNot(&'a str),
    GateRshift(&'a str, i32),
    GateLshift(&'a str, i32),
    Wire(&'a str),
    Signal(u16),
}
```
By the time the input is finished processing, the above 2 hashmaps are fully populated and contain all necessary information needed to calculate signals for every wire. Signal is represented as an optional `Option<u16>`

### Signal propagation
For each wire, there is a counter that tracks how many missing source signals it has. When this counter hits 0, it is ready to have its signal calculated. The signal propagation algorithm will iterate through a `queue` of wires that is ordered by this counter in ascending order. For each wire that is retrieved:
1. The counter should be 0 allowing us to calculate its signal
2. Decrement the counter of all child wires
3. Re-sort the queue with a custom stable sorting algorithm
4. Retrieve the next wire and repeat

The sorting algorithm should take advantage of 3 facts:
1. The queue is sorted up to a known position.
2. The minimum value of the counter is 0, so any wire with a counter of 0 can be moved to the front.
3. Only wires with a counter of 0 need to be moved.
