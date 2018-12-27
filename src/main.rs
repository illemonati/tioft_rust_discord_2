#![feature(refcell_replace_swap)]


extern crate find_folder;
extern crate serenity;

use serenity::model::channel::Embed;
use serenity::model::channel::EmbedImage;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::utils::MessageBuilder;
use std::env;
use std::fs::File;
use std::time::Instant;


use std::cell::RefCell;

const PREFIX: &str = "@@";
struct Handler;

impl EventHandler for Handler {
    fn message(&self, ctx: Context, msg: Message) {
        let assets = find_folder::Search::KidsThenParents(3, 5)
            .for_folder("assets")
            .unwrap();
        let when_someone_calls_you_path = assets.join("images/when-someone-calls-you.jpg");

        let when_someone_calls_you = File::open(when_someone_calls_you_path).unwrap();

        let files = vec![(&when_someone_calls_you, "don't_call_for_me.jpg")];

        let RED: Colour = Colour::from_rgb(204, 0, 0);

        // println!("{:?}\n\n\n",&msg );

        fn is_person(msg: &Message, name: &str, id: u64) -> bool {
            let mut is_result = false;
            if msg.content.trim().to_lowercase().contains(name) {
                is_result = true;
            };

            for user in msg.mentions.iter() {
                if user.id.as_u64() == &id {
                    is_result = true;
                }
            }

            return is_result;
        }

        if is_command(&msg.content, "dance_char") {
            let msg_char: Vec<&str> = msg.content.trim().split_whitespace().collect();
            let msg_char: String = String::from(msg_char[1]).trim().to_lowercase();
            let acceptable_chars = ["a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4", "5", "6", "7", "8", "9"];
            let message = RefCell::new(String::from("Error"));
            if acceptable_chars.contains(&((&msg_char).as_str())){
                message.replace_with(|_|format!("http://dance.cavifax.com/images/{}.gif", &msg_char));
            } else {
                message.replace_with(|_|format!("The char {} is not supported !", &msg_char));
            }
            match msg.channel_id.say(message.into_inner()) {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        if (&msg).content.trim().to_lowercase().contains("beter") {
            let img = "https://i.kym-cdn.com/photos/images/newsfeed/001/233/110/001.png";
            match msg
                .channel_id
                .send_message(|m| m.embed(|e| e.title("BEater").image(&img)))
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        if is_command(&msg.content, "alex") {
            let img = "https://i.imgur.com/uzEivpm.png";
            match msg
                .channel_id
                .send_message(|m| m.embed(|e| e.title("BEater").image(&img)))
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        if (&msg).content.trim().to_lowercase().contains("alex") {
            match msg.channel_id.say("He be tard. --Jon Skeet") {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        if ((&msg).content.trim().to_lowercase().contains("homo")) || (&msg).content.trim().to_lowercase().contains("gay") {
            let img = "https://i.imgur.com/U9UbZJI.png";
            match msg
                .channel_id
                .send_message(|m| m.embed(|e| e.title("HOMO").image(&img))){
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
        }

        if is_person(&msg, "tong", 313687614853218306u64) {
            match msg.channel_id.send_files(files, |m| m.content("")) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        if is_command(&msg.content, "say") {
            let message: Vec<&str> = msg.content.trim().split_whitespace().collect();
            let message: String = String::from(message[1..].join(" "));
            match msg.channel_id.say(message) {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        if is_command(&msg.content, "ping") {
            match msg.channel_id.say("Pong") {
                Ok(_) => {}
                Err(e) => eprintln!("Error: {}", e),
            }
        }

        if is_command(&msg.content, "communism") {
            let communism_text = "A spectre is haunting Europe — the spectre of communism. All the powers of old Europe have entered into a holy alliance to exorcise this spectre: Pope and Tsar, Metternich and Guizot, French Radicals and German police-spies.Where is the party in opposition that has not been decried as communistic by its opponents in power? Where is the opposition that has not hurled back the branding reproach of communism, against the more advanced opposition parties, as well as against its reactionary adversaries?Two things result from this fact:I. Communism is already acknowledged by all European powers to be itself a power.II. It is high time that Communists should openly, in the face of the whole world, publish their views, their aims, their tendencies, and meet this nursery tale of the Spectre of Communism with a manifesto of the party itself.To this end, Communists of various nationalities have assembled in London and sketched the following manifesto, to be published in the English, French, German, Italian, Flemish and Danish languages";
            let communism_url =
                "https://www.marxists.org/archive/marx/works/1848/communist-manifesto/ch01.htm";
            let img = "http://www.fm-base.co.uk/forum/attachments/transfer-updates-custom-leagues-editing/211761d1324568367-ussr-yugoslavia-leagues-national-sides-database-soviet_union_ussr_grunge_flag_by_think0.jpg";
            match msg.channel_id.send_message(|m| {
                m.embed(|e| {
                    e.title("Manifesto of the Communist Party")
                        .description(&communism_text)
                        .color(RED)
                        .image(&img)
                        .url(&communism_url)
                })
            }) {
                Ok(_) => {}
                Err(e) => println!("Error: {}", e),
            }
        }

        fn print_msg(msg: &Message) {
            println!("-------------------------------------------------------------------------");
            println!("id: {}\n", (&msg).id);
            println!("content: {}\n", (&msg).content);
            println!("content_debug: {:?}\n", (&msg).content);
            println!("embeds: {:?}\n", (&msg).embeds);
            println!("attachments: {:?}\n", (&msg).attachments);
            println!("author: {}\n", (&msg).author);
            println!("author_debug: {:?}\n", (&msg).author);
            // println!("{:?}",(&msg).bot);
            // println!("{:?}",(&msg).name);
            // println!("{:?}",(&msg).discriminator);
            println!("channel_id: {}\n", (&msg).channel_id);
            println!("guild_id: {:?}\n", (&msg).guild_id);
            println!("kind: {:?}\n", (&msg).kind);
            // println!("{:?}",(&msg).memeber);
            println!("mention_everyone: {}\n", (&msg).mention_everyone);
            println!("mention_roles: {:?}\n", (&msg).mention_roles);
            println!("mentions: {:?}\n", (&msg).mentions);
            println!("tts: {}\n", (&msg).tts);
            println!("webhook_id: {:?}\n", (&msg).webhook_id);
            println!("timestamp: {}\n", (&msg).timestamp);
            println!("-------------------------------------------------------------------------");
            print!("\n\n\n");
        }

        print_msg(&msg);
    }

    fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!\n", ready.user.name);
    }
}

fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // let token = "123";
    let now = Instant::now();

    let mut client = Client::new(&token, Handler).expect("Err creating client");
    if let Err(why) = client.start_shards(1) {
        println!("Client error: {:?}", why);
    }
}

fn is_command(message: &String, command_name: &str) -> bool {
    // message.trim() == String::from(PREFIX) + command_name
    let message: Vec<&str> = message.trim().split_whitespace().collect();
    // match ([&message[0], message[1]].join(" ") == String::from(PREFIX) + command_name){
    (message[0] == String::from(PREFIX) + command_name)
}
