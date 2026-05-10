use serenity::{Client, async_trait};
use serenity::all::{Context, EventHandler, GatewayIntents, Message, Ready};
use tokio::sync::RwLock;

use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use crate::log::LogType;

pub type CommandFuture<T> = Pin<Box<dyn Future<Output = T> + Send>>;
pub type Command = fn(BotState, Arc<CommandContext>) -> CommandFuture<()>;
pub type CommandPrefix = &'static str;

pub struct CommandContext {
    pub ctx: Context,
    pub msg: Message,
    pub content: String,
}

pub struct CommandSet {
    prefix: CommandPrefix,
    commands: HashMap<String, Command>,
}

pub struct BotStateRaw {
    auth: String,
    command_sets: HashMap<CommandPrefix, CommandSet<>>,
}

#[derive(Clone)]
pub struct BotState(Arc<RwLock<BotStateRaw>>, crate::log::LogContext);

impl BotState {
    async fn get<'a>(&'a self) -> tokio::sync::RwLockReadGuard<'a, BotStateRaw> {
        self.read().await
    }

    async fn get_mut<'a>(&'a mut self) -> tokio::sync::RwLockWriteGuard<'a, BotStateRaw> {
        self.write().await
    }

    pub async fn new(auth: String) -> Self {
        Self(
            Arc::new(RwLock::new(BotStateRaw {
                auth,
                command_sets: HashMap::new(),
            })),
            crate::log::LogContext::new(&["DISCORD"])
        )
    }

    pub async fn add_command_set(&mut self, set: CommandSet) {
        self.get_mut().await.command_sets.insert(set.prefix, set);
    }

    pub async fn run(&self) {
        let intent = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT;

        self.1.log(LogType::Info, "Creating client...").await.unwrap();

        let mut client = Client::builder(&self.read().await.auth, intent)
            .event_handler(self.clone()).await
            .expect("Error acquiring client");

        self.1.log(LogType::Info, "Client created, starting bot...").await.unwrap();
        client.start().await.expect("Bot error occurred");
    }

    pub async fn info<S: AsRef<str>>(&self, s: S) {
        self.1.log(LogType::Info, s).await.unwrap();
    }

    pub async fn debug<S: AsRef<str>>(&self, s: S) {
        self.1.log(LogType::Debug, s).await.unwrap()
    }

    pub async fn warn<S: AsRef<str>>(&self, s: S) {
        self.1.log(LogType::Warn, s).await.unwrap()
    }

    pub async fn error<S: AsRef<str>>(&self, s: S) {
        self.1.log(LogType::Error, s).await.unwrap()
    }
}

impl std::ops::Deref for BotState {
    type Target = Arc<RwLock<BotStateRaw>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl CommandSet {
    pub fn new(prefix: CommandPrefix) -> Self {
        Self {
            prefix,
            commands: HashMap::new(),
        }
    }

    pub fn add_command(mut self, name: &'static str, cmd: Command) -> Self {
        self.commands.insert(name.into(), cmd);
        self
    }
}

#[async_trait]
impl EventHandler for BotState {
    /// Dispatch messages to all command sets that have a
    /// prefix found at the start of this message.
    async fn message(&self, ctx: Context, msg: Message) {
        let mut set = tokio::task::JoinSet::new();

        let (cmd, content) = msg.content
            .split_once(" ")
            .map(|(a, b)| (a.to_string(), b.to_string()))
            .unwrap_or_else(|| (msg.content.clone(), String::new()));

        let cmd = cmd.to_string();
        let content = content.to_string();

        let mut ctx = Arc::new(CommandContext{
            ctx,
            msg,
            content,
        });

        for (&prefix, cmdset) in self.read().await.command_sets.iter() {
            if let Some(name) = cmd.strip_prefix(prefix)
                && let Some(cmd) = cmdset.commands.get(name) {
                set.spawn(cmd(self.clone(), ctx.clone()));
            }
        }

        while let Some(result) = set.join_next().await {
            if let Err(e) = result {
                panic!("A command errored! {e:?}");
            }
        }
    }

    async fn ready(&self, _: Context, bot_info: Ready) {
        self.info("Ready!").await;

        self.info(format!("API Version: {}", bot_info.version)).await;
        self.info(format!("Bot name: {}", bot_info.user.display_name())).await;
        self.info(format!("# of guilds: {}", bot_info.guilds.len())).await;
    }
}
unsafe impl Send for BotState {}
unsafe impl Sync for BotState {}

