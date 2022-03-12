pub mod event;

use core::fmt;
use event::Event;
use once_cell::sync::Lazy;
use std::collections::LinkedList;

static mut SCHEDULER: Lazy<Scheduler> = Lazy::new(|| new());

/// Schedule emulation events on instruction level rather than on clock level.
pub struct Scheduler {
    pub master_cycles: i64,
    pub relative_cycles: i64,
    pub root: LinkedList<*mut Event>,
    pub next_event: i64,
}

pub fn new() -> Scheduler {
    Scheduler {
        master_cycles: 0,
        relative_cycles: 0,
        root: LinkedList::new(),
        next_event: 0,
    }
}

pub fn get() -> &'static Scheduler {
    return unsafe { &SCHEDULER };
}

pub fn get_mut() -> &'static mut Scheduler {
    return unsafe { &mut SCHEDULER };
}

/// Returns the clock cycle since the emulator was powered-up.
pub fn cycles() -> i64 {
    unsafe {
        return SCHEDULER.master_cycles + SCHEDULER.relative_cycles;
    }
}

pub fn add(c: i64) -> i64 {
    unsafe {
        SCHEDULER.master_cycles += c;
        let master_cycles = SCHEDULER.master_cycles;

        let mut idx = 0;
        for event in &SCHEDULER.root {
            let e = &mut (*(*event));
            let next_when = e.when - master_cycles;
            if next_when > 0 {
                for _ in 0..idx {
                    SCHEDULER.root.pop_front();
                }
                return next_when;
            }
            idx += 1;
            (e.callback)(-next_when);
        }

        for _ in 0..idx {
            SCHEDULER.root.pop_front();
        }

        return SCHEDULER.next_event;
    }
}

pub fn schedule(e: &mut Event, after: i64) {
    unsafe {
        let after = after + SCHEDULER.relative_cycles;
        e.when = after + SCHEDULER.master_cycles;
        if after < SCHEDULER.next_event {
            SCHEDULER.next_event = after;
        }

        let mut idx = 0;
        let prio = e.prio;
        for e in &SCHEDULER.root {
            let next_when = (*(*e)).when - SCHEDULER.master_cycles;
            if next_when > after || (next_when == after && (*(*e)).prio > prio) {
                break;
            }
            idx += 1;
        }

        let mut split = SCHEDULER.root.split_off(idx);
        SCHEDULER.root.push_back(e);
        SCHEDULER.root.append(&mut split);
    }
}

pub fn reschedule(e: &mut Event, after: i64) {
    deschedule(e);
    schedule(e, after);
}

pub fn schedule_abs(e: &mut Event, when: i64) {
    schedule(e, when - cycles());
}

pub fn deschedule(e: &mut Event) {
    unsafe {
        let event = e as *mut Event;
        let mut idx = 0;

        for e in &SCHEDULER.root {
            if e.eq(&event) {
                break;
            }
            idx += 1;
        }

        let mut split = SCHEDULER.root.split_off(idx);
        split.pop_front();
        SCHEDULER.root.append(&mut split);
    }
}

pub fn any_event() -> bool {
    unsafe {
        return SCHEDULER.relative_cycles > SCHEDULER.next_event;
    }
}

impl fmt::Display for Scheduler {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = "Scheduler: ".to_string();
        for e in &self.root {
            let e = unsafe { &*(*e) };
            s += &format!("{}", e);
            s += " -> ";
        }
        write!(f, "{}", s)
    }
}
