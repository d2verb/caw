use caw::logs::{describe_log_groups, describe_log_streams, get_log_events};
use chrono::{Local, TimeZone};
use clap::{AppSettings, Clap};
use rusoto_logs::OutputLogEvent;

#[derive(Clap)]
#[clap(version = "0.0.1", author = "d2verb")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "0.0.1", author = "d2verb")]
    List(ListCommandArg),

    #[clap(version = "0.0.1", author = "d2verb")]
    Show(ShowCommandArg),
}

#[derive(Clap)]
struct ListCommandArg {
    #[clap(short = 'p', long)]
    log_group_name_pat: Option<String>,
}

#[derive(Clap)]
struct ShowCommandArg {
    log_group_name: String,

    #[clap(short = 'p', long)]
    message_pat: Option<String>,
}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmd {
        SubCommand::List(arg) => {
            let pat = match arg.log_group_name_pat {
                Some(s) => s,
                None => "".to_string(),
            };

            let log_groups: Vec<String> = describe_log_groups(None)
                .await
                .into_iter()
                .filter(|name| name.contains(&pat))
                .collect();

            for log_group in log_groups {
                println!("{}", log_group);
            }
        }
        SubCommand::Show(arg) => {
            let log_group_name = arg.log_group_name;

            let pat = match arg.message_pat {
                Some(s) => s,
                None => "".to_string(),
            };

            for log_stream in describe_log_streams(log_group_name.clone()).await {
                let log_stream_name = log_stream.log_stream_name.unwrap();
                let log_events: Vec<OutputLogEvent> =
                    get_log_events(log_group_name.clone(), log_stream_name.clone())
                        .await
                        .into_iter()
                        .filter(|event| event.message.clone().unwrap().contains(&pat))
                        .collect();

                for log_event in log_events {
                    let dt = Local.timestamp(log_event.timestamp.unwrap(), 0);
                    println!(
                        "{}: {}",
                        dt.format("%Y-%m-%d %H:%M:%S").to_string(),
                        log_event.message.unwrap()
                    );
                }
            }
        }
    }
}
