#![feature(refcell_replace_swap)]

extern crate find_folder;
extern crate reqwest;
extern crate select;
extern crate serenity;
extern crate qrcode_generator;
extern crate barcoders;
extern crate regex;

use barcoders::sym::code39::*;
use barcoders::generators::image::*;
use select::document::Document;
use select::predicate::{Attr, Class, Name, Predicate};
use serenity::model::channel::Embed;
use serenity::model::channel::EmbedImage;
use serenity::model::channel::Message;
use serenity::model::event::TypingStartEvent;
use serenity::model::gateway::Ready;
use serenity::model::guild::Member;
use serenity::model::id::ChannelId;
use serenity::model::id::GuildId;
use serenity::model::id::UserId;
use serenity::model::user::User;
use serenity::prelude::*;
use serenity::utils::Colour;
use serenity::utils::MessageBuilder;
// use serenity::http::raw::broadcast_typing;
use std::env;
use std::fs::{self, File};
use std::time::Instant;
use std::thread;
use std::io;
use std::io::BufWriter;
use std::io::prelude::*;
use std::cell::RefCell;
use std::path::Path;
use std::ffi::OsStr;
use qrcode_generator::QrCodeEcc;

mod scp;

const PREFIX: &str = "@@";
struct Handler;

impl EventHandler for Handler {
    // fn typing_start(&self, _ctx: Context, tse: TypingStartEvent){
    //     println!("type");
    //     tse.channel_id.broadcast_typing();
    //     tse.channel_id.say("type");
    // }
    fn guild_member_addition(&self, _ctx: Context, _guild_id: GuildId, new_member: Member) {
        let new_user: User = new_member.user.read().id.to_user().unwrap();
        let message = format!("Hi, {name}, welcome.\n\nThis is a 𝗻𝗶𝗰𝗲 server full of _friendly_ people.\n\nRemember, you will ***Always*** be welcome here!\n\n:)", name=new_user.name);
        match new_user.direct_message(|m| m.content(&message)) {
            Ok(_) => {}
            Err(e) => eprintln!("Error: {}", e),
        }
    }

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
            if msg.content.trim().len() == 0{
                return false;
            }

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

        // (&msg).channel_id.broadcast_typing();
        if is_command(&msg.content, "dance_char") {
            let msg_char: Vec<&str> = msg.content.trim().split_whitespace().collect();
            let msg_char: String = String::from(msg_char[1]).trim().to_lowercase();
            let acceptable_chars = [
                "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p",
                "q", "r", "s", "t", "u", "v", "w", "x", "y", "z", "0", "1", "2", "3", "4", "5",
                "6", "7", "8", "9",
            ];
            let message = RefCell::new(String::from("Error"));
            if acceptable_chars.contains(&((&msg_char).as_str())) {
                message
                    .replace_with(|_| format!("http://dance.cavifax.com/images/{}.gif", &msg_char));
            } else {
                message.replace_with(|_| format!("The char {} is not supported !", &msg_char));
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

        if is_command(&msg.content, "get_qr") {
            let thing:Vec<&str> = msg.content.trim().split_whitespace().collect();
            let qr = qrcode_generator::to_png_to_file(thing.get(1).unwrap_or(&""), QrCodeEcc::Medium,1024, "assets/images/qr.png").unwrap();
            let qr = vec!["assets/images/qr.png"];
            let _ = &msg.channel_id.send_files(qr, |m| m.content(""));
        }

        if is_command(&msg.content, "get_bar") {
            let thing:Vec<&str> = msg.content.trim().split_whitespace().collect();
            let barcode = Code39::new(thing.get(1).unwrap_or(&"mega oof").to_uppercase()).unwrap();
            let png = Image::png(80); // You must specify the height in pixels.
            let encoded = barcode.encode();

            // Image generators return a Result<Vec<u8>, barcoders::error::Error) of encoded bytes.
            let bytes = png.generate(&encoded[..]).unwrap();

            // Which you can then save to disk.
            let file = File::create(&Path::new("assets/images/bar.png")).unwrap();
            let mut writer = BufWriter::new(file);
            writer.write(&bytes[..]).unwrap();
            let bar = vec!["assets/images/bar.png"];
            let _ = &msg.channel_id.send_files(bar, |m| m.content(""));
        }

        // if (&msg).content.trim().to_lowercase().contains("alex") {
        //     match msg.channel_id.say("He be tard. --Jon Skeet") {
        //         Ok(_) => {}
        //         Err(e) => {
        //             eprintln!("Error: {}", e);
        //         }
        //     }
        // }

        // if ((&msg).content.trim().to_lowercase().contains("homo")) || (&msg).content.trim().to_lowercase().contains("gay") {
        //     let img = "https://i.imgur.com/U9UbZJI.png";
        //     match msg
        //         .channel_id
        //         .send_message(|m| m.embed(|e| e.title("HOMO").image(&img))){
        //             Ok(_) => {}
        //             Err(e) => {
        //                 eprintln!("Error: {}", e);
        //             }
        //         }
        // }

        if is_command(&msg.content, "bestgirl") {
            let img = "https://vignette.wikia.nocookie.net/haruhi/images/2/28/SuzumiyaHaruhi_Char2.jpg/revision/latest?cb=20171012164721";
            match msg
                .channel_id
                .send_message(|m| m.embed(|e| e.title("Best Female").image(&img)))
            {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
        if is_command(&msg.content, "avatar") {
            if (&msg).mentions.len() < 1 {
                let img = (&msg).author.avatar_url().unwrap();
                match msg
                    .channel_id
                    .send_message(|m| m.embed(|e| e.title("Avatar").image(&img)))
                {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            } else {
                for user in &msg.mentions {
                    let img = user.avatar_url().unwrap();
                    match msg
                        .channel_id
                        .send_message(|m| m.embed(|e| e.title("Avatar").image(&img)))
                    {
                        Ok(_) => {}
                        Err(e) => {
                            eprintln!("Error: {}", e);
                        }
                    }
                }
            }
        }

        if is_command(&msg.content, "nh") {
            let message: Vec<&str> = msg.content.trim().split_whitespace().collect();
            for num_str in message[1..].iter() {
                nh_p1(&msg, num_str);
            }
        }

        if is_command(&msg.content, "scp") {
            let message: Vec<&str> = msg.content.trim().split_whitespace().collect();
            for num_str in message[1..].iter() {
                scp_p1(&msg, num_str);
            }
        }

       fn scp_p1(msg: &Message, number_str: &str) {
            let number: i64 = number_str.parse().unwrap_or(0i64);
            if (number == 0i64) {
                match msg
                    .channel_id
                    .say("Make sure the the sequence in numbers only!")
                {
                    Ok(_) => {}
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                let s = scp::SCP::new(number);
                let title = format!("{} ({})", (&s).item_n, (&s).object_class );
                let description = (&s).get_description_short();
                let url = &(&s).url;
                let procedure = (&s).get_procedure_short();
                match msg.channel_id.send_message(|m| {
                    m.embed(|e| e.title(&title).description(&description).url(url).footer(|f| f.text(procedure)))
                }) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        }

        if is_command(&msg.content, "nhr"){
            let url = format!("https://nhentai.net/random/");
            let body = reqwest::get((&url).as_str()).unwrap().text().unwrap();
            // println!("body = {:?}", body);
            let document = Document::from(body.as_str());
            let mut title: String = String::from("Not Found");
            for node in document.find(Attr("name", "twitter:title")) {
                title = String::from(node.attr("content").unwrap());
            }
            let mut tag: String = String::from("");
            for node in document.find(Attr("name", "twitter:description")) {
                tag = String::from(node.attr("content").unwrap());
            }
            let mut imurl: String = String::from("");
            for node in document.find(Attr("itemprop", "image")) {
                imurl = String::from(node.attr("content").unwrap());
            }
            let mut description: String = String::from("");
            for node in document.find(Attr("id", "info")) {
                let description = node.first_child().unwrap().html();
                println!("{:?}", description);
            }

            match msg.channel_id.send_message(|m| {
                m.embed(|e| e.title(&title).description(&tag).image(&imurl).url(url))
            }) {
                Ok(_) => {}
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }

        if ((&msg).author.id.as_u64()) == &313783057838768128u64{
            // (&msg).channel_id.say("Hi");
            if (&msg).channel_id.as_u64() == &524396339338018873u64{
                let message: Vec<&str> = msg.content.trim().split_whitespace().collect();
                let is_all_nums = RefCell::new(true);
                for num_str in message.iter() {
                    if !num_str.parse::<i64>().is_ok(){
                        is_all_nums.replace(false);
                    }
                }
                if is_all_nums.into_inner(){
                    for num_str in message.iter() {
                        nh_p1(&msg, num_str);
                    }
                }
            }
        }

        fn nh_p1(msg: &Message, number_str: &str) {
            let number: i64 = number_str.parse().unwrap_or(0i64);
            if (number == 0i64) {
                match msg
                    .channel_id
                    .say("Make sure the the sequence in numbers only!")
                {
                    Ok(_) => {}
                    Err(e) => eprintln!("Error: {}", e),
                }
            } else {
                let url = format!("https://nhentai.net/g/{num}/", num = &number);
                let body = reqwest::get((&url).as_str()).unwrap().text().unwrap();
                // println!("body = {:?}", body);
                let document = Document::from(body.as_str());
                let mut title: String = String::from("Not Found");
                for node in document.find(Attr("name", "twitter:title")) {
                    title = String::from(node.attr("content").unwrap());
                }
                let mut tag: String = String::from("");
                for node in document.find(Attr("name", "twitter:description")) {
                    tag = String::from(node.attr("content").unwrap());
                }
                let mut imurl: String = String::from("");
                for node in document.find(Attr("itemprop", "image")) {
                    imurl = String::from(node.attr("content").unwrap());
                }
                let mut description: String = String::from("");
                for node in document.find(Attr("id", "info")) {
                    let description = node.first_child().unwrap().html();
                    println!("{:?}", description);
                }


                match msg.channel_id.send_message(|m| {
                    m.embed(|e| e.title(&title).description(&tag).image(&imurl).url(url))
                }) {
                    Ok(_) => {}
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
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

        if is_command(&msg.content, "get_log"){
            if msg.author.id.as_u64() == &313687614853218306u64{
                send_log(&msg.channel_id);
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

        fn send_log(channel_id: &ChannelId) {
            let log_path = env::var("DISCORD_LOG").expect("Expected DISCORD_LOG to be set in the environment");
            let log_path = log_path.as_str();
            // let log_path = Path::new(&log_path);
            // let log_file = File::open(&log_path).expect("Log file not found");
            let log_files = vec![log_path];
            let _ = channel_id.send_files(log_files, |m| m.content("Log file"));
        }

        fn download_attactments(msg: &Message){
            let attachments = &msg.attachments;
            if attachments.is_empty(){
                return;
            } else {
                for attachment in attachments{
                    //borrowed (stolen) code from https://docs.rs/serenity/0.5.11/serenity/model/channel/struct.Attachment.html
                    let content = match attachment.download() {
                        Ok(content) => content,
                        Err(why) => {
                            eprintln!("Error downloading attachment: {:?}", why);
                            return;
                        },
                    };
                    let file_path = format!("./attachments/files/{id}/{name}", id = &attachment.id, name = &attachment.filename);
                    let dir_path = format!("./attachments/files/{id}", id = &attachment.id);
                    let dir_path = Path::new(&dir_path);
                    fs::create_dir_all(&dir_path).unwrap();
                    let path = Path::new(&file_path);
                    let mut file = match File::create(&file_path) {
                        Ok(file) => file,
                        Err(why) => {
                            eprintln!("Error creating file: {:?}", why);
                            return;
                        },
                    };

                    if let Err(why) = file.write_all(&content) {
                        eprintln!("Error writing to file: {:?}", why);
                        return;
                    }
                }
            }
        }

        print_msg(&msg);
        thread::spawn(move|| {
            download_attactments(&msg);
        });
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
    if message.trim().len() == 0{
        return false;
    }
    // message.trim() == String::from(PREFIX) + command_name
    let message: Vec<&str> = message.trim().split_whitespace().collect();
    // match ([&message[0], message[1]].join(" ") == String::from(PREFIX) + command_name){
    (message[0] == String::from(PREFIX) + command_name)
}
