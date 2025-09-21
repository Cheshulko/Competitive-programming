// https://leetcode.com/problems/design-movie-rental-system

use std::collections::{BTreeSet, HashMap};

type Movie = i32;
type Price = i32;
type Shop = i32;

struct MovieRentingSystem {
    movie_at_shop: HashMap<Movie, BTreeSet<(Price, Shop)>>,
    movie_at_shop_price: HashMap<(Movie, Shop), Price>,

    available_movie_at_shop: HashMap<Movie, BTreeSet<(Price, Shop)>>,
    rented_movie: BTreeSet<(Price, Shop, Movie)>,
}

impl MovieRentingSystem {
    fn new(_n: i32, entries: Vec<Vec<i32>>) -> Self {
        let (movie_at_shop, movie_at_shop_price) = entries.into_iter().fold(
            (
                HashMap::<Movie, BTreeSet<(Price, Shop)>>::new(),
                HashMap::<(Movie, Shop), Price>::new(),
            ),
            |(mut m_at_s, mut m_at_s_p), entry| {
                let &[shop, movie, price] = entry.as_slice() else {
                    panic!()
                };

                m_at_s.entry(movie).or_default().insert((price, shop));
                m_at_s_p.insert((movie, shop), price);

                (m_at_s, m_at_s_p)
            },
        );

        let available_movie_at_shop = movie_at_shop.clone();
        let rented_movie = BTreeSet::new();

        Self {
            movie_at_shop,
            movie_at_shop_price,
            available_movie_at_shop,
            rented_movie,
        }
    }

    fn search(&self, movie: i32) -> Vec<i32> {
        self.available_movie_at_shop
            .get(&movie)
            .map(|entries| entries.iter().take(5).map(|&(_, shop)| shop).collect())
            .unwrap_or_default()
    }

    fn rent(&mut self, shop: i32, movie: i32) {
        let price = self
            .movie_at_shop_price
            .get(&(movie, shop))
            .copied()
            .unwrap();

        self.available_movie_at_shop
            .entry(movie)
            .or_default()
            .remove(&(price, shop));

        self.rented_movie.insert((price, shop, movie));
    }

    fn drop(&mut self, shop: i32, movie: i32) {
        let price = self
            .movie_at_shop_price
            .get(&(movie, shop))
            .copied()
            .unwrap();

        self.available_movie_at_shop
            .entry(movie)
            .or_default()
            .insert((price, shop));

        self.rented_movie.remove(&(price, shop, movie));
    }

    fn report(&self) -> Vec<Vec<i32>> {
        self.rented_movie
            .iter()
            .take(5)
            .map(|&(_, shop, movie)| vec![shop, movie])
            .collect()
    }
}
