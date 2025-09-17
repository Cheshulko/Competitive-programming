// https://leetcode.com/problems/design-a-food-rating-system

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet};

type CuisineId = usize;
type FoodId = usize;
type Rating = i32;

struct FoodRatings {
    cuisines: BTreeMap<String, CuisineId>,
    foods: BTreeMap<String, FoodId>,
    food_by_id: Vec<String>,
    food_details: Vec<(Rating, CuisineId)>,
    cuisine_foods: Vec<BTreeSet<(Reverse<Rating>, FoodId)>>,
}

impl FoodRatings {
    fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        fn build_unique_ordered<T: Ord + Clone>(v: &[T]) -> BTreeMap<T, usize> {
            v.iter()
                .fold(BTreeSet::new(), |mut hs, t| {
                    if !hs.contains(t) {
                        hs.insert(t.clone());
                    }

                    hs
                })
                .into_iter()
                .fold(BTreeMap::<T, usize>::new(), |mut hm, t| {
                    hm.insert(t, hm.len());

                    hm
                })
        }

        let cuisines_unique = build_unique_ordered(&cuisines);
        let foods_unique = build_unique_ordered(&foods);

        let food_by_id = foods_unique
            .iter()
            .map(|(food, _)| food.clone())
            .collect::<Vec<_>>();

        let food_details = foods.iter().zip(cuisines.iter()).zip(ratings.iter()).fold(
            vec![(0, 0); foods_unique.len()],
            |mut v, ((food, cuisine), rating)| {
                let cuisine_id = cuisines_unique.get(cuisine).copied().unwrap();
                let food_id = foods_unique.get(food).copied().unwrap();
                v[food_id] = (*rating, cuisine_id);

                v
            },
        );

        let cuisine_foods = foods
            .into_iter()
            .zip(cuisines.into_iter())
            .zip(ratings.into_iter())
            .fold(
                vec![BTreeSet::new(); cuisines_unique.len()],
                |mut v, ((food, cuisine), rating)| {
                    let cuisine_id = cuisines_unique.get(&cuisine).copied().unwrap();
                    let food_id = foods_unique.get(&food).copied().unwrap();

                    v[cuisine_id].insert((Reverse(rating), food_id));

                    v
                },
            );

        Self {
            cuisines: cuisines_unique,
            foods: foods_unique,
            food_details,
            food_by_id,
            cuisine_foods,
        }
    }

    fn change_rating(&mut self, food: String, new_rating: i32) {
        let food_id = self.foods.get(&food).copied().unwrap();
        let (rating, cuisine_id) = self.food_details[food_id];

        self.food_details[food_id].0 = new_rating;
        self.cuisine_foods[cuisine_id].remove(&(Reverse(rating), food_id));
        self.cuisine_foods[cuisine_id].insert((Reverse(new_rating), food_id));
    }

    fn highest_rated(&self, cuisine: String) -> String {
        let cuisine_id = self.cuisines.get(&cuisine).copied().unwrap();
        let &(_, food_id) = self.cuisine_foods[cuisine_id].first().unwrap();

        self.food_by_id[food_id].clone()
    }
}
