use rustc_hash::FxHashMap;
#[allow(dead_code)]
fn find_sue<F>(sues: &str, filter: F) -> Option<i32>
where
    F: Fn(&str, i32, i32) -> bool,
{
    let mut ticker_tape: FxHashMap<&'static str, i32> = FxHashMap::default();
    ticker_tape.insert("children", 3);
    ticker_tape.insert("cats", 7);
    ticker_tape.insert("samoyeds", 2);
    ticker_tape.insert("pomeranians", 3);
    ticker_tape.insert("akitas", 0);
    ticker_tape.insert("vizslas", 0);
    ticker_tape.insert("goldfish", 5);
    ticker_tape.insert("trees", 3);
    ticker_tape.insert("cars", 2);
    ticker_tape.insert("perfumes", 1);

    for line in sues.lines() {
        let (sue_num, attributes) = line.split_once(": ").unwrap();

        let attributes_split = attributes.split(", ");
        let mut all_match = true;
        for attribute in attributes_split {
            let mut attribute_split = attribute.split(": ");
            let attribute_name = attribute_split.next().unwrap();
            let attribute_num = attribute_split.next().unwrap().parse::<i32>().unwrap();
            if let Some(ticker_attribute_num) = ticker_tape.get(attribute_name) {
                all_match &= filter(attribute_name, attribute_num, *ticker_attribute_num);
            }
        }
        if all_match {
            return Some(sue_num.split(" ").nth(1).unwrap().parse::<i32>().unwrap());
        }
    }
    None
}

#[cfg(test)]
mod solution {
    use crate::input::get_input::get_input;

    #[test]
    fn find_sue() {
        let sues = get_input("sues").unwrap();
        let sue = super::find_sue(&sues, |_, attribute_num, ticker_attribute_num| {
            ticker_attribute_num == attribute_num
        })
        .unwrap();
        assert_eq!(sue, 40);
    }

    #[test]
    fn find_sue_actual() {
        let sues = get_input("sues").unwrap();
        let filter =
            |attribute_name: &str, attribute_num, ticker_attribute_num| match attribute_name {
                "cats" | "trees" => attribute_num > ticker_attribute_num,
                "pomeranians" | "goldfish" => attribute_num < ticker_attribute_num,
                _ => attribute_num == ticker_attribute_num,
            };
        let sue = super::find_sue(&sues, filter).unwrap();
        assert_eq!(sue, 241);
    }
}
