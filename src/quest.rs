use crate::constants::QUEST_TARGET_KILLS;

pub struct Quest {
    pub kills: u32,
    pub completed: bool,
}

impl Quest {
    pub fn new() -> Self {
        Self { kills: 0, completed: false }
    }

    pub fn register_kill(&mut self) -> bool {
        if self.completed {
            return false;
        }
        self.kills += 1;
        if self.kills >= QUEST_TARGET_KILLS {
            self.completed = true;
            return true; // همین الان کامل شد
        }
        false
    }

    pub fn description(&self) -> String {
        if self.completed {
            "Quest Complete: Wolf Hunter!".to_string()
        } else {
            format!("Quest: Defeat wolves ({}/{})", self.kills, QUEST_TARGET_KILLS)
        }
    }
}