/// From some analysis of the text, there is a 1-1 mapping between
/// gates and output wires. I.e. the same two input wires cannot be provided
/// to 2 different wires.
use rustc_hash::FxHashMap;
use std::cell::RefCell;

#[allow(dead_code)]
#[derive(Debug, Clone)]
enum InputSource {
    GateAndWire(String, String),
    GateAndLiteral(u16, String),
    GateOr(String, String),
    GateNot(String),
    GateRshift(String, i32),
    GateLshift(String, i32),
    Wire(String),
    Signal(u16),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct SourceInfo {
    source: InputSource,
    signal: Option<u16>,
    missing_source_signals: i32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Wire {
    source_info: Option<SourceInfo>,
    supplies: Vec<String>,
}

thread_local! {
    static CIRCUIT: RefCell<Circuit> = RefCell::new(Circuit::new());
}

#[allow(dead_code)]
#[derive(Debug)]
struct Circuit {
    wires: FxHashMap<String, Wire>,
    queue: Vec<String>,
}

#[allow(dead_code)]
impl Circuit {
    fn new() -> Self {
        Self {
            wires: FxHashMap::default(),
            queue: Vec::new(),
        }
    }

    fn add_wire(&mut self, source: InputSource, identifier: &str) {
        let source_info = match source {
            InputSource::GateAndWire(ref input_wire1, ref input_wire2)
            | InputSource::GateOr(ref input_wire1, ref input_wire2) => {
                if let Some(input_wire) = self.wires.get_mut(input_wire1) {
                    input_wire.supplies.push(String::from(identifier));
                } else {
                    let mut input_wire = Wire {
                        source_info: None,
                        supplies: Vec::new(),
                    };
                    input_wire.supplies.push(String::from(identifier));
                    self.wires.insert(String::from(input_wire1), input_wire);
                }
                if let Some(input_wire) = self.wires.get_mut(input_wire2) {
                    input_wire.supplies.push(String::from(identifier));
                } else {
                    let mut input_wire = Wire {
                        source_info: None,
                        supplies: Vec::new(),
                    };
                    input_wire.supplies.push(String::from(identifier));
                    self.wires.insert(String::from(input_wire2), input_wire);
                }
                SourceInfo {
                    source,
                    signal: None,
                    missing_source_signals: 2,
                }
            }
            InputSource::GateAndLiteral(_, ref input_wire1)
            | InputSource::GateNot(ref input_wire1)
            | InputSource::GateRshift(ref input_wire1, _)
            | InputSource::GateLshift(ref input_wire1, _)
            | InputSource::Wire(ref input_wire1) => {
                if let Some(input_wire) = self.wires.get_mut(input_wire1) {
                    input_wire.supplies.push(String::from(identifier));
                } else {
                    let mut input_wire = Wire {
                        source_info: None,
                        supplies: Vec::new(),
                    };
                    input_wire.supplies.push(String::from(identifier));
                    self.wires.insert(String::from(input_wire1), input_wire);
                }
                SourceInfo {
                    source,
                    signal: None,
                    missing_source_signals: 1,
                }
            }
            InputSource::Signal(signal) => SourceInfo {
                source,
                signal: Some(signal),
                missing_source_signals: 0,
            },
        };
        if let Some(wire) = self.wires.get_mut(identifier) {
            wire.source_info = Some(source_info);
        } else {
            self.wires.insert(
                String::from(identifier),
                Wire {
                    source_info: Some(source_info),
                    supplies: Vec::new(),
                },
            );
        }

        self.queue.push(String::from(identifier));
    }

    fn connect_wires(&mut self, s: &str) {
        // format: input wire -> output wire
        let mut wires = s.split("->");
        let mut input_split = wires
            .next()
            .unwrap()
            .strip_suffix(' ')
            .unwrap()
            .split(' ')
            .peekable();
        let output_wire = wires.next().unwrap().strip_prefix(' ').unwrap();
        let token1 = input_split.next().unwrap();

        // Branch off token1
        if token1 == "NOT" {
            self.add_wire(
                InputSource::GateNot(String::from(input_split.next().unwrap())),
                output_wire,
            );
            return;
        } else if let Ok(signal) = token1.parse::<u16>() {
            if let Some(token2) = input_split.peek() {
                if *token2 == "AND" {
                    input_split.next();
                    self.add_wire(
                        InputSource::GateAndLiteral(
                            token1.parse().unwrap(),
                            String::from(input_split.next().unwrap()),
                        ),
                        output_wire,
                    );
                    return;
                }
            }
            // Token1 is a signal
            self.add_wire(InputSource::Signal(signal), output_wire);
            return;
        }
        // Token1 is an input wire, there may not be a token2
        if let Some(token2) = input_split.next() {
            match token2 {
                "RSHIFT" => {
                    self.add_wire(
                        InputSource::GateRshift(
                            String::from(token1),
                            input_split.next().unwrap().parse::<i32>().unwrap(),
                        ),
                        output_wire,
                    );
                }
                "LSHIFT" => {
                    self.add_wire(
                        InputSource::GateLshift(
                            String::from(token1),
                            input_split.next().unwrap().parse::<i32>().unwrap(),
                        ),
                        output_wire,
                    );
                }
                "AND" => {
                    self.add_wire(
                        InputSource::GateAndWire(
                            String::from(token1),
                            String::from(input_split.next().unwrap()),
                        ),
                        output_wire,
                    );
                }
                "OR" => {
                    self.add_wire(
                        InputSource::GateOr(
                            String::from(token1),
                            String::from(input_split.next().unwrap()),
                        ),
                        output_wire,
                    );
                }
                _ => panic!("The sentence could not be parse: {}", s),
            }
        } else {
            self.add_wire(InputSource::Wire(String::from(token1)), output_wire);
        }
        self.queue.sort_by(|identifier1, identifier2| {
            let wire1 = self.wires.get(identifier1).unwrap();
            let wire2 = self.wires.get(identifier2).unwrap();
            wire1
                .source_info
                .as_ref()
                .unwrap()
                .missing_source_signals
                .cmp(&wire2.source_info.as_ref().unwrap().missing_source_signals)
        });
    }

    fn propagate_signals(&mut self) {
        for i in 0..self.queue.len() {
            assert_eq!(
                self.wires
                    .get(&self.queue[i])
                    .unwrap()
                    .source_info
                    .as_ref()
                    .unwrap()
                    .missing_source_signals,
                0
            );
            // Get next in queue
            let identifier = &self.queue[i];
            let supplies: Vec<String> = {
                let wire = self.wires.get_mut(identifier).unwrap();
                wire.supplies.clone()
            };

            for downstream in &supplies {
                {
                    let downstream_wire = self.wires.get_mut(downstream).unwrap();
                    downstream_wire
                        .source_info
                        .as_mut()
                        .unwrap()
                        .missing_source_signals -= 1;
                }
                let (source_info, missing_signals) = {
                    let downstream_wire = self.wires.get(downstream).unwrap();
                    (
                        downstream_wire.source_info.as_ref().unwrap().clone(),
                        downstream_wire
                            .source_info
                            .as_ref()
                            .unwrap()
                            .missing_source_signals,
                    )
                };

                if missing_signals == 0 && source_info.signal.is_none() {
                    match source_info.source {
                        InputSource::GateAndWire(identifier1, identifier2) => {
                            let signal1 = self
                                .wires
                                .get(&identifier1)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            let signal2 = self
                                .wires
                                .get(&identifier2)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            self.wires
                                .get_mut(downstream)
                                .unwrap()
                                .source_info
                                .as_mut()
                                .unwrap()
                                .signal = Some(signal1 & signal2);
                        }
                        InputSource::GateAndLiteral(signal1, ref identifier2) => {
                            let signal2 = self
                                .wires
                                .get(identifier2)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            self.wires
                                .get_mut(downstream)
                                .unwrap()
                                .source_info
                                .as_mut()
                                .unwrap()
                                .signal = Some(signal1 & signal2);
                        }
                        InputSource::GateOr(identifier1, identifier2) => {
                            let signal1 = self
                                .wires
                                .get(&identifier1)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            let signal2 = self
                                .wires
                                .get(&identifier2)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            self.wires
                                .get_mut(downstream)
                                .unwrap()
                                .source_info
                                .as_mut()
                                .unwrap()
                                .signal = Some(signal1 | signal2);
                        }
                        InputSource::GateNot(identifier1) => {
                            let signal1: u16 = self
                                .wires
                                .get(&identifier1)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            self.wires
                                .get_mut(downstream)
                                .unwrap()
                                .source_info
                                .as_mut()
                                .unwrap()
                                .signal = Some(!signal1);
                        }
                        InputSource::GateRshift(identifier1, shift) => {
                            let signal1: u16 = self
                                .wires
                                .get(&identifier1)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            self.wires
                                .get_mut(downstream)
                                .unwrap()
                                .source_info
                                .as_mut()
                                .unwrap()
                                .signal = Some(signal1 >> shift);
                        }
                        InputSource::GateLshift(identifier1, shift) => {
                            let signal1: u16 = self
                                .wires
                                .get(&identifier1)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            self.wires
                                .get_mut(downstream)
                                .unwrap()
                                .source_info
                                .as_mut()
                                .unwrap()
                                .signal = Some(signal1 << shift);
                        }
                        InputSource::Wire(identifier1) => {
                            let signal1: u16 = self
                                .wires
                                .get(&identifier1)
                                .unwrap()
                                .source_info
                                .as_ref()
                                .unwrap()
                                .signal
                                .unwrap();
                            self.wires
                                .get_mut(downstream)
                                .unwrap()
                                .source_info
                                .as_mut()
                                .unwrap()
                                .signal = Some(signal1);
                        }
                        _ => (),
                    }
                }
            }
            self.queue.sort_by(|identifier1, identifier2| {
                let wire1 = self.wires.get(identifier1).unwrap();
                let wire2 = self.wires.get(identifier2).unwrap();
                wire1
                    .source_info
                    .as_ref()
                    .unwrap()
                    .missing_source_signals
                    .cmp(&wire2.source_info.as_ref().unwrap().missing_source_signals)
            });
        }
    }

    fn get_signal(&self, identifier: &str) -> u16 {
        self.wires
            .get(identifier)
            .unwrap()
            .source_info
            .as_ref()
            .unwrap()
            .signal
            .unwrap()
    }
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn get_signal_a() {
        for line in get_input("wires").unwrap().lines() {
            CIRCUIT.with(|c| c.borrow_mut().connect_wires(line));
        }
        CIRCUIT.with(|c| {
            assert_eq!(c.borrow().queue.len(), c.borrow().wires.len());
            c.borrow_mut().propagate_signals();
            for (identifier, wire) in c.borrow().wires.iter() {
                dbg!(identifier, wire);
            }
            assert_eq!(c.borrow().get_signal("a"), 16076);
        });
    }
}
