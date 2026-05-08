//! This module govorns dice-rolls and their behaviour,
//! including deserializing the rules from a csv.
//!
//! The rules are read from `<bin-dir>/tables/csv/dice.csv`,
//! and must be in the following format:
//!
//! | Name (String) | Number of Sides (Positive Integer) | Associated Color (String) | Results |
//!
//! Here's an example row:
//! ```csv
//! Advantage,6,Blue,//S/SO/OO/O
//! ```
//!
//! This will be deserialized by the [`csv`] crate, in combination with
//! [`deserialize_rollaxes`], into a list of [`DiceRollTableItem`].
//!
//! An entry for "Results" is a sequence of outcomes delimited by a `/`
//! character, with each character representing a score:
//! | Character | Meaning |
//! |   | Blank |
//! | B | Blank |
//! | S | Success |
//! | F | Failure |
//! | O | Opportunity |
//! | T | Threat |
//! | * | Triumph |
//! | X | Despair |
//!
//! NOTE:   "blank" can be denoted either by a lack of any characters,
//!         or by a single "B" character
//!
//! The number of items delimited by the `/` character MUST match the value
//! provided for the "Number of Sides" field.
//!
//! For example, the current "advantage" roll is defined as
//! ```
//! blank | blank | green | green blue | blue blue | blue
//! ```
//! So the corresponding csv entry would be `//S/SO/OO/O`.
//!
//! After all entries have been read from the CSV, they are collected into
//! [`ROLL_TABLES`]
//!
//! For convenience, the [`RollAxes`] can be added together to evaluate the
//! total outcome.
//! ```rust
//! let a = RollAxes::PassFail{ outcome: true, outstanding: true };
//! let b = RollAxes::PassFail{ outcome: false, outstanding: false };
//!
//! // DiceRollResult {
//! //     passfail:  0,
//! //     blank:     0,
//! //     luck:      0,
//! //     triumph:   1,
//! //     despair:   0,
//! // }
//! let result: DiceRollResult = &a + &b;
//! ```
//!

use rand::RngExt;

use std::collections::HashMap;
use std::sync::LazyLock;

// Will be initialized using read_dice_table upon first dereference (access).
// See LazyLock for more info.
pub static ROLL_TABLES: LazyLock<HashMap<String, DiceRollTableItem>>
    = LazyLock::new(read_dice_table);

#[derive(Debug)]
pub struct IoError(std::io::Error);

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum RollError {
    InvalidAxisChar(char),
    InvalidAxisString(String),
    InvalidRow(String),
    InvalidSource(String),

    ValidationError(String),

    Io(IoError),
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum RollAxes {
    Blank,
    OppThreat(bool),

    PassFail {
        outcome: bool,
        outstanding: bool,
    },
}

#[derive(Debug, Clone)]
struct RollAxesVec(Vec<RollAxes>);

#[derive(Debug)]
pub struct DiceRollTableItem {
    pub name: String,
    pub die_sides: u8,
    #[allow(unused)]
    pub color: String,
    pub outcomes: Vec<DiceRollResult>,
}

#[derive(Default, Debug, Clone, Copy)]
pub struct DiceRollResult {
    pub passfail:   i16,
    pub blank:      u8,
    pub luck:       i8,
    pub triumph:    u8,
    pub despair:    u8,
}

pub fn roll_dice(pool: &Vec<(String, u8)>) -> DiceRollResult {
    let mut result = DiceRollResult::default();

    for die in pool {
        assert!(ROLL_TABLES.contains_key(&die.0));
        result += &ROLL_TABLES[&die.0].roll(die.1);
    }

    result
}

fn read_dice_table() -> HashMap<String, DiceRollTableItem> {
    let mut table_path = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => panic!("Failed to locate the location of the chronobot executable: {:?}", e),
    };

    table_path.pop();
    table_path.push("tables/csv/dice.csv");

    if !(table_path.exists() && table_path.is_file()) {
        table_path = match std::env::current_dir() {
            Ok(p) => p,
            Err(e) => panic!("Failed to locate current working directory: {:?}", e),
        };
        table_path.push("tables/csv/dice.csv");
    }

    let mut table: std::fs::File = match std::fs::File::open(&table_path) {
        Ok(reader) => reader,
        Err(e) => panic!("Failed to read the file '{:?}': {:?}", table_path, e),
    };

    let mut table_raw = String::new();
    if let Err(e) = std::io::Read::read_to_string(&mut table, &mut table_raw) {
        panic!("Failed to read table contents: {:?}", e);
    }

    let mut table_entries = std::collections::HashMap::<String, DiceRollTableItem>::new();
    for line in table_raw.lines().skip(1) {
        let entry: DiceRollTableItem = match DiceRollTableItem::try_from(line) {
            Ok(item) => item,
            Err(e) => panic!("Failed to deserialize a line: {}", e),
        };

        table_entries.insert(entry.name.clone(), entry);
    }

    table_entries
}

impl DiceRollTableItem {
    pub fn roll(&self, count: u8) -> DiceRollResult {
        let mut result = DiceRollResult::default();
        let mut rng = rand::rng();

        for _ in 0..count {
            let roll = rng.random_range(0..self.die_sides) as usize;
            result += &self.outcomes[roll];
        }

        result
    }
}

impl From<Vec<RollAxes>> for RollAxesVec {
    fn from(vec: Vec<RollAxes>) -> Self {
        Self(vec)
    }
}

impl std::ops::Deref for RollAxesVec {
    type Target = Vec<RollAxes>;

    fn deref(&self) -> &Self::Target { &self.0 }
}

impl std::error::Error for RollError {}
impl std::fmt::Display for RollError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RollError::*;

        match self {
            InvalidAxisChar(c) => write!(f, "{}", c),

            InvalidAxisString(content)
            | InvalidRow(content)
            | InvalidSource(content)
                => f.write_str(content),

            ValidationError(what) => f.write_str(what),

            Io(e) => e.fmt(f),
        }
    }
}

impl Clone for IoError {
    fn clone(&self) -> Self {
        Self(self.0.kind().into())
    }
}

impl std::error::Error for IoError {}
impl std::fmt::Display for IoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a> std::ops::Add<&'a RollAxes> for &RollAxes {
    type Output = DiceRollResult;

    fn add(self, rhs: &'a RollAxes) -> Self::Output {
        let mut result = DiceRollResult::from(self);
        result += rhs;
        result
    }
}

// DiceRollResult += RollAxes
impl std::ops::AddAssign<RollAxes> for DiceRollResult {
    fn add_assign(&mut self, rhs: RollAxes) {
        *self += &rhs;
    }
}

// DiceRollResult += &RollAxes
impl<'a> std::ops::AddAssign<&'a RollAxes> for DiceRollResult {
    fn add_assign(&mut self, rhs: &'a RollAxes) {
        match rhs {
            RollAxes::Blank => { self.blank += 1; },

            RollAxes::PassFail{ outcome, outstanding } => {
                self.passfail += *outcome as i16 * 2 - 1;

                if *outstanding {
                    if *outcome {
                        self.triumph += 1;
                    } else {
                        self.despair += 1;
                    }
                }
            },

            RollAxes::OppThreat(ot) => self.luck += *ot as i8 * 2 - 1,
        }
    }
}

// DiceRollResult += DiceRollResult
// DiceRollResult += &DiceRollResult
impl<B> std::ops::AddAssign<B> for DiceRollResult
where
    B: std::borrow::Borrow<DiceRollResult>,
{
    fn add_assign(&mut self, rhs: B) {
        let rhs: &DiceRollResult = rhs.borrow();

        self.passfail += rhs.passfail;
        self.blank += rhs.blank;
        self.luck += rhs.luck;
        self.triumph += rhs.triumph;
        self.despair += rhs.despair;
    }
}

impl<'a> From<&'a RollAxesVec> for DiceRollResult {
    fn from(v: &'a RollAxesVec) -> Self {
        let mut result = Self::default();
        v.iter().for_each(|x| result += x);
        result
    }
}

impl<'a> From<&'a RollAxes> for DiceRollResult {
    fn from(item: &'a RollAxes) -> Self {
        let mut result = Self::default();
        result += item;
        result
    }
}

impl TryFrom<&str> for DiceRollTableItem {
    type Error = RollError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let mut parts = s.split(',');

        let name = parts.next()
            .ok_or_else(|| RollError::InvalidRow("dice row is missing name".into()))?
            .to_string();

        let die_sides = parts.next()
            .ok_or_else(|| RollError::InvalidRow("dice row missing die_sides".into()))?
            .parse::<u8>()
            .map_err(|_| RollError::ValidationError("dice row has invalid u8 for die_sides".into()))?;

        let color = parts.next()
            .ok_or_else(|| RollError::InvalidRow("dice row missing color".into()))?
            .to_string();


        let outcomes: Vec<DiceRollResult> = {
            use std::str::FromStr;

            let nested = parts.next()
                .ok_or_else(|| RollError::InvalidRow("dice row missing outcomes".into()))?
                .split('/')
                .map(RollAxesVec::from_str)
                .collect::<Result<Vec<RollAxesVec>, RollError>>()?;

            nested.iter()
                .map(DiceRollResult::from)
                .collect()
        };

        if outcomes.len() as u8 != die_sides {
            return Err(RollError::ValidationError(
                format!("- Length of results does not equal die_sides ({} != {});\n- Line: {}", outcomes.len(), die_sides, s)
            ));
        }

        Ok(Self {
            name,
            die_sides,
            color,
            outcomes,
        })
    }
}

impl TryFrom<char> for RollAxes {
    type Error = RollError;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'S' => Ok(RollAxes::PassFail{ outcome: true,  outstanding: false }),
            'F' => Ok(RollAxes::PassFail{ outcome: false, outstanding: false }),
            '*' => Ok(RollAxes::PassFail{ outcome: true,  outstanding: true  }),
            'X' => Ok(RollAxes::PassFail{ outcome: false, outstanding: true  }),

            'O' => Ok(RollAxes::OppThreat(true)),
            'T' => Ok(RollAxes::OppThreat(false)),

            _ => Err(RollError::InvalidAxisChar(c)),
        }
    }
}

impl std::str::FromStr for RollAxesVec {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "" | "B" => Ok(RollAxesVec(vec![RollAxes::Blank])),

            _ => s.chars()
                .map(RollAxes::try_from)
                .collect::<Result<Vec<RollAxes>, RollError>>()
                .map(RollAxesVec::from)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::roll_dice;

    #[test]
    fn roll0() {
        let empty = roll_dice(&vec![]);

        assert!(empty.passfail == 0);
        assert!(empty.blank == 0);
        assert!(empty.luck == 0);
        assert!(empty.triumph == 0);
        assert!(empty.despair == 0);
    }
}

