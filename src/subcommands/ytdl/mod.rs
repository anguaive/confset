use clap::{arg, ArgMatches, Command};
use std::{
    io::{BufRead, BufReader},
    process::{Command as StdCommand, Stdio},
};
use xshell::{cmd, Shell};

use crate::util::get_clipboard_contents;

mod aggregator;
mod message;
mod test_macros;

struct DownloadMetadata {
    title: String,
    url: String,
    filename: String,
}

#[derive(Debug, Clone, Copy)]
enum DownloadFormat {
    UpTo1440p,
    UpTo1080p,
    UpTo720p,
    UpTo480p,
    WorstVideo,
    AudioOnly,
}

fn get_download_format_specifier(format: DownloadFormat) -> &'static [&'static str] {
    match format {
        DownloadFormat::UpTo1440p => {
            &["-f", "bestvideo[height<=1440]+bestaudio/best[height<=1440]"]
        }
        DownloadFormat::UpTo1080p => {
            &["-f", "bestvideo[height<=1080]+bestaudio/best[height<=1080]"]
        }
        DownloadFormat::UpTo720p => &["-f", "bestvideo[height<=720]+bestaudio/best[height<=720]"],
        DownloadFormat::UpTo480p => &["-f", "bestvideo[height<=480]+bestaudio/best[height<=480]"],
        DownloadFormat::WorstVideo => &["-S", "+size,+br,+res,+fps"],
        DownloadFormat::AudioOnly => &["-x", "--audio-format", "mp3"],
    }
}

pub fn command_extension(cmd: Command) -> Command {
    let inner_subcommands = [
        Command::new("download").about("Download a video or audio file")
            .arg_required_else_help(true)
            .subcommand_required(true).subcommands(
                [
                    Command::new("url").about("Download a video or audio file from a given URL")
                        .arg_required_else_help(true)
                        .arg(arg!([URL] "The URL to download from")),
                    Command::new("clipboard").about("Download a video or audio file, trying to interpret the clipboard contents as an URL")
                ]
            ),
        Command::new("run_progress_server").about("Run the progress server, which aggregates the progress of ongoing downloads"),
        Command::new("get_download_progress").about("Get the progress of ongoing downloads")
    ];
    cmd.subcommand_required(true)
        .arg_required_else_help(true)
        .subcommands(inner_subcommands.iter())
}

fn get_download_url(download_args: &ArgMatches) -> anyhow::Result<String> {
    match download_args.subcommand() {
        Some(("url", url_args)) => {
            let url_arg = url_args
                .get_one::<String>("URL")
                .expect("URL should be a required argument");
            Ok(url_arg.clone())
        }
        Some(("clipboard", _)) => {
            let string_from_clipboard = get_clipboard_contents()?;
            Ok(string_from_clipboard)
        }
        _ => panic!("Missing required subcommand for 'download'"),
    }
}

fn parse_ytdl_output_line(line: &str) {}

fn download(download_args: &ArgMatches) -> anyhow::Result<()> {
    let url = get_download_url(download_args)?;

    let mut child = StdCommand::new("yt-dlp")
        .args(["-f", "160", "--progress", "--newline", &url])
        .stdout(Stdio::piped())
        .spawn()
        .expect("it to work");
    let stdout = child.stdout.take().expect("Child should have stdout");
    let bufreader = BufReader::new(stdout);
    for line in bufreader.lines() {
        match line {
            Ok(line) => println!("{}", line),
            Err(err) => println!("Error: {:?}", err),
        }
    }

    let ecode = child.wait().expect("wait on child failed");
    println!("Ecode: {}", ecode);
    Ok(())
}

pub fn run(sh: &Shell, args: &ArgMatches) -> anyhow::Result<()> {
    match args.subcommand() {
        Some(("download", download_args)) => download(download_args)?,
        Some(("run_progress_server", _)) => aggregator::run()?,
        Some(("get_download_progress", _)) => {}
        _ => {}
    }

    Ok(())
}
