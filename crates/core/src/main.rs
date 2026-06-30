#![feature(never_type)]

use chronobot::Chronobot;

fn main() -> anyhow::Result<!> {
    Chronobot::parse().execute()
}
