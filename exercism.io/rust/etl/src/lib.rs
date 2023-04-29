use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    BTreeMap::from_iter(
        h.iter()
            .map(|(value, chars)| {
                chars
                    .iter()
                    .map(|ch| (ch.to_ascii_lowercase(), *value))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>()
            .concat(),
    )
}
