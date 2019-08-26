#[macro_use]
extern crate lazy_static;

extern crate regex;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

use std::cmp::Ordering;

use regex::Regex;
use regex::RegexSet;

type Time = i32;
type GuardId = u32;

#[derive(PartialEq, Eq, Debug, Clone)]
enum EventType {
    Awake,
    Asleep,
    Change(GuardId),
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Sleep {
    start_time: Time,
    end_time: Time,
    guard_id: GuardId,
}

#[derive(PartialEq, Eq, Debug, Clone)]
struct Event {
    which: EventType,
    year: Time,
    month: Time,
    day: Time,
    hour: Time,
    minute: Time,
}

#[derive(PartialEq, Eq, Debug)]
struct GuardEvent {
    event: Event,
    guard_id: GuardId,
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
    let annotated_events = annotate_events(&sorted_event_list);
    let sleeps = pair_up(&annotated_events);
    let mut sleep_times: HashMap<GuardId, u32> = HashMap::new();
    for sleep in &sleeps {
        let current_count = sleep_times.entry(sleep.guard_id).or_insert(0);
        assert!(sleep.end_time > sleep.start_time);
        *current_count += (sleep.end_time - sleep.start_time) as u32;
    }
    let (most_likely_guard_number, _): (&u32, _) =
        sleep_times.iter().max_by_key(|(_, &b)| b).unwrap();
    let most_likely_guard_number = *most_likely_guard_number;
    let our_guard_actions: Vec<_> = sleeps
        .iter()
        .filter(|s| s.guard_id == most_likely_guard_number)
        .map(|s| s.clone())
        .collect();
    let _our_guard_most_likely_minute = most_common_minute(&our_guard_actions);

    let mut guards_by_minutes: HashMap<(GuardId, Time), u32> = HashMap::new();
    for s in &sleeps {
        for min in s.start_time..s.end_time {
            let entry = guards_by_minutes.entry((s.guard_id, min)).or_insert(0);
            *entry += 1;
        }
    }
    if let Some(((id, minute), count)) = guards_by_minutes
        .iter()
        .inspect(|p| println!("{:?}", p))
        .max_by_key(|(_, &c)| c)
    {
        println!("id is {}, time is {}, count is {}", id, minute, count);
        println!("{}", (*id) as Time * *minute);
    } else {
        panic!("Apparently something was empty. Hmm.");
    }
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

fn most_common_minute(events: &[Sleep]) -> Time {
    let mut sleepy_minutes = HashMap::new();

    for sleep in events {
        for i in sleep.start_time..=sleep.end_time {
            let ent = sleepy_minutes.entry(i).or_insert(0);
            *ent += 1;
        }
    }
    let (most_likely_minute, _): (&Time, _) =
        sleepy_minutes.iter().max_by_key(|(_, &k)| k).unwrap();
    *most_likely_minute
}

fn annotate_events(events: &[Event]) -> Vec<GuardEvent> {
    let mut out = Vec::new();
    let mut current_guard_number = None;
    for e in events {
        match e.which {
            EventType::Change(id) => current_guard_number = Some(id),
            _ => out.push(GuardEvent {
                event: e.clone(),
                guard_id: current_guard_number.unwrap(),
            }),
        }
    }
    out
}

fn pair_up(events: &[GuardEvent]) -> Vec<Sleep> {
    let clamp = |h, m| {
        let n = 60 * h + m;
        if n > 12 * 60 {
            24 * 60 - n
        } else {
            n
        }
    };
    assert!(events.len() % 2 == 0);
    let num_wakes = events
        .iter()
        .filter(|e| {
            if let EventType::Awake = e.event.which {
                true
            } else {
                false
            }
        })
        .count();
    let num_sleeps = events
        .iter()
        .filter(|e| {
            if let EventType::Asleep = e.event.which {
                true
            } else {
                false
            }
        })
        .count();
    assert!(num_wakes == num_sleeps);
    (0..events.len() / 2)
        .map(|i| 2 * i)
        .map(|i| (&events[i], &events[i + 1]))
        .inspect(|(f, s)| assert!(f.guard_id == s.guard_id && f.event < s.event))
        .map(|(f, s)| Sleep {
            start_time: clamp(f.event.hour, f.event.minute),
            end_time: clamp(s.event.hour, s.event.minute),
            guard_id: f.guard_id,
        })
        .collect()
}
