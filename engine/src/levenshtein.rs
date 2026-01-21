pub fn levenshtein(a: &str, b: &str) -> usize {
    let mut costs = (0..=b.len()).collect::<Vec<_>>();

    for (i, ca) in a.chars().enumerate() {
        let mut last = i;
        costs[0] = i + 1;

        for (j, cb) in b.chars().enumerate() {
            let new = if ca == cb {
                last
            } else {
                1 + last.min(costs[j]).min(costs[j + 1])
            };
            last = costs[j + 1];
            costs[j + 1] = new;
        }
    }
    costs[b.len()]
}