use core::{ops::Add, fmt::Display};

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref SYSTEM_CLOCK: Mutex<SystemClock> = Mutex::new(SystemClock::new(10.0));
}


#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Instant(usize);

impl Instant {
    pub fn now() -> Self {
        SYSTEM_CLOCK.lock().now()
    }

    pub fn zero() -> Self {
        Self(0)
    }

    pub fn checked_add(&self, rhs: usize) -> Option<Self> {
        self.0.checked_add(rhs).map(|i| Instant(i))
    }

    pub fn seconds(&self) -> usize {
        let tick = self.0;
        let seconds = tick as f32 / SYSTEM_CLOCK.lock().rate;
        seconds as usize
    }

    pub fn minutes(&self) -> (usize, usize) {
        let seconds = self.seconds();
        let minutes = seconds / 60;
        (seconds - (60 * minutes), minutes)
    }

    pub fn hours(&self) -> (usize, usize, usize) {
        let (seconds, minutes) = self.minutes();
        let hours = minutes / 60;
        (seconds, minutes - (60 * hours), hours)
    }
}

impl Add<usize> for Instant {
    type Output = Instant;

    fn add(self, rhs: usize) -> Self::Output {
        Instant(self.0 + rhs)
    }
}

impl Display for Instant {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        Display::fmt(&self.0, f)
    }
}

pub struct SystemClock {
    tick: Instant,
    loops: usize,
    rate: f32,
}

impl SystemClock {
    pub fn new(rate: f32) -> Self {
        Self {
            tick: Instant::zero(),
            loops: 0,
            rate,
        }
    }

    pub fn tick(&mut self) {
        use core::intrinsics::likely;
        let now = self.tick.checked_add(1);
        
        if likely(now.is_some()) {
            let now = now.unwrap();
            self.tick = now;
        } else {
            self.tick = Instant::zero();
            self.loops += 1;
        }
    }

    pub fn now(&self) -> Instant {
        self.tick
    }

    pub fn total(&self) -> f32 {
        use core::intrinsics::likely;

        if likely(self.loops < 1) {
            self.tick.0 as f32
        } else {
            self.tick.0 as f32 * self.loops as f32
        }
    }

    pub fn seconds(&self) -> usize {
        let tick = self.total();
        let seconds = tick / self.rate;
        seconds as usize
    }

    pub fn minutes(&self) -> (usize, usize) {
        let seconds = self.seconds();
        let minutes = seconds / 60;
        (seconds - (60 * minutes), minutes)
    }

    pub fn hours(&self) -> (usize, usize, usize) {
        let (seconds, minutes) = self.minutes();
        let hours = minutes / 60;
        (seconds, minutes - (60 * hours), hours)
    }
}
