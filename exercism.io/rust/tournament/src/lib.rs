use std::str::FromStr;

use crate::tournament::*;

mod tournament {
    use std::{cell::RefCell, collections::HashMap, rc::Rc, str::FromStr};

    pub enum Result {
        Win,
        Draw,
        Loss,
    }

    impl FromStr for Result {
        type Err = String;

        fn from_str(s: &str) -> std::result::Result<Result, String> {
            match s.to_lowercase().as_str() {
                "win" => Ok(Result::Win),
                "draw" => Ok(Result::Draw),
                "loss" => Ok(Result::Loss),
                _ => Err("Wrong result".to_owned()),
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Clone)]
    pub struct Team {
        name: String,
        wins: u32,
        losses: u32,
        draws: u32,
    }

    pub struct Tournament {
        teams_cnt: usize,
        pub teams: HashMap<String, usize>,
        pub scores: Vec<Rc<RefCell<Team>>>,
    }

    pub struct MatchResult {
        first_team: Team,
        second_team: Team,
        result: Result,
    }

    #[derive(Default)]
    pub struct MatchResultBuilder {
        first_team: Option<Team>,
        second_team: Option<Team>,
        result: Option<Result>,
    }

    impl Team {
        pub fn new(name: &str) -> Self {
            Team {
                name: name.to_string(),
                wins: 0,
                losses: 0,
                draws: 0,
            }
        }

        pub fn get_points(&self) -> u32 {
            self.wins * 3 + self.draws * 1 + self.losses * 0
        }
    }

    impl MatchResult {}

    impl MatchResultBuilder {
        pub fn new() -> MatchResultBuilder {
            MatchResultBuilder {
                first_team: None,
                second_team: None,
                result: None,
            }
        }

        pub fn add_first_team(mut self, team: Team) -> MatchResultBuilder {
            assert!(self.first_team.is_none());
            self.first_team = Some(team);
            self
        }

        pub fn add_second_team(mut self, team: Team) -> MatchResultBuilder {
            assert!(self.second_team.is_none());
            self.second_team = Some(team);
            self
        }

        pub fn add_result(mut self, result: Result) -> MatchResultBuilder {
            assert!(self.result.is_none());
            self.result = Some(result);
            self
        }

        pub fn build(self) -> std::result::Result<MatchResult, ()> {
            if [
                &self.first_team.is_none(),
                &self.second_team.is_none(),
                &self.result.is_none(),
            ]
            .into_iter()
            .any(|x| *x)
            {
                Err(())
            } else {
                Ok(MatchResult {
                    first_team: self.first_team.unwrap(),
                    second_team: self.second_team.unwrap(),
                    result: self.result.unwrap(),
                })
            }
        }
    }

    impl Tournament {
        pub fn new() -> Self {
            Tournament {
                teams_cnt: 0,
                teams: HashMap::new(),
                scores: vec![],
            }
        }

        // TODO: Split
        pub fn get_team_or_insert(&mut self, team: Team) -> &usize {
            self.teams.entry(team.name.clone()).or_insert_with(|| {
                self.scores.push(Rc::new(RefCell::new(team)));
                self.teams_cnt += 1;
                self.teams_cnt - 1
            })
        }

        pub fn add_match_result(&mut self, match_result: MatchResult) {
            let first_team_ind = *self.get_team_or_insert(match_result.first_team);
            let second_team_ind = *self.get_team_or_insert(match_result.second_team);

            match match_result.result {
                Result::Win => {
                    self.scores[first_team_ind].borrow_mut().wins += 1;
                    self.scores[second_team_ind].borrow_mut().losses += 1;
                }
                Result::Draw => {
                    self.scores[first_team_ind].borrow_mut().draws += 1;
                    self.scores[second_team_ind].borrow_mut().draws += 1;
                }
                Result::Loss => {
                    self.scores[first_team_ind].borrow_mut().losses += 1;
                    self.scores[second_team_ind].borrow_mut().wins += 1;
                }
            }
        }

        pub fn get_table(&mut self) -> String {
            self.sort();

            let ftm = |team: String, mp: String, w: String, d: String, l: String, p: String| {
                format!(
                    "{:<w1$}| {:^w2$}| {:^w2$}| {:^w2$}| {:^w2$}|{:>w2$}",
                    team,
                    mp,
                    w,
                    d,
                    l,
                    p,
                    w1 = 31,
                    w2 = 3,
                )
            };

            let mut content = vec![ftm(
                "Team".to_string(),
                "MP".to_string(),
                "W".to_string(),
                "D".to_string(),
                "L".to_string(),
                "P".to_string(),
            )];

            content.extend(
                self.scores
                    .iter()
                    .map(|team| {
                        let team = team.borrow();
                        ftm(
                            team.name.to_string(),
                            (team.wins + team.draws + team.losses).to_string(),
                            team.wins.to_string(),
                            team.draws.to_string(),
                            team.losses.to_string(),
                            team.get_points().to_string(),
                        )
                    })
                    .collect::<Vec<String>>(),
            );

            content.join("\n")
        }

        fn sort(&mut self) {
            self.scores.sort_by(|first, second| {
                let first_team = first.borrow();
                let second_team = second.borrow();

                let points_first = first_team.get_points();
                let points_second = second_team.get_points();

                match points_first.cmp(&points_second).reverse() {
                    std::cmp::Ordering::Equal => first_team.name.cmp(&second_team.name),
                    res @ _ => res,
                }
            });
        }
    }
}

pub fn tally(match_results: &str) -> String {
    let mut tournament = Tournament::new();

    match_results.split('\n').for_each(|result| {
        if let Ok(match_result) = result
            .split(';')
            .enumerate()
            .fold(
                MatchResultBuilder::new(),
                |builder, (ind, result_part)| match ind {
                    0 => builder.add_first_team(Team::new(result_part)),
                    1 => builder.add_second_team(Team::new(result_part)),
                    _ => builder.add_result(Result::from_str(result_part).unwrap()),
                },
            )
            .build()
        {
            tournament.add_match_result(match_result);
        }
    });

    let table = tournament.get_table();

    table
}
