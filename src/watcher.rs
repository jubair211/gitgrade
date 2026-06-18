use chrono::Local;
use std::fs::{OpenOptions, File};
use std::io::{BufReader, BufRead, Write};
use std::path::Path;
use notify::{Watcher, RecursiveMode, recommended_watcher, Event, EventKind};
use std::sync::mpsc::channel;
use git2::Repository;
use std::collections::HashMap;

const LOG_FILE: &str = ".gitgrade_activity.log";

pub fn watch(path: &str) {
    println!("Watching '{}' for file activity... Press CTRL+C to stop.", path);
    println!("Logging to: {}", LOG_FILE);

    let (tx, rx) = channel();

    let mut watcher = recommended_watcher(move |res: Result<Event, _>| {
        if let Ok(event) = res {
            let _ = tx.send(event);
        }
    }).expect("Failed to create watcher");

    watcher.watch(Path::new(path), RecursiveMode::Recursive)
        .expect("Failed to watch path");

    let mut log = OpenOptions::new()
        .create(true)
        .append(true)
        .open(LOG_FILE)
        .expect("Cannot open log file");

    loop {
        if let Ok(event) = rx.recv() {
            let now = Local::now();
            let timestamp = now.format("%Y-%m-%d %H:%M:%S").to_string();

            let kind = match event.kind {
                EventKind::Create(_) => "CREATE",
                EventKind::Modify(_) => "MODIFY",
                EventKind::Remove(_) => "REMOVE",
                EventKind::Access(_) => "OPEN",
                _ => continue,
            };

            for path in &event.paths {
                let path_str = path.to_string_lossy();
                if path_str.contains(".git") { continue; }
                let line = format!("{}|{}|{}", timestamp, kind, path_str);
                writeln!(log, "{}", line).ok();
                println!("[{}] {} {}", timestamp, kind, path_str);
            }
        }
    }
}

pub fn compare(path: &str) {
    println!("=== GitGrade Activity Compare ===\n");

    // Read activity log
    let log_path = Path::new(LOG_FILE);
    if !log_path.exists() {
        println!("No activity log found. Run 'gitgrade watch .' first!");
        return;
    }

    let file = File::open(LOG_FILE).expect("Cannot open log file");
    let reader = BufReader::new(file);

    let mut file_events: HashMap<String, u32> = HashMap::new();
    for line in reader.lines().flatten() {
        let parts: Vec<&str> = line.splitn(3, '|').collect();
        if parts.len() == 3 {
            let date = &parts[0][..10];
            *file_events.entry(date.to_string()).or_insert(0) += 1;
        }
    }

    // Read git commits
    let mut git_commits: HashMap<String, u32> = HashMap::new();
    if let Ok(repo) = Repository::open(path) {
        if let Ok(mut revwalk) = repo.revwalk() {
            revwalk.push_head().ok();
            for oid in revwalk.flatten() {
                if let Ok(commit) = repo.find_commit(oid) {
                    let time = commit.time().seconds();
                    if let Some(dt) = chrono::DateTime::from_timestamp(time, 0) {
                        let date = dt.with_timezone(&Local).format("%Y-%m-%d").to_string();
                        *git_commits.entry(date).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    // Combine all dates
    let mut all_dates: Vec<String> = file_events.keys()
        .chain(git_commits.keys())
        .cloned()
        .collect::<std::collections::HashSet<_>>()
        .into_iter()
        .collect();
    all_dates.sort();
    all_dates.reverse();

    println!("{:<12} {:<12} {:<14} {}", "Date", "File Events", "Git Commits", "Verdict");
    println!("{}", "-".repeat(60));

    for date in &all_dates {
        let events = file_events.get(date).copied().unwrap_or(0);
        let commits = git_commits.get(date).copied().unwrap_or(0);
        let verdict = if events > 10 && commits == 0 {
            "You worked but did not commit!"
        } else if events > 0 && commits > 0 {
            "Good — active and committed"
        } else if events == 0 && commits == 0 {
            "Rest day"
        } else {
            "Low activity"
        };
        println!("{:<12} {:<12} {:<14} {}", date, events, commits, verdict);
    }
}
