use dotenv::dotenv;
use serenity::all::{
    Client, Context, CreateInteractionResponse, CreateInteractionResponseMessage, EventHandler,
    GatewayIntents, GuildId, Interaction, async_trait,
};
use std::env;

use serenity_commands::{Command, Commands, SubCommand};
#[derive(Debug, Commands)]
enum AllCommands {
    /// Ping the bot.
    Ping,

    /// Echo a message.
    Echo {
        /// The message to echo.
        message: String,
    },

    /// Perform math operations.
    Math(MathCommand),
}

impl AllCommands {
    fn run(self) -> String {
        match self {
            Self::Ping => "Pong!".to_string(),
            Self::Echo { message } => message,
            Self::Math(math) => math.run().to_string(),
        }
    }
}

#[derive(Debug, Command)]
enum MathCommand {
    /// Add two numbers.
    Add(BinaryOperation),

    /// Subtract two numbers.
    Subtract(BinaryOperation),

    /// Multiply two numbers.
    Multiply(BinaryOperation),

    /// Divide two numbers.
    Divide(BinaryOperation),

    /// Negate a number.
    Negate {
        /// The number to negate.
        a: f64,
    },
}

impl MathCommand {
    fn run(self) -> f64 {
        match self {
            Self::Add(BinaryOperation { a, b }) => a + b,
            Self::Subtract(BinaryOperation { a, b }) => a - b,
            Self::Multiply(BinaryOperation { a, b }) => a * b,
            Self::Divide(BinaryOperation { a, b }) => a / b,
            Self::Negate { a } => -a,
        }
    }
}

#[derive(Debug, SubCommand)]
struct BinaryOperation {
    /// The first number.
    a: f64,

    /// The second number.
    b: f64,
}

struct Handler {
    guild_id: GuildId,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, ctx: Context, _: serenity::all::Ready) {
        self.guild_id
            .set_commands(&ctx, AllCommands::create_commands())
            .await
            .unwrap();
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            let command_data = AllCommands::from_command_data(&command.data).unwrap();
            command
                .create_response(
                    ctx,
                    CreateInteractionResponse::Message(
                        CreateInteractionResponseMessage::new().content(command_data.run()),
                    ),
                )
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
async fn main() {
    // Login with a bot token from the environment
    dotenv().ok();
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let guild_id = GuildId::new(1323104058369769532);
    let main_handler = Handler { guild_id };
    // Create a new instance of the Client, logging in as a bot.
    let mut client = Client::builder(&token, intents)
        .event_handler(main_handler)
        .await
        .expect("Err creating client");

    // Start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
