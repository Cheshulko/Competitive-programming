// https://leetcode.com/problems/image-smoother

struct Solution {}

impl Solution {
    pub fn image_smoother(img: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        (0..img.len())
            .map(|i| {
                (0..img[i].len())
                    .map(|j| {
                        let x = [
                            img.get(i - 1).and_then(|y| y.get(j - 1)),
                            img.get(i - 1).and_then(|y| y.get(j)),
                            img.get(i - 1).and_then(|y| y.get(j + 1)),
                            img.get(i).and_then(|y| y.get(j - 1)),
                            img.get(i).and_then(|y| y.get(j)),
                            img.get(i).and_then(|y| y.get(j + 1)),
                            img.get(i + 1).and_then(|y| y.get(j - 1)),
                            img.get(i + 1).and_then(|y| y.get(j)),
                            img.get(i + 1).and_then(|y| y.get(j + 1)),
                        ]
                        .into_iter()
                        .filter_map(|x| x.map(|y| *y))
                        .collect::<Vec<_>>();
                        x.iter().sum::<i32>() / x.len() as i32
                    })
                    .collect()
            })
            .collect()
    }
}
