use std::collections::HashMap;

use mio::{Poll, Token};

// String is a placeholder for a service name for now. It might stay so forever.
#[derive(Debug)]
pub struct Task(String);

#[derive(Debug)]
pub struct ScheduleManager<'a> {
    // Both will last the lifetime of this applciation, the ref allows it to be 
    // used elsewhere for signal handling.
    poll: &'a Poll,
    // Can we establish a way to make the Token an index in a Vec to reduce 
    // compute overhead? A Vec which holds empty tasks until scheduled?!?!?!
    // Does this need to wrap the timer? TimerFd will simply be contained in the 
    // token, so the Task should have a TimerFd wrapper?
    timers: HashMap<Token, Task>,
    next_token: usize
}

impl<'a> ScheduleManager<'a> {
    pub fn new(poll: &'a Poll) -> ScheduleManager<'a> {
        ScheduleManager {
            poll,
            timers: HashMap::new(),
            next_token: 1 // Token 0 is already being used as the signal token.
        }
    }

    // This should take the deadline, update the timer in the Task and register 
    // it with Poll.
    pub fn schedule_task(task: &mut Task) -> Token {
        todo!()
    }
}