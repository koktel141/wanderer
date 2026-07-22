use crate::constants::QUEST_TARGET_KILLS;

#[derive(PartialEq, Clone, Copy)]
pub enum QuestStatus {
    NotStarted,
    Active,
    Completed,
}

pub struct Quest {
    pub status: QuestStatus,
    pub kills: u32,
}

impl Quest {
    pub fn new() -> Self {
        Self {
            status: QuestStatus::NotStarted,
            kills: 0,
        }
    }

    pub fn start(&mut self) {
        if self.status == QuestStatus::NotStarted {
            self.status = QuestStatus::Active;
        }
    }

    pub fn is_active(&self) -> bool {
        self.status == QuestStatus::Active
    }

    pub fn register_kill(&mut self) -> bool {
        if self.status != QuestStatus::Active {
            return false;
        }
        self.kills += 1;
        if self.kills >= QUEST_TARGET_KILLS {
            self.status = QuestStatus::Completed;
            return true;
        }
        false
    }

    pub fn description(&self) -> String {
        match self.status {
            QuestStatus::NotStarted => "Talk to the elder to begin".to_string(),
            QuestStatus::Active => format!(
                "Quest: Defeat wolves ({}/{})",
                self.kills, QUEST_TARGET_KILLS
            ),
            QuestStatus::Completed => "Quest Complete: Wolf Hunter!".to_string(),
        }
    }
}
