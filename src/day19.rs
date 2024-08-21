/// This one was lame.
use rustc_hash::{FxHashMap, FxHashSet};

struct Elements<'a> {
    s: &'a str,
    index: usize,
}

impl<'a> Iterator for Elements<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        let bytes = self.s.as_bytes();
        if self.index >= self.s.len() {
            return None;
        }
        let c = bytes[self.index];
        let mut i = self.index + 1;
        if c == b'e' {
        } else if c.is_ascii_uppercase() && i < bytes.len() && bytes[i].is_ascii_lowercase() {
            i += 1;
        }
        let result = &self.s[self.index..i];
        self.index = i;
        Some(result)
    }
}

trait ElementsExt {
    fn elements(&self) -> Elements;
}

impl ElementsExt for str {
    fn elements(&self) -> Elements {
        Elements { s: self, index: 0 }
    }
}

#[allow(dead_code)]
fn get_replacements(replacements_str: &str) -> FxHashMap<&str, Vec<&str>> {
    let mut replacements = FxHashMap::default();

    for line in replacements_str.lines() {
        let mut line_split = line.split(" => ");
        let molecule = line_split.next().unwrap();
        let replaced_with = line_split.next().unwrap();
        replacements
            .entry(molecule)
            .and_modify(|entry: &mut Vec<&str>| entry.push(replaced_with))
            .or_insert(vec![replaced_with]);
    }
    replacements
}

#[allow(dead_code)]
fn get_reverse_replacements(replacements_str: &str) -> FxHashMap<&str, &str> {
    let mut replacements = FxHashMap::default();

    for line in replacements_str.lines() {
        let mut line_split = line.split(" => ");
        let molecule = line_split.next().unwrap();
        let replaced_with = line_split.next().unwrap();
        assert!(replacements.insert(replaced_with, molecule).is_none());
    }
    replacements
}

#[allow(dead_code)]
fn calibrate_machine(
    medicine: &str,
    replacements: FxHashMap<&str, Vec<&str>>,
) -> FxHashSet<String> {
    let mut calibration: FxHashSet<String> = FxHashSet::default();
    let mut i = 0;
    for element in medicine.elements() {
        if let Some(molecule_replacements) = replacements.get(element) {
            let remaining = i + element.len();
            let mut new_molecule = String::with_capacity(medicine.len() + 7);
            new_molecule.push_str(&medicine[..i]);
            for molecule_replacement in molecule_replacements {
                new_molecule.push_str(molecule_replacement);
                new_molecule.push_str(&medicine[remaining..]);
                calibration.insert(new_molecule.clone());
                new_molecule.truncate(i);
            }
        }
        i += element.len();
    }
    calibration
}

#[cfg(test)]
mod solution {
    use super::*;
    use crate::input::get_input::get_input;

    #[test]
    fn calibrate_machine() {
        let replacements = get_input("replacements").unwrap();
        let mut replacements_split = replacements.split("\n\n");
        let replacements_map = get_replacements(replacements_split.next().unwrap());
        let calibration = super::calibrate_machine(
            replacements_split
                .next()
                .unwrap()
                .strip_suffix("\n")
                .unwrap(),
            replacements_map,
        );
        assert_eq!(calibration.len(), 535);
    }

    #[test]
    fn deconstruct_medicine() {
        let replacements = get_input("replacements").unwrap();
        let mut replacements_split = replacements.split("\n\n");
        let reverse_replacements_map = get_reverse_replacements(replacements_split.next().unwrap());
        let mut medicine = replacements_split
            .next()
            .unwrap()
            .strip_suffix("\n")
            .unwrap()
            .to_string();

        // This works by luck
        let mut count = 0;
        while medicine != "e" {
            for k in reverse_replacements_map.keys() {
                if medicine.contains(k) {
                    let index = medicine.rfind(k).unwrap();
                    medicine.replace_range(
                        index..index + k.len(),
                        reverse_replacements_map.get(k).unwrap(),
                    );
                    count += 1;
                }
            }
        }
        assert_eq!(count, 212);
    }
}
