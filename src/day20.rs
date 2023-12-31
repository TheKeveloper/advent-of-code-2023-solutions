use std::collections::{HashMap, VecDeque};
use std::iter::Sum;
use std::ops::Add;
use std::str::FromStr;

use itertools::Itertools;

use crate::common::Solution;

pub enum Day20 {}

impl Solution for Day20 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let mut system = System::from_lines(lines);
        let count: PulseCount = (0..1000).map(|_| system.press_button()).sum::<PulseCount>();

        (count.high * count.low).to_string()
    }
}

pub enum Day20P2 {}
impl Solution for Day20P2 {
    fn solve(lines: impl Iterator<Item = impl AsRef<str>>) -> String {
        let system = System::from_lines(lines);
        system.presses_until_rx().to_string()
    }
}

struct System {
    modules: HashMap<String, Module>,
}

impl System {
    pub fn from_lines(lines: impl Iterator<Item = impl AsRef<str>>) -> System {
        let mut modules: HashMap<String, Module> = lines
            .map(|s| s.as_ref().parse::<Module>().unwrap())
            .map(|module| (module.name.clone(), module))
            .collect();
        let names = modules.keys().map(|s| s.to_string()).collect::<Vec<_>>();

        // we need to prepopulate the inputs for the conjunctions so we can check if all are on
        for name in names {
            let outputs = modules.get(name.as_str()).unwrap().outputs.clone();
            for output in outputs {
                if let Some(Module {
                    module_type: ModuleType::Conjunction(conjunction),
                    ..
                }) = modules.get_mut(&output)
                {
                    conjunction.inputs.insert(name.clone(), Pulse::Low);
                }
            }
        }

        System { modules }
    }

    pub fn press_button(&mut self) -> PulseCount {
        let mut queue: VecDeque<Message> = VecDeque::new();
        queue.push_back(Message {
            source: "button".to_string(),
            destination: "broadcaster".to_string(),
            pulse: Pulse::Low,
        });

        let mut count = PulseCount::default();

        while let Some(message) = queue.pop_front() {
            count.update(&message.pulse);
            let Some(module) = self.modules.get_mut(&message.destination) else {
                continue;
            };
            for next in module.handle_pulse(message.source.clone(), message.pulse) {
                if next.pulse.is_high() {
                    module.sent_high_count += 1;
                }
                queue.push_back(next);
            }
        }

        count
    }

    pub fn presses_until_rx(mut self) -> usize {
        // inspected the input, rx has exactly one input, that is a conjunction
        let rx_input_name: String = self
            .modules
            .values()
            .filter(|module| module.outputs.iter().any(|s| s == "rx"))
            .exactly_one()
            .unwrap()
            .name
            .clone();

        let mut counts_until_high: HashMap<String, usize> = HashMap::new();

        let mut press_count: usize = 0;

        loop {
            self.press_button();
            press_count += 1;

            let rx_input = self.modules.get(&rx_input_name).unwrap();
            let ModuleType::Conjunction(conjunction) = &rx_input.module_type else {
                unreachable!()
            };

            for name in conjunction.inputs.keys() {
                if self.modules.get(name).unwrap().sent_high_count >= 1 {
                    counts_until_high
                        .entry(name.to_string())
                        .or_insert(press_count);
                }
            }

            if counts_until_high.len() == conjunction.inputs.len() {
                return counts_until_high
                    .values()
                    .fold(1, |a, b| num::integer::lcm(a, *b));
            }
        }
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Module {
    name: String,
    module_type: ModuleType,
    outputs: Vec<String>,
    sent_high_count: usize,
}

impl Module {
    pub fn handle_pulse(&mut self, source: String, pulse: Pulse) -> Vec<Message> {
        let name = &self.name;
        match &mut self.module_type {
            ModuleType::Broadcaster => self
                .outputs
                .iter()
                .map(move |out| Message {
                    source: name.clone(),
                    destination: out.clone(),
                    pulse,
                })
                .collect(),
            ModuleType::Conjunction(conjunction) => {
                conjunction.inputs.insert(source, pulse).unwrap();
                let output_pulse = conjunction.get_pulse();
                self.outputs
                    .iter()
                    .map(move |out| Message {
                        source: name.clone(),
                        destination: out.clone(),
                        pulse: output_pulse,
                    })
                    .collect()
            }
            ModuleType::FlipFlop(flipflop) => match pulse {
                Pulse::High => vec![],
                Pulse::Low => {
                    if flipflop.toggle() {
                        self.outputs
                            .iter()
                            .map(move |out| Message {
                                source: name.clone(),
                                destination: out.clone(),
                                pulse: Pulse::High,
                            })
                            .collect()
                    } else {
                        self.outputs
                            .iter()
                            .map(move |out| Message {
                                source: name.clone(),
                                destination: out.clone(),
                                pulse: Pulse::Low,
                            })
                            .collect()
                    }
                }
            },
        }
    }
}

impl FromStr for Module {
    type Err = anyhow::Error;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (first, outputs) = line.split_once(" -> ").ok_or_else(|| {
            anyhow::Error::msg("Could not parse module from line").context(line.to_string())
        })?;

        let (module_type, name): (ModuleType, String) = match first.trim().as_bytes() {
            [b'&', name @ ..] => (
                ModuleType::Conjunction(Conjunction::default()),
                String::from_utf8_lossy(name).to_string(),
            ),
            [b'%', name @ ..] => (
                ModuleType::FlipFlop(FlipFlop::default()),
                String::from_utf8_lossy(name).to_string(),
            ),
            [b'b', b'r', b'o', b'a', b'd', b'c', b'a', b's', b't', b'e', b'r'] => {
                (ModuleType::Broadcaster, "broadcaster".to_string())
            }
            _ => {
                return Err(anyhow::Error::msg("Invalid module received").context(line.to_string()))
            }
        };

        let outputs = outputs.split(", ").map(|s| s.to_string()).collect();
        Ok(Module {
            module_type,
            name,
            outputs,
            sent_high_count: 0,
        })
    }
}

#[derive(Eq, PartialEq, Clone, Debug)]
enum ModuleType {
    Conjunction(Conjunction),
    FlipFlop(FlipFlop),
    Broadcaster,
}

#[derive(Eq, PartialEq, Clone, Debug, Default)]
struct Conjunction {
    inputs: HashMap<String, Pulse>,
}

impl Conjunction {
    pub fn get_pulse(&self) -> Pulse {
        if self.inputs.values().all(|pulse| pulse.is_high()) {
            Pulse::Low
        } else {
            Pulse::High
        }
    }
}

#[derive(Eq, PartialEq, Clone, Hash, Debug, Default)]
struct FlipFlop {
    is_on: bool,
}

impl FlipFlop {
    pub fn toggle(&mut self) -> bool {
        self.is_on = !self.is_on;
        self.is_on
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Message {
    source: String,
    pulse: Pulse,
    destination: String,
}

#[derive(Eq, PartialEq, Clone, Copy, Hash, Debug)]
enum Pulse {
    High,
    Low,
}

impl Pulse {
    pub fn is_high(&self) -> bool {
        matches!(self, Pulse::High)
    }
}

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug, Default)]
struct PulseCount {
    high: usize,
    low: usize,
}

impl PulseCount {
    pub fn update(&mut self, pulse: &Pulse) {
        match pulse {
            Pulse::High => self.high += 1,
            Pulse::Low => self.low += 1,
        }
    }
}

impl Add for PulseCount {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        PulseCount {
            high: self.high + rhs.high,
            low: self.low + rhs.low,
        }
    }
}

impl Sum for PulseCount {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.reduce(|acc, cur| acc + cur)
            .unwrap_or(PulseCount::default())
    }
}

#[cfg(test)]
mod test {
    use crate::common::Solution;
    use crate::day20::{Day20, Day20P2};

    const FIRST_EXAMPLE: &str = r"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a";

    const SECOND_EXAMPLE: &str = r#"broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output"#;

    #[test]
    fn test_example() {
        assert_eq!(Day20::solve(FIRST_EXAMPLE.lines()), "32000000");
        assert_eq!(Day20::solve(SECOND_EXAMPLE.lines()), "11687500");
    }

    #[test]
    fn test_real_p2() {
        let input = r#"%jb -> fz
%xz -> ck, bg
%xm -> qt, cs
%df -> hc, lq
%mt -> sx
%fr -> ks, hc
%tn -> pf
%gt -> pp, kb
%jn -> ck, nz
%td -> kz
&rd -> vd
%pp -> gv, kb
&qt -> jb, vx, bt, gh, td, gb
%ms -> xz
%vx -> fp
%rb -> ck, mt
%nz -> hh
%fp -> rp, qt
%gd -> gc
%gv -> kb
%nl -> cc, hc
%cs -> qt
%kz -> jb, qt
%vg -> fr, hc
%zq -> qt, xm
%pv -> ps
&bt -> vd
%ps -> kb, rf
%hh -> ck, ms
broadcaster -> gn, gb, rb, df
%gh -> td
%rf -> kb, nm
%rp -> qt, gh
%gc -> kb, pv
%gb -> vx, qt
%rq -> ck, ts
%nm -> gt
%gn -> kb, tn
&ck -> nz, fv, rb, sx, ms, mt
&fv -> vd
%cc -> vg
%bg -> ck, rq
&hc -> qr, ch, df, dj, cc, rd
%qr -> dj
%gq -> hc, ch
&pr -> vd
%ks -> lc, hc
%dj -> nl
%fz -> qt, zq
%lq -> gq, hc
&kb -> pv, pr, tn, nm, pf, gn, gd
%ts -> ck
%lc -> hc
%jl -> ck, jn
%sx -> jl
%pf -> gd
&vd -> rx
%ch -> qr"#;
        assert_eq!(Day20P2::solve(input.lines()), "228300182686739");
    }
}
