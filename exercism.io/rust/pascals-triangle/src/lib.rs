pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {
            rows: (1..=row_count).fold(Vec::<Vec<u32>>::new(), |mut res, cur| {
                let mut new_one: Vec<u32> = vec![1];

                new_one.extend(match cur - 1 {
                    1.. => {
                        let prev = &res[(cur - 2) as usize];
                        let mut new_one_part = prev.iter().enumerate().skip(1).fold(
                            Vec::<u32>::new(),
                            |mut v, (ind, cur)| {
                                v.push(cur + prev[ind - 1]);
                                v
                            },
                        );
                        new_one_part.push(1);
                        new_one_part
                    }
                    _ => Vec::<u32>::new(),
                });

                res.push(new_one);

                return res;
            }),
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
