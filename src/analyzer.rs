use git2::Repository;
use chrono::{DateTime, Local, Datelike, Timelike};
use std::collections::HashMap;

pub fn scan(path: &str) {
    let repo = match Repository::open(path) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Error: not a git repository: {}", path);
            return;
        }
    };

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let mut commits_per_hour: HashMap<u32, u32> = HashMap::new();
    let mut total_commits = 0;
    let mut peak_hour = 0;
    let mut peak_count = 0;

    for oid in revwalk.flatten() {
        let commit = repo.find_commit(oid).unwrap();
        let time = commit.time().seconds();
        let dt: DateTime<Local> = DateTime::from_timestamp(time, 0)
            .unwrap()
            .with_timezone(&Local);

        let hour = dt.hour();
        let count = commits_per_hour.entry(hour).or_insert(0);
        *count += 1;
        total_commits += 1;

        if *count > peak_count {
            peak_count = *count;
            peak_hour = hour;
        }
    }

    println!("=== GitGrade Scan: {} ===", path);
    println!("Total commits:   {}", total_commits);
    println!("Peak coding hour: {:02}:00 - {:02}:00", peak_hour, peak_hour + 1);
    println!("Commits at peak hour: {}", peak_count);
}

pub fn streak(path: &str) {
    let repo = match Repository::open(path) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Error: not a git repository: {}", path);
            return;
        }
    };

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let mut days: Vec<chrono::NaiveDate> = vec![];

    for oid in revwalk.flatten() {
        let commit = repo.find_commit(oid).unwrap();
        let time = commit.time().seconds();
        let dt: DateTime<Local> = DateTime::from_timestamp(time, 0)
            .unwrap()
            .with_timezone(&Local);
        let date = dt.date_naive();
        if !days.contains(&date) {
            days.push(date);
        }
    }

    days.sort();
    days.reverse();

    let mut streak = 0;
    let mut prev = Local::now().date_naive();

    for day in &days {
        let diff = (prev - *day).num_days();
        if diff <= 1 {
            streak += 1;
            prev = *day;
        } else {
            break;
        }
    }

    println!("=== GitGrade Streak: {} ===", path);
    println!("Current coding streak: {} day(s) 🔥", streak);
}

pub fn weekly(path: &str) {
    let repo = match Repository::open(path) {
        Ok(r) => r,
        Err(_) => {
            eprintln!("Error: not a git repository: {}", path);
            return;
        }
    };

    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();

    let mut commits_per_day: HashMap<String, u32> = HashMap::new();
    let now = Local::now();

    for oid in revwalk.flatten() {
        let commit = repo.find_commit(oid).unwrap();
        let time = commit.time().seconds();
        let dt: DateTime<Local> = DateTime::from_timestamp(time, 0)
            .unwrap()
            .with_timezone(&Local);

        let days_ago = (now.date_naive() - dt.date_naive()).num_days();
        if days_ago <= 7 {
            let label = dt.format("%a %d/%m").to_string();
            *commits_per_day.entry(label).or_insert(0) += 1;
        }
    }

    println!("=== GitGrade Weekly: {} ===", path);
    if commits_per_day.is_empty() {
        println!("No commits in the last 7 days.");
    } else {
        let mut days: Vec<(String, u32)> = commits_per_day.into_iter().collect();
        days.sort_by(|a, b| a.0.cmp(&b.0));
        for (day, count) in days {
            let bar = "█".repeat(count as usize);
            println!("{:12} | {} ({})", day, bar, count);
        }
    }
}
