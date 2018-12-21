#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::cmp::Ordering;

use regex::Regex;
use regex::RegexSet;

#[derive(PartialEq, Eq, Debug)]
enum EventType {
    Awake,
    Asleep,
    Change(u32),
}

#[derive(PartialEq, Eq, Debug)]
struct Event {
    which: EventType,
    year: u32,
    month: u32,
    day: u32,
    hour: u32,
    minute: u32,
}

impl Event {
    fn total_minutes(&self) -> u32 {
        const DAYS_IN_MONTH: u32 = 30;
        const MONTHS_IN_YEAR: u32 = 12;
        const HOURS_IN_DAY: u32 = 24;
        const MINUTES_IN_HOUR: u32 = 60;
        self.year * MONTHS_IN_YEAR * DAYS_IN_MONTH * HOURS_IN_DAY * MINUTES_IN_HOUR
            + self.month * DAYS_IN_MONTH * HOURS_IN_DAY * MINUTES_IN_HOUR
            + self.day * HOURS_IN_DAY * MINUTES_IN_HOUR
            + self.hour * MINUTES_IN_HOUR
            + self.minute
    }
}

impl Ord for Event {
    fn cmp(&self, other: &Event) -> Ordering {
        if self.year != other.year {
            return self.year.cmp(&other.year);
        }
        if self.month != other.month {
            return self.month.cmp(&other.month);
        }
        if self.day != other.day {
            return self.day.cmp(&other.day);
        }
        if self.hour != other.hour {
            return self.hour.cmp(&other.hour);
        }
        return self.minute.cmp(&other.minute);
    }
}

impl PartialOrd for Event {
    fn partial_cmp(&self, other: &Event) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let sorted_event_list: Vec<Event> = parse_input();
    let mut current_guard = None;
    let mut last_sleep_time = None;
    let mut sleep_times = HashMap::new();
    for event in &sorted_event_list {
        match event.which {
            EventType::Awake => {
                if let Some(guard) = current_guard {
                    let current_count = sleep_times.entry(guard).or_insert(0);
                    *current_count += event.minute - last_sleep_time.unwrap();
                    last_sleep_time = None;
                } else {
                    panic!("We got a bad ordering! A guard woke up before a guard was stationed!");
                }
            }
            EventType::Asleep => last_sleep_time = Some(event.minute),
            EventType::Change(guard_number) => current_guard = Some(guard_number),
        }
    }
    let (most_likely_guard_number, _): (&u32, _) =
        sleep_times.iter().max_by_key(|(_, &b)| b).unwrap();
    let most_likely_guard_number = *most_likely_guard_number;

    let mut sleepy_minutes = HashMap::new();
    let sleepy_guard_start_indices = (1..)
        .zip(sorted_event_list.iter())
        .filter(|(_, e)| match e.which {
            EventType::Change(id) if id == most_likely_guard_number => true,
            _ => false,
        })
        .map(|(idx, _)| idx);

    let sleepy_guard_days = sleepy_guard_start_indices.map(|idx| {
        sorted_event_list[idx..sorted_event_list.len()]
            .iter()
            .take_while(|e| match e.which {
                EventType::Asleep | EventType::Awake => true,
                _ => false,
            })
    });
    let sleepy_guard_actions: Vec<_> = sleepy_guard_days.flatten().collect();
    assert!(sleepy_guard_actions.len() % 2 == 0);
    let pairs = (0..sleepy_guard_actions.len() / 2)
        .map(|i| 2 * i)
        .map(|i| (sleepy_guard_actions[i], sleepy_guard_actions[i + 1]))
        .inspect(|(f, s)| {
            if let (EventType::Asleep, EventType::Awake) = (&f.which, &s.which) {
                assert!(f < s);
            } else {
                panic!("Got a bad pair! {:?}", (f, s));
            }
        });

    for (f, s) in pairs {
        let in_mins = |e: &Event| e.hour * 60 + e.minute;
        let rng = in_mins(f)..in_mins(s);
        for i in rng {
            let ent = sleepy_minutes.entry(i).or_insert(0);
            *ent += 1;
        }
    }
    let (most_likely_minute, _): (&u32, _) = sleepy_minutes
        .iter()
        .max_by_key(|(_, &k)| k)
        .unwrap();
    println!("{}", most_likely_minute * most_likely_guard_number);
}

fn parse_input() -> Vec<Event> {
    const INPUT_FILE: &str = "input.txt";
    let l = BufReader::new(File::open(INPUT_FILE).unwrap()).lines();

    let parse_message = |s: &str| {
        lazy_static! {
            static ref set: RegexSet = RegexSet::new(&[r"falls asleep", r"wakes up"]).unwrap();
            static ref guard_start: Regex =
                Regex::new(r"Guard #(?P<number>\d{1,4}) begins").unwrap();
        }
        let mtch = set.matches(s);
        if mtch.matched(0) {
            EventType::Asleep
        } else if mtch.matched(1) {
            EventType::Awake
        } else {
            if let Some(mtch) = guard_start.captures(s) {
                let number = mtch.name("number").unwrap().as_str().parse().unwrap();
                EventType::Change(number)
            } else {
                panic!("Couldn't parse message {}", s);
            }
        }
    };

    let mapper = |s: &str| {
        lazy_static! {
            static ref re: Regex = Regex::new(r"^\[(?P<year>\d{4})-(?P<month>\d{2})-(?P<day>\d{2}) (?P<hour>\d{2}):(?P<minute>\d{2})\] (?P<message>.*)$").unwrap();
        }
        if let Some(matc) = re.captures(s) {
            let year = matc.name("year").unwrap().as_str().parse().unwrap();
            let month = matc.name("month").unwrap().as_str().parse().unwrap();
            let day = matc.name("day").unwrap().as_str().parse().unwrap();
            let hour = matc.name("hour").unwrap().as_str().parse().unwrap();
            let minute = matc.name("minute").unwrap().as_str().parse().unwrap();
            let which = parse_message(matc.name("message").unwrap().as_str());
            Event {
                which: which,
                year: year,
                month: month,
                day: day,
                hour: hour,
                minute: minute,
            }
        } else {
            panic!("A line didn't match! It was {}", s);
        }
    };
    let mut out: Vec<_> = l.map(|r| mapper(&r.unwrap())).collect();
    out.sort_unstable();
    out
}
