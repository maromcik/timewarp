use clap::Parser;
use std::fs::{DirEntry, File};
use std::path::PathBuf;
use std::time::Duration;
use std::time::SystemTime;
use std::{fs, io};
use time::{Date, Month, OffsetDateTime, Time};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// Path to files
    #[clap(short, long, value_name = "PATH_TO_FILE")]
    path: String,

    /// offset in seconds
    #[arg(short = 'o', long, default_value_t = 1)]
    offset: u64,

    /// year to set
    #[arg(short = 'y', long, default_value_t = 0)]
    year: i32,

    /// month to set
    #[arg(short = 'm', long, default_value_t = 1)]
    month: u8,

    /// day to set
    #[arg(short = 'd', long, default_value_t = 1)]
    day: u8,

    /// hour to set
    #[arg(short = 'k', long, default_value_t = 0)]
    hour: u8,

    /// minute to set
    #[arg(short = 'l', long, default_value_t = 0)]
    minute: u8,

    /// second to set
    #[arg(short = 's', long, default_value_t = 0)]
    second: u8,
}

fn main() -> io::Result<()> {
    let cli = Cli::parse();
    let mut entries = list_files(&cli.path)?;
    entries.sort_by_key(|e| e.file_name());
    let date = Date::from_calendar_date(
        cli.year,
        Month::try_from(cli.month).expect("Incorrect Month entered"),
        cli.day,
    )
    .expect("Incorrect day entered");


    let time = Time::from_hms(cli.hour, cli.minute, cli.second).expect("Incorrect time entered");
    let current = OffsetDateTime::now_utc();
    let datetime = OffsetDateTime::new_in_offset(date, time, current.offset());
    let mut system_time = SystemTime::from(datetime);
    
    println!("Files to be updated (in this order):");
    for entry in &entries {
        if let Ok(file_name) = entry.file_name().into_string() {
            let current_time = OffsetDateTime::from(entry.metadata()?.modified()?);
            println!("{}: {} -> {}", file_name, current_time, datetime);    
        }
    }
    
    println!("Do you want to proceed? (y/n)");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    buffer = buffer.to_lowercase().trim().to_string();
    if buffer != "y" && buffer != "yes" {
        return Ok(());
    }
    
    let offset = Duration::from_secs(cli.offset);
    for entry in entries {
        let file = File::open(entry.path())?;
        file.set_modified(system_time)?;
        system_time = system_time.checked_add(offset).expect("Could not increase time");
    }
    println!("DONE");
    Ok(())
}

pub fn list_files<T: Into<PathBuf>>(path: T) -> io::Result<Vec<DirEntry>> {
    Ok(fs::read_dir(path.into())?.map_while(Result::ok).collect())
}
