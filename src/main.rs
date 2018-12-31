use serenity::client::Client;
use serenity::model::channel::Message;
use serenity::prelude::{Context, EventHandler};
use std::collections::HashMap;
use std::env;
use std::fmt;
use std::sync::Mutex;

struct ILJJ_Mk_X {
    entries: HashMap<&'static str, Box<Mutex<Command + Send>>>,
}
impl ILJJ_Mk_X {
    fn new(entries: HashMap<&'static str, Box<Mutex<Command + Send>>>) -> ILJJ_Mk_X {
        ILJJ_Mk_X { entries }
    }
}

trait Command {
    fn help(&self) -> &str {
        return "no description";
    }

    fn execute(&self, ctx: &Context, msg: &Message) {
        msg.reply("not implemented");
    }
}

struct DebugPrint;
impl Command for DebugPrint {
    fn execute(&self, ctx: &Context, msg: &Message) {
        let reply: String = format!("{:?}", msg);
        msg.reply(reply.as_str());
    }
}

impl EventHandler for ILJJ_Mk_X {
    fn message(&self, ctx: Context, msg: Message) {
        if msg.content.starts_with("!mkx") {
            let argv: Vec<&str> = msg.content.split(" ").collect();

            let cmd = argv.get(1).unwrap();

            match self.entries.get(cmd) {
                Some(module) => {
                    let module = module.lock().unwrap();
                    module.execute(&ctx, &msg);
                }
                None => {
                    msg.reply("no such command");
                }
            }
        }
    }
}

fn main() {
    let mut entries: HashMap<&str, Box<Mutex<Command + Send>>> = HashMap::new();
    entries.insert("debug", Box::new(Mutex::new(DebugPrint)));

    let mkx = ILJJ_Mk_X::new(entries);
    let token = &env::var("DISCORD_TOKEN").unwrap();
    let mut client = Client::new(token, mkx).unwrap();

    if let Err(why) = client.start() {
        println!("{:?}", why);
    }
}
