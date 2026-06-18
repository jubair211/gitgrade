# GitGrade

Automatic coding habit tracker for 1st and 2nd semester Health Informatics students.

## The Problem

Most beginner programming students do not know if they are practicing enough.
They commit everything the night before a deadline and wonder why they are not improving.
Worse — they work for hours without committing, and their git history does not show the real effort.

GitGrade reads your git history AND watches your file activity to show the full picture.

## What GitGrade Does

GitGrade gives you two layers of insight:

**Layer 1 — Git History Analysis (automatic)**
- How many days you coded this week
- Whether you are a consistent learner or a deadline crammer
- Your most active coding hour
- Your progress since your first commit
- Beginner milestones and streaks

**Layer 2 — File Activity Tracking (automatic)**
- Watches your project folder for file open, modify and create events
- Logs all activity with timestamps
- Compares file activity with git commits
- Shows days where you worked hard but forgot to commit

## Installation

Download the binary from the website or build from source:

    cargo build --release

## Usage

    # Full activity scan
    gitgrade scan .

    # Check your weekly coding habits
    gitgrade habits .

    # See your progress since week 1
    gitgrade progress .

    # See when you code and what it means
    gitgrade patterns .

    # Check your beginner milestones
    gitgrade milestones .

    # Watch a folder and log file activity
    gitgrade watch .

    # Compare file activity with git commits
    gitgrade compare .

## Example Output

    === GitGrade Activity Compare ===
    Date         File Events  Git Commits    Verdict
    ------------------------------------------------------------
    2026-06-18   47           1              You worked but did not commit!
    2026-06-17   12           3              Good - active and committed
    2026-06-16   0            0              Rest day

## Business Model

Free
- All 7 commands
- Git history analysis
- File activity tracking
- Milestone detection

Pro - 2.99 EUR lifetime
- Everything in Free
- Weekly PDF report
- Semester progress export
- Multi-repo scanning
- Email reminders to commit today
- Vim undo history tracking
- VS Code plugin

## Who Is This For

GitGrade is built for 1st and 2nd semester Health Informatics students
who are learning to program and want to build consistent daily coding habits.

## Built With

- Rust
- clap - CLI argument parsing
- chrono - date and time handling
- git2 - git repository access
- notify - file system event watching

## License

MIT
