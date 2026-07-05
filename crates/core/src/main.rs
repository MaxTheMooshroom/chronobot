#![feature(never_type)]

fn main() -> anyhow::Result<!> {
    chronobot::ChronobotArgs::execute()?;
}
