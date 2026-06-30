
use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

#[derive(Subcommand, Debug, Clone)]
pub enum SubcommandRoll {
    /// Rolls 1 Mutant Class, 2 of its subclasses, and
    /// (if applicable) a subclass `Variant`. See `whatis mutant-class`
    /// for more info.
    MutantClass,

    /// Rolls 2 Mutant Classes.
    Agent,

    /// Rolls a `Future Tech` and (if applicable) its subtable(s).
    FutureTech,

    /// Rolls either a minor, major, or apocalypotic anomaly.
    Anomaly { kind: String },

    /// Rolls a `Variant` of a Mutant Subclass.
    Variant { kind: String },

    /// Rolls a Mutation and (if applicable) its subtable(s).
    Mutation {
        #[arg(default_value_t = 1)]
        n_mutations: usize
    },

    /// A collection of dice-pools, each formatted as `<N>d<n>` or `<N>D<n>`.
    ///
    /// Eg. `5d12 2d20`
    DicePools {
        pools: Vec<DicePool>,
    },
}

#[derive(Debug, Clone, Copy)]
pub struct DicePool(pub (u8, u8));

pub enum CommandRollResult {
    Agent {
        // TODO: Define and use constant in crate::consts
        classes: [crate::chrono::types::MutantClassInstance; 2],

        mutations: Vec<crate::chrono::types::MutationInstance>,
    },
}

impl SubcommandRoll {
    pub fn do_rolls(&self) -> Result<()> {
        use SubcommandRoll::*;

        match self {
            DicePools{ pools } => Ok(()),
            _ => todo!(),
        }
    }
}

impl std::str::FromStr for DicePool {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (count_raw, size_raw) = s.split_once(['d', 'D'])
            .ok_or_else(|| anyhow!("Invalid dice-pool format {}", s))?;

        let count = count_raw.parse::<u8>()
            .map_err(|_|
                anyhow!("Invalid dice-pool format {}; N is not a valid u8", s)
            )?;
        let size = size_raw.parse::<u8>()
            .map_err(|_|
                anyhow!("Invalid dice-pool format {}; n is not a valid u8", s)
            )?;

        Ok(Self((count, size)))
    }
}

impl std::ops::Deref for DicePool {
    type Target = (u8, u8);

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

