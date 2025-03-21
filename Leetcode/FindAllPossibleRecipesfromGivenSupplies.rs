// https://leetcode.com/problems/find-all-possible-recipes-from-given-supplies

use std::collections::HashMap;

struct Solution {}

impl Solution {
    pub fn find_all_recipes(
        recipes: Vec<String>,
        ingredients: Vec<Vec<String>>,
        supplies: Vec<String>,
    ) -> Vec<String> {
        let mut index = HashMap::<String, usize>::new();
        let mut name = vec![];

        for recipe in recipes {
            name.push(recipe.clone());
            index.insert(recipe, index.len());
        }

        for supply in supplies {
            index.insert(supply, index.len());
        }

        let n = index.len();
        let mut can = vec![0; n];
        let mut adj = vec![vec![]; n];
        for (i, recipe_ingredients) in ingredients.iter().enumerate() {
            for ingredient in recipe_ingredients {
                if let Some(&j) = index.get(ingredient) {
                    adj[j].push(i);

                    if j >= ingredients.len() {
                        can[i] += 1;
                    }
                }
            }
        }

        let mut ans = vec![false; n];
        for _ in 0..n {
            for i in 0..n {
                if i < ingredients.len() && can[i] == ingredients[i].len() && !ans[i] {
                    ans[i] = true;

                    for &to in adj[i].iter() {
                        can[to] += 1;
                    }
                }
            }
        }

        ans.into_iter()
            .enumerate()
            .filter_map(|(i, can)| if can { Some(name[i].clone()) } else { None })
            .collect()
    }
}
