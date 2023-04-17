struct Verse {
    n: u32,
}

impl Verse {
    fn new(n: u32) -> Self {
        Verse { n }
    }

    fn count(&self, n: u32) -> String {
        match n {
            0 => "no more bottles".to_string(),
            1 => "1 bottle".to_string(),
            _ => format!("{} bottles", n),
        }
    }

    pub fn count_init(&self) -> String {
        self.count(self.n)
    }

    pub fn count_after(&self) -> String {
        self.count((100 + self.n - 1) % 100)
    }

    fn take(&self) -> String {
        match self.n {
            1 => "Take it".to_string(),
            _ => "Take one".to_string(),
        }
    }

    pub fn action(&self) -> String {
        match self.n {
            0 => "Go to the store and buy some more".to_string(),
            _ => format!("{} down and pass it around", self.take()),
        }
    }
}

impl std::fmt::Display for Verse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        fn capitalize(s: &String) -> String {
            let s_capitalized = match s.chars().next() {
                None => String::new(),
                Some(c) => c.to_ascii_uppercase().to_string() + &s[1..],
            };
            s_capitalized
        }

        let bottle_init_count = self.count_init();
        let bottle_after_count = self.count_after();

        write!(
            f,
            "{} of beer on the wall, {} of beer.\n{}, {} of beer on the wall.\n",
            capitalize(&bottle_init_count),
            bottle_init_count,
            self.action(),
            bottle_after_count
        )
    }
}

pub fn verse(n: u32) -> String {
    format!("{}", Verse::new(n))
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
        .rev()
        .map(|i| verse(i))
        .collect::<Vec<String>>()
        .join("\n")
}
