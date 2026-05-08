#![allow(unused)]

use anyhow::{anyhow, Result};

use std::collections::HashMap;
use std::sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard};

static VARS: LazyLock<RwLock<HashMap<String, String>>>
    = LazyLock::new(|| RwLock::new(HashMap::new()));

pub fn read() -> Result<RwLockReadGuard<'static, HashMap<String, String>>> {
    VARS.read().map_err(|_| anyhow!("VARS lock is poisoned."))
}

pub fn write() -> Result<RwLockWriteGuard<'static, HashMap<String, String>>> {
    VARS.write().map_err(|_| anyhow!("VARS lock is poisoned."))
}

pub fn load() -> Result<()> {
    let mut env = std::env::current_dir()?;
    env.push(".env");

    let mut write_guard = write()?;
    for entry in dotenvy::Iter::new(std::fs::File::open(env)?) {
        let (name, value) = entry?;
        write_guard.insert(name, value);
    }

    Ok(())
}

