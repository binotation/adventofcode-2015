//! The solution here basically involves putting every wire on a queue sorted by how many signals it's missing.
//! We then take all the wires with no missing signals off the queue, propagate their signals, re-sort the queue and repeat.
use rustc_hash::FxHashMap;

#[allow(dead_code)]
#[derive(Debug)]
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

#[derive(Debug)]
struct WireState {
    signal: Option<u16>,
    /// How many source signals are missing
    missing_source_signals: i32,
}

#[allow(dead_code)]
struct Circuit<'a> {
    /// The source wires for each wire.
    wires_upstream: FxHashMap<&'a str, WireSource<'a>>,
    /// The downstream wires for each wire.
    wires_downstreams: FxHashMap<&'a str, Vec<&'a str>>,
    /// Mutable state data for each wire.
    wires_state: FxHashMap<&'a str, WireState>,
    /// A list of wire identifiers in the order they should be processed.
    queue: Vec<&'a str>,
    sorted_up_to: usize,
}

#[allow(dead_code)]
impl<'a> Circuit<'a> {
    fn new() -> Self {
        Self {
            wires_upstream: FxHashMap::default(),
            wires_downstreams: FxHashMap::default(),
            wires_state: FxHashMap::default(),
            queue: Vec::new(),
            sorted_up_to: 0,
        }
    }

    /// Move all wires with a missing signals counter of 0 to the front.
    fn update_queue(&mut self) {
        let mut front = self.sorted_up_to;
        for i in self.sorted_up_to + 1..self.queue.len() {
            let wire = self.queue[i];
            let missing_signals = self.wires_state.get(wire).unwrap().missing_source_signals;
            if missing_signals == 0 {
                // Swap with front
                front += 1;
                let temp = self.queue[front];
                self.queue[front] = wire;
                self.queue[i] = temp;
            }
        }
        self.sorted_up_to = front;
    }

    fn add_wire(&mut self, source: WireSource<'a>, wire: &'a str) {
        // Populate wire downstreams and get initial wire state
        let wire_state = match source {
            WireSource::GateAndWire(source_wire1, source_wire2)
            | WireSource::GateOr(source_wire1, source_wire2) => {
                self.wires_downstreams
                    .entry(source_wire1)
                    .and_modify(|wire_downstreams| wire_downstreams.push(wire))
                    .or_insert(vec![wire]);
                self.wires_downstreams
                    .entry(source_wire2)
                    .and_modify(|wire_downstreams| wire_downstreams.push(wire))
                    .or_insert(vec![wire]);
                WireState {
                    signal: None,
                    missing_source_signals: 2,
                }
            }
            WireSource::GateAndLiteral(_, source_wire1)
            | WireSource::GateNot(source_wire1)
            | WireSource::GateRshift(source_wire1, _)
            | WireSource::GateLshift(source_wire1, _)
            | WireSource::Wire(source_wire1) => {
                self.wires_downstreams
                    .entry(source_wire1)
                    .and_modify(|wire_downstreams| wire_downstreams.push(wire))
                    .or_insert(vec![wire]);
                WireState {
                    signal: None,
                    missing_source_signals: 1,
                }
            }
            WireSource::Signal(signal) => WireState {
                signal: Some(signal),
                missing_source_signals: 0,
            },
        };
        // Create remaining wire entries
        self.wires_upstream.insert(wire, source);
        self.wires_state.insert(wire, wire_state);
        self.queue.push(wire);
    }

    fn connect_wires(&mut self, wires: &'a str) {
        for line in wires.lines() {
            // line_split: input wire -> output wire
            let mut line_split = line.split("->");
            // input_split: [NOT] (wire_ident | literal) (OR | AND | RSHIFT | LSHIFT) (wire_ident | literal)
            let mut input_split = line_split
                .next()
                .unwrap()
                .strip_suffix(' ')
                .unwrap()
                .split(' ')
                .peekable();
            let output_wire = line_split.next().unwrap().strip_prefix(' ').unwrap();
            let token1 = input_split.next().unwrap();

            // Condition branch off token1
            if token1 == "NOT" {
                self.add_wire(
                    WireSource::GateNot(input_split.next().unwrap()),
                    output_wire,
                );
                continue;
            } else if let Ok(signal) = token1.parse::<u16>() {
                // If token1 is a literal
                if let Some(token2) = input_split.peek() {
                    if *token2 == "AND" {
                        input_split.next(); // Consume "AND"
                        self.add_wire(
                            WireSource::GateAndLiteral(
                                signal,
                                input_split.next().unwrap(), // Second source wire
                            ),
                            output_wire,
                        );
                        continue;
                    }
                } else {
                    // Wire source type is Signal
                    self.add_wire(WireSource::Signal(signal), output_wire);
                    continue;
                }
            }
            // Token1 is an input wire identifier
            if let Some(token2) = input_split.next() {
                match token2 {
                    "RSHIFT" => {
                        self.add_wire(
                            WireSource::GateRshift(
                                token1,
                                input_split.next().unwrap().parse::<i32>().unwrap(),
                            ),
                            output_wire,
                        );
                    }
                    "LSHIFT" => {
                        self.add_wire(
                            WireSource::GateLshift(
                                token1,
                                input_split.next().unwrap().parse::<i32>().unwrap(),
                            ),
                            output_wire,
                        );
                    }
                    "AND" => {
                        self.add_wire(
                            WireSource::GateAndWire(token1, input_split.next().unwrap()),
                            output_wire,
                        );
                    }
                    "OR" => {
                        self.add_wire(
                            WireSource::GateOr(token1, input_split.next().unwrap()),
                            output_wire,
                        );
                    }
                    _ => panic!("Invalid line: {}", line),
                }
            } else {
                self.add_wire(WireSource::Wire(token1), output_wire);
            }
        }
        self.queue.sort_unstable_by(|identifier1, identifier2| {
            let wire1 = self.wires_state.get(identifier1).unwrap();
            let wire2 = self.wires_state.get(identifier2).unwrap();
            wire1
                .missing_source_signals
                .cmp(&wire2.missing_source_signals)
        });
        for (i, wire) in self.queue.iter().enumerate() {
            if self.wires_state.get(wire).unwrap().missing_source_signals != 0 {
                self.sorted_up_to = i - 1;
                break;
            }
        }
    }

    fn propagate_signals(&mut self) {
        // Calculate signal for every wire
        for i in 0..self.queue.len() {
            let wire = self.queue[i];
            let wire_state = self.wires_state.get(wire).unwrap();
            // By the time we retrieve a wire from the queue, it should have no missing source signals, i.e.
            // it should be ready to hold a signal.
            assert_eq!(wire_state.missing_source_signals, 0);
            let wire_upstream = self.wires_upstream.get(wire).unwrap();

            // Calculate wire's signal
            match wire_upstream {
                WireSource::GateAndWire(source_wire1, source_wire2) => {
                    let source_signal1 =
                        self.wires_state.get(source_wire1).unwrap().signal.unwrap();
                    let source_signal2 =
                        self.wires_state.get(source_wire2).unwrap().signal.unwrap();
                    self.wires_state.get_mut(wire).unwrap().signal =
                        Some(source_signal1 & source_signal2);
                }
                WireSource::GateAndLiteral(source_signal1, source_wire2) => {
                    let source_signal2 =
                        self.wires_state.get(source_wire2).unwrap().signal.unwrap();
                    self.wires_state.get_mut(wire).unwrap().signal =
                        Some(source_signal1 & source_signal2);
                }
                WireSource::GateOr(source_wire1, source_wire2) => {
                    let source_signal1 =
                        self.wires_state.get(source_wire1).unwrap().signal.unwrap();
                    let source_signal2 =
                        self.wires_state.get(source_wire2).unwrap().signal.unwrap();
                    self.wires_state.get_mut(wire).unwrap().signal =
                        Some(source_signal1 | source_signal2);
                }
                WireSource::GateNot(source_wire1) => {
                    let source_signal1: u16 =
                        self.wires_state.get(source_wire1).unwrap().signal.unwrap();
                    self.wires_state.get_mut(wire).unwrap().signal = Some(!source_signal1);
                }
                WireSource::GateRshift(source_wire1, shift) => {
                    let source_signal1: u16 =
                        self.wires_state.get(source_wire1).unwrap().signal.unwrap();
                    self.wires_state.get_mut(wire).unwrap().signal = Some(source_signal1 >> shift);
                }
                WireSource::GateLshift(source_wire1, shift) => {
                    let source_signal1: u16 =
                        self.wires_state.get(source_wire1).unwrap().signal.unwrap();
                    self.wires_state.get_mut(wire).unwrap().signal = Some(source_signal1 << shift);
                }
                WireSource::Wire(source_wire1) => {
                    let source_signal1: u16 =
                        self.wires_state.get(source_wire1).unwrap().signal.unwrap();
                    self.wires_state.get_mut(wire).unwrap().signal = Some(source_signal1);
                }
                WireSource::Signal(..) => (), // Signal would've been populated when the wire was created.
            }

            // Decrement missing source signals counter for all downstream wires.
            if let Some(wire_downstreams) = self.wires_downstreams.get(wire) {
                for downstream_wire in wire_downstreams {
                    self.wires_state
                        .get_mut(downstream_wire)
                        .unwrap()
                        .missing_source_signals -= 1;
                }
                // Update queue after updating the missing source signals counter.
                self.update_queue();
            }
        }
    }

    fn get_signal(&self, wire: &str) -> u16 {
        self.wires_state.get(wire).unwrap().signal.unwrap()
    }
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn get_signal_a() {
        let wires = get_input("wires").unwrap();

        let mut circuit = Circuit::new();
        circuit.connect_wires(&wires);

        assert_eq!(circuit.queue.len(), circuit.wires_upstream.len());
        assert_eq!(circuit.queue.len(), circuit.wires_state.len());
        assert_eq!(circuit.queue.len() - 2, circuit.wires_downstreams.len());

        circuit.propagate_signals();
        assert_eq!(circuit.get_signal("a"), 16076);
    }

    #[test]
    fn get_signal_a_2() {
        let mut wires = get_input("wires").unwrap();
        let index = wires.find("19138 -> b").unwrap();
        wires.replace_range(index..index + 10, "16076 -> b");

        let mut circuit = Circuit::new();
        circuit.connect_wires(&wires);

        assert_eq!(circuit.queue.len(), circuit.wires_upstream.len());
        assert_eq!(circuit.queue.len(), circuit.wires_state.len());
        assert_eq!(circuit.queue.len() - 2, circuit.wires_downstreams.len());

        circuit.propagate_signals();
        assert_eq!(circuit.get_signal("a"), 2797);
    }
}
