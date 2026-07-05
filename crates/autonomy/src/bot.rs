#![allow(unused)]

// use serenity::Client;
use serenity::all::{Context, EventHandler, GatewayIntents, Message, Ready};
use tokio::sync::{OnceCell, RwLock};

use std::collections::HashSet;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub type CommandPrefix = &'static str;

pub type CommandFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
// pub type CommandAsync = fn(BotState, Arc<CommandContext>) -> CommandFuture<()>;
// pub type CommandSync = fn(BotState, Arc<CommandContext>);

pub struct CommandContext {
    pub ctx: Context,
    pub msg: Message,
    pub content: String,
}

pub struct BotStateRaw {
    bot_info: OnceCell<Ready>,
    api_handle: OnceCell<Context>,
    // command_sets: HashSet<impl crate::CommandSet>,
}

// #[derive(Clone)]
// pub struct BotState(Arc<RwLock<BotStateRaw>>);
//
// impl BotState {
//     async fn get<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, BotStateRaw> {
//         self.read().await
//     }
//
//     async fn get_mut<'a>(&'a mut self) -> tokio::sync::RwLockWriteGuard<'a, BotStateRaw> {
//         self.write().await
//     }
//
//     pub fn new() -> Self {
//         Self(
//             Arc::new(RwLock::new(BotStateRaw {
//                 bot_info: OnceCell::new(),
//                 api_handle: OnceCell::new(),
//                 // command_sets: HashMap::new(),
//             })),
//             // crate::log::LogContext::new(&["DISCORD"])
//         )
//     }
// }
//
// impl std::ops::Deref for BotState {
//     type Target = Arc<RwLock<BotStateRaw>>;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// #[async_trait::async_trait]
// impl EventHandler for BotState {
//     /// Dispatch messages to all command sets that have a
//     /// prefix found at the start of this message.
//     async fn message(&self, ctx: Context, msg: Message) {
//         let mut set = tokio::task::JoinSet::new();
//
//         let (cmd, content) = msg.content
//             .split_once(" ")
//             .map(|(a, b)| (a.to_string(), b.to_string()))
//             .unwrap_or_else(|| (msg.content.clone(), String::new()));
//
//         let cmd = cmd.to_string();
//         let content = content.to_string();
//
//         let mut ctx = Arc::new(CommandContext{
//             ctx,
//             msg,
//             content,
//         });
//
//         for (&prefix, cmdset) in self.read().await.command_sets.iter() {
//             if let Some(name) = cmd.strip_prefix(prefix)
//                 && let Some(cmd) = cmdset.commands.get(name) {
//                 set.spawn(cmd(self.clone(), ctx.clone()));
//             }
//         }
//
//         while let Some(result) = set.join_next().await {
//             if let Err(e) = result {
//                 panic!("A command errored! {e:?}");
//             }
//         }
//     }
//
//     async fn ready(&self, ctx: Context, bot_info: Ready) {
//         // self.info("Ready!").await;
//         //
//         // self.info(format!("API Version: {}", bot_info.version)).await;
//         // self.info(format!("Bot name: {}", bot_info.user.display_name())).await;
//         // self.info(format!("# of guilds: {}", bot_info.guilds.len())).await;
//
//         {
//             let handle = self.get().await;
//             handle.bot_info.set(bot_info).unwrap();
//             handle.api_handle.set(ctx).unwrap();
//         }
//     }
// }
// unsafe impl Send for BotState {}
// unsafe impl Sync for BotState {}

