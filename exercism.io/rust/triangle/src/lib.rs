use std::iter::Sum;

pub struct Triangle<T>
where
    T: PartialOrd + Sum + Copy,
{
    sides: [T; 3],
}

impl<T> Triangle<T>
where
    T: PartialOrd + Sum + Copy,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        if sides.iter().enumerate().any(|(ind, x)| {
            *x >= sides
                .iter()
                .enumerate()
                .filter(|(inner_ind, _)| inner_ind != &ind)
                .map(|(_, y)| *y)
                .sum()
        }) {
            None
        } else {
            Some(Triangle { sides })
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides.iter().enumerate().all(|(ind, x)| {
            self.sides
                .iter()
                .enumerate()
                .filter(|(inner_ind, _)| inner_ind != &ind)
                .all(|y| y.1 == x)
        })
    }

    pub fn is_isosceles(&self) -> bool {
        let is_equilateral = self.is_equilateral();

        let any = self.sides.iter().enumerate().any(|(ind, x)| {
            self.sides
                .iter()
                .enumerate()
                .filter(|(inner_ind, _)| inner_ind != &ind)
                .any(|y| y.1 == x)
        });

        !is_equilateral && any
    }

    pub fn is_scalene(&self) -> bool {
        !self.is_equilateral() && !self.is_isosceles()
    }
}
