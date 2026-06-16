# GitGrade

> Automatic coding habit tracker for 1st and 2nd semester Health Informatics students.

## The Problem

Most beginner programming students don't know if they are practicing enough.
They commit everything the night before a deadline and wonder why they are not improving.
GitGrade reads your git history automatically and tells you the truth about your coding habits.

## What GitGrade Does

GitGrade scans your local git repository and shows you:

- How many days you coded this week
- Whether you are a consistent learner or a deadline crammer
- Your most active coding hour and what it means for your focus
- Your progress since your first commit
- Beginner milestones to keep you motivated

No manual input. No forms. Just run it and see your habits.

## Installation

Download the binary from the website or build from source:

```bash
cargo build --release
```

## Usage

```bash
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
```

## Example Output

=== GitGrade Habits ===

Days coded this week : 5
Verdict : Great job! You coded 5 days this week.

=== GitGrade Patterns ===

Most active hour : 21:00
Verdict : You code mostly at night. Try morning sessions for better focus.

=== GitGrade Milestones ===

Total commits : 10
Active coding days : 8
Current streak : 3 day(s)
Milestone : 10 commits total - keep going!


## Business Model

**Free**
- All 5 commands
- Habit tracking
- Milestone detection
- Coding pattern analysis
- Progress since week 1

**Pro - 2.99 EUR/month**
- Weekly PDF report
- Semester progress export
- Multi-repo scanning
- Share your stats with your professor
- Email reminders to code today

## Who Is This For

GitGrade is built specifically for 1st and 2nd semester Health Informatics students
who are learning to program and want to build consistent daily coding habits.

## Built With

- Rust
- clap - CLI argument parsing
- chrono - date and time handling
- git2 - git repository access

## License

MIT
