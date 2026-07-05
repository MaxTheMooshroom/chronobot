#![feature(never_type)]

use chronobot::Chronobot;

use harmony_autonomy::util::Enum;

#[derive(Enum)]
enum A {
    A,
    B(),
    C(u8),
    D(u8, String),
    E { __: u8 },
    F(u8),
    G(()),
}

fn main() -> anyhow::Result<!> {
    Chronobot::parse().execute()
}
