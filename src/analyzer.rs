use git2::Repository;
use chrono::{DateTime, Local, Timelike};
use std::collections::HashMap;

fn open_repo(path: &str) -> Option<Repository> {
    match Repository::open(path) {
        Ok(r) => Some(r),
        Err(_) => {
            eprintln!("Error: no git repository found in '{}'.", path);
            eprintln!("Tip: run 'git init' first, or point gitgrade at a folder with a .git directory.");
            None
        }
    }
}

fn get_commits(repo: &Repository) -> Vec<DateTime<Local>> {
    let mut revwalk = repo.revwalk().unwrap();
    revwalk.push_head().unwrap();
    let mut dates = vec![];
    for oid in revwalk.flatten() {
        if let Ok(commit) = repo.find_commit(oid) {
            let time = commit.time().seconds();
            if let Some(dt) = DateTime::from_timestamp(time, 0) {
                dates.push(dt.with_timezone(&Local));
            }
        }
    }
    dates
}

pub fn scan(path: &str) {
    let repo = match open_repo(path) { Some(r) => r, None => return };
    let commits = get_commits(&repo);

    let total = commits.len();
    let mut hours: HashMap<u32, u32> = HashMap::new();
    for dt in &commits {
        *hours.entry(dt.hour()).or_insert(0) += 1;
    }
    let peak_hour = hours.iter().max_by_key(|e| e.1).map(|(h, _)| *h).unwrap_or(0);

    println!("=== GitGrade Scan: {} ===", path);
    println!("Total commits    : {}", total);
    println!("Peak coding hour : {:02}:00 - {:02}:00", peak_hour, peak_hour + 1);

    if total >= 10 {
        println!("Milestone        : {} commits total - keep going!", total);
    } else {
        println!("Milestone        : {} commits so far - {} more to reach 10!", total, 10 - total);
    }
}

pub fn habits(path: &str) {
    let repo = match open_repo(path) { Some(r) => r, None => return };
    let commits = get_commits(&repo);

    let now = Local::now();
    let mut days_this_week = std::collections::HashSet::new();
    let mut last_day = None;
    let mut crammer = false;

    for dt in &commits {
        let days_ago = (now.date_naive() - dt.date_naive()).num_days();
        if days_ago <= 7 {
            days_this_week.insert(dt.date_naive());
        }
        if days_ago <= 1 {
            if let Some(prev) = last_day {
                if prev == dt.date_naive() {
                    crammer = true;
                }
            }
            last_day = Some(dt.date_naive());
        }
    }

    let days_count = days_this_week.len();

    println!("=== GitGrade Habits ===");
    println!("Days coded this week : {}", days_count);

    if days_count >= 5 {
        println!("Verdict              : Great job! You coded {} days this week.", days_count);
    } else if days_count >= 3 {
        println!("Verdict              : Good effort! Try to code at least 5 days a week.");
    } else if days_count == 1 && crammer {
        println!("Verdict              : Warning: You only committed on deadline day. Try spreading your work.");
    } else {
        println!("Verdict              : Only {} day(s) this week. Consistency is key for beginners!", days_count);
    }
}

pub fn progress(path: &str) {
    let repo = match open_repo(path) { Some(r) => r, None => return };
    let commits = get_commits(&repo);

    if commits.is_empty() {
        println!("No commits found.");
        return;
    }

    let now = Local::now();
    let mut week1: u32 = 0;
    let mut this_week: u32 = 0;
    let oldest = commits.iter().min_by_key(|d| d.timestamp()).unwrap();
    let first_week_end = *oldest + chrono::Duration::days(7);

    for dt in &commits {
        let days_ago = (now.date_naive() - dt.date_naive()).num_days();
        if days_ago <= 7 {
            this_week += 1;
        }
        if *dt <= first_week_end {
            week1 += 1;
        }
    }

    println!("=== GitGrade Progress ===");
    println!("First week commits   : {}", week1);
    println!("This week commits    : {}", this_week);

    if week1 > 0 && this_week > week1 {
        let growth = this_week / week1;
        if growth >= 2 {
            println!("Verdict              : Your commits grew {}x since week 1. Keep it up!", growth);
        } else {
            println!("Verdict              : You are committing more than week 1. Good progress!");
        }
    } else if week1 == 0 {
        println!("Verdict              : Not enough history yet. Keep coding!");
    } else {
        println!("Verdict              : Try to commit more than your first week!");
    }
}

pub fn patterns(path: &str) {
    let repo = match open_repo(path) { Some(r) => r, None => return };
    let commits = get_commits(&repo);

    let mut hours: HashMap<u32, u32> = HashMap::new();
    for dt in &commits {
        *hours.entry(dt.hour()).or_insert(0) += 1;
    }

    let peak_hour = hours.iter().max_by_key(|e| e.1).map(|(h, _)| *h).unwrap_or(0);

    println!("=== GitGrade Patterns ===");
    println!("Most active hour     : {:02}:00", peak_hour);

    if peak_hour >= 22 || peak_hour < 4 {
        println!("Verdict              : You code mostly at night. Try morning sessions for better focus.");
    } else if peak_hour >= 6 && peak_hour <= 12 {
        println!("Verdict              : Great! Morning coding builds strong habits.");
    } else if peak_hour >= 13 && peak_hour <= 17 {
        println!("Verdict              : Afternoon coder - solid and consistent.");
    } else {
        println!("Verdict              : Evening coder - just make sure you get enough sleep!");
    }
}

pub fn milestones(path: &str) {
    let repo = match open_repo(path) { Some(r) => r, None => return };
    let commits = get_commits(&repo);

    let total = commits.len();
    let now = Local::now();
    let mut days_active = std::collections::HashSet::new();
    for dt in &commits {
        days_active.insert(dt.date_naive());
    }

    let mut streak = 0;
    let mut check_day = now.date_naive();
    loop {
        if days_active.contains(&check_day) {
            streak += 1;
            check_day -= chrono::Duration::days(1);
        } else {
            break;
        }
    }

    println!("=== GitGrade Milestones ===");
    println!("Total commits        : {}", total);
    println!("Active coding days   : {}", days_active.len());
    println!("Current streak       : {} day(s)", streak);

    if total >= 100 {
        println!("Milestone            : 100 commits - you are no longer a beginner!");
    } else if total >= 50 {
        println!("Milestone            : 50 commits - halfway to 100. Impressive!");
    } else if total >= 10 {
        println!("Milestone            : 10 commits total - keep going!");
    } else {
        println!("Milestone            : {} commits so far - {} more to reach your first milestone!", total, 10 - total);
    }

    if streak >= 7 {
        println!("Streak bonus         : 7 day streak - outstanding consistency!");
    } else if streak >= 3 {
        println!("Streak bonus         : {} day streak - you are building a great habit!", streak);
    }
}

pub fn badge(path: &str) {
    let repo = match open_repo(path) { Some(r) => r, None => return };
    let commits = get_commits(&repo);

    let total = commits.len();
    let now = Local::now();

    let mut days_active = std::collections::HashSet::new();
    for dt in &commits {
        days_active.insert(dt.date_naive());
    }
    let mut streak = 0;
    let mut check_day = now.date_naive();
    loop {
        if days_active.contains(&check_day) {
            streak += 1;
            check_day -= chrono::Duration::days(1);
        } else {
            break;
        }
    }

    let mut hours: std::collections::HashMap<u32, u32> = std::collections::HashMap::new();
    for dt in &commits {
        *hours.entry(dt.hour()).or_insert(0) += 1;
    }
    let peak_hour = hours.iter().max_by_key(|e| e.1).map(|(h, _)| *h).unwrap_or(0);
    let time_label = if peak_hour >= 22 || peak_hour < 4 {
        "night+coder"
    } else if peak_hour >= 6 && peak_hour <= 12 {
        "morning+coder"
    } else if peak_hour >= 13 && peak_hour <= 17 {
        "afternoon+coder"
    } else {
        "evening+coder"
    };

    let streak_color = if streak >= 7 { "00d4aa" } else if streak >= 3 { "6366f1" } else { "f59e0b" };
    let commit_color = if total >= 50 { "00d4aa" } else if total >= 10 { "6366f1" } else { "f59e0b" };

    println!("=== GitGrade Badge ===");
    println!("Copy this into your GitHub profile README:\n");
    println!("![GitGrade Streak](https://img.shields.io/badge/streak-{}%20days-{}?style=flat&logo=git&logoColor=white)", streak, streak_color);
    println!("![GitGrade Commits](https://img.shields.io/badge/commits-{}-{}?style=flat&logo=git&logoColor=white)", total, commit_color);
    println!("![GitGrade Style](https://img.shields.io/badge/style-{}-00d4aa?style=flat&logo=git&logoColor=white)", time_label);
    println!("\nPaste all three lines into your GitHub profile README.md");
}
