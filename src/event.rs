use regex::Regex;
use std::fmt::Display;

#[derive(Debug)]
pub struct Event {
    pub name: String,
    pub weight: u32,
}

impl TryFrom<&str> for Event {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let re = Regex::new(r"(?<name>.+):(?<weight>\d+)$").unwrap();

        let Some(caps) = re.captures(value) else {
            return Ok(Self {
                name: value.to_string(),
                weight: 1,
            });
        };

        let name = caps.name("name").unwrap().as_str();

        let weight = caps
            .name("weight")
            .unwrap()
            .as_str()
            .parse::<u32>()
            .unwrap();

        Ok(Self {
            name: name.to_string(),
            weight,
        })
    }
}

impl Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}
