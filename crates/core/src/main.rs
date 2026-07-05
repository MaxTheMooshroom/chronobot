#![feature(never_type)]

use h_autonomy::CommandContext;

fn main() -> anyhow::Result<!> {
    chronobot::ChronobotArgs::execute()?;
}
