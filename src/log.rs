#![allow(unused)]

use chrono::Local;
use tokio::sync::mpsc;

use std::sync::OnceLock;

pub enum LogType {
    Info,
    Debug,
    Warn,
    Error,
}

static LOGGER: OnceLock<mpsc::Sender<String>> = OnceLock::new();

fn handler(mut rx: mpsc::Receiver<String>) {
    let mut batch = Vec::<String>::with_capacity(256);

    tokio::spawn(async move {
        loop {
            batch.clear();

            rx.recv_many(&mut batch, 256).await;
            println!("{}", batch.join("\n"));
        }
    });
}

impl LogType {
    fn prefix(&self) -> &'static str {
        match self {
            Self::Info  => "[INFO]",
            Self::Debug => "[DEBUG]",
            Self::Warn  => "[WARN]",
            Self::Error => "[ERROR]",
        }
    }
}

pub fn init() -> anyhow::Result<()> {
    if LOGGER.get().is_some() {
        return Err(anyhow::anyhow!("Logger already initialized."));
    }

    let (tx, rx) = mpsc::channel::<String>(1024);

    handler(rx);
    LOGGER.set(tx).unwrap();

    Ok(())
}

pub fn log<S: AsRef<str>>(lt: LogType, contents: S)
    -> impl std::future::Future<Output = Result<(), tokio::task::JoinError>>
{
    LOGGER.get().expect("crate::log::log() called before crate::log::init()");

    let ftime = Local::now().format("[%Y-%m-%d][%H:%M:%S%.3f]").to_string();
    let prefix = lt.prefix();
    let mut msg = String::with_capacity(
        ftime.len() + prefix.len() + contents.as_ref().len()
    );

    msg.push_str(&ftime);
    msg.push_str(lt.prefix());
    msg.push_str(contents.as_ref());

    tokio::spawn(async move {
        while let Err(e) = LOGGER.get().unwrap().try_send(msg) {
            msg = e.into_inner();
            tokio::task::yield_now().await;
        }
    })
}

#[derive(Clone)]
pub struct LogContext(String);

impl LogContext {
    pub fn new<'a>(context: &'a [&'a str]) -> Self {
        let len = context.iter().fold(0, |acc, s| acc + s.len() + 2);
        let mut s = String::with_capacity(len);
        for n in context {
            s.push('[');
            s.push_str(n);
            s.push(']');
        }
        Self(s)
    }

    pub fn log<S: AsRef<str>>(&self, lt: LogType, contents: S)
        -> impl std::future::Future<Output = Result<(), tokio::task::JoinError>>
    {
        LOGGER.get().expect("crate::log::LogContext::log() called before crate::log::init()");

        let ftime = Local::now()
            .format("[%Y-%m-%d][%H:%M:%S%.3f]")
            .to_string();

        let prefix = lt.prefix();
        let mut msg = String::with_capacity(
            ftime.len() + self.0.len() + prefix.len() + 2 + contents.as_ref().len()
        );

        msg.push_str(&ftime);
        msg.push_str(&self.0);
        msg.push_str(lt.prefix());
        msg.push_str(": ");
        msg.push_str(contents.as_ref());

        tokio::spawn(async move {
            while let Err(e) = LOGGER.get().unwrap().try_send(msg) {
                msg = e.into_inner();
                tokio::task::yield_now().await;
            }
        })
    }
}

