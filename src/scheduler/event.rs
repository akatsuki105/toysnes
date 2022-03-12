use core::fmt;

fn _default_callback(_cycles_late: i64) {}

pub struct Event {
    pub name: String,
    pub when: i64,
    pub prio: usize,
    pub callback: fn(cycles_late: i64),
}

impl Event {
    pub fn new(name: String, prio: usize, cb: fn(cycles_late: i64)) -> Self {
        Self {
            name: name.to_string(),
            when: 0,
            prio: prio,
            callback: cb,
        }
    }
}

impl Default for Event {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            when: 0,
            prio: 0,
            callback: _default_callback,
        }
    }
}

impl fmt::Display for Event {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}:{}", self.name, self.when)
    }
}
