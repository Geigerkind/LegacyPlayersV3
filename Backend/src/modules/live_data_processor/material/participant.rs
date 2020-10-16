#[derive(Debug, Clone)]
pub struct Participant {
    pub id: u64,
    pub is_player: bool,
    pub name: String,
    pub hero_class_id: Option<u8>,
    pub server: Option<(u32, String)>,
    pub gear_setups: Option<Vec<(u64, Vec<Option<(u32, Option<u32>)>>)>>,
    pub active_intervals: Vec<(u64, u64)>,

    // Technical
    last_seen: u64
}

impl Participant {
    pub fn new(id: u64, is_player: bool, name: String, last_seen: u64) -> Self {
        Participant {
            id,
            is_player,
            hero_class_id: None,
            name,
            server: None,
            gear_setups: None,
            active_intervals: vec![(last_seen, last_seen)],
            last_seen
        }
    }

    // Assumes that now > last_seen
    pub fn add_participation_point(&mut self, now: u64) {
        static PARTICIPATION_TIMEOUT: u64 = 5 * 60000;
        if now - self.last_seen <= PARTICIPATION_TIMEOUT {
            self.active_intervals.last_mut().unwrap().1 = now;
        } else {
            self.active_intervals.push((now, now));
        }
        self.last_seen = now;
    }
}