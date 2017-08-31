use std::cmp::Ordering;

pub fn tally(matches: &str) -> String {
    Tournament::new().inject(matches.to_owned()).score_table()
}

#[derive(Eq)]
struct Score {
    name: String,
    wins: u32,
    ties: u32,
    loss: u32,
}

impl Score {
    fn new(name: String) -> Score {
        Score {
            name,
            wins: 0,
            ties: 0,
            loss: 0,
        }
    }

    fn score(&self) -> u32 {
        self.wins * 3 + self.ties
    }

    fn total_played(&self) -> u32 {
        self.wins + self.ties + self.loss
    }
}

impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        if self.score() > other.score() {
            Ordering::Less
        } else if self.score() < other.score() {
            Ordering::Greater
        } else {
            self.name.cmp(&other.name)
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Score {
    fn eq(&self, other: &Score) -> bool {
        self.name == other.name
    }
}

struct Tournament(Vec<Score>);

impl Tournament {
    fn new() -> Tournament {
        Tournament(Vec::new())
    }

    fn inject(mut self, score_lines: String) -> Tournament {
        for line in score_lines.split('\n').filter(|&x| !x.is_empty()) {
            let (teams, result) = line.split_at(line.rfind(";").expect("score separator"));
            let (team_a, team_b) = teams.split_at(line.find(";").expect("team separator"));
            self.score(
                team_a.to_owned(),
                team_b.split_at(1).1.to_owned(),
                result.split_at(1).1,
            );
        }
        self
    }

    fn score_table(&self) -> String {
        let new_line = |x: &Score| {
            format!(
                "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                x.name,
                x.total_played(),
                x.wins,
                x.ties,
                x.loss,
                x.score()
            )
        };

        let mut table = "Team                           | MP |  W |  D |  L |  P".to_owned();
        table.push_str(
            self.0
                .iter()
                .map(|score| new_line(score))
                .collect::<String>()
                .as_str(),
        );

        table
    }

    fn update<T>(&mut self, name: String, func: T)
    where
        T: Fn(&mut Score)
    {
        let ref mut coll = self.0;
        if let None = coll.iter().skip_while(|&x| x.name != name).next() {
            coll.push(Score::new(name.to_owned()));
        }
        coll
            .into_iter()
            .skip_while(|ref x| x.name != name)
            .next()
            .map(|ref mut x| func(x));
        coll.sort();
    }

    fn score<'a>(&mut self, team_a: String, team_b: String, result: &'a str) {
        match result {
            "win" => {
                self.update(team_a, |ref mut x| x.wins += 1);
                self.update(team_b, |ref mut x| x.loss += 1);
            }
            "loss" => {
                self.update(team_a, |ref mut x| x.loss += 1);
                self.update(team_b, |ref mut x| x.wins += 1);
            }
            "draw" => {
                self.update(team_a, |ref mut x| x.ties += 1);
                self.update(team_b, |ref mut x| x.ties += 1);
            }
            _ => unreachable!(),
        }
    }
}
