use crate::WhatAppChat;
use log::{error, info};
use polars::prelude::*;

pub struct ChatAnalyzer {
    chat: Option<WhatAppChat>,
    pub df: Option<DataFrame>,
    pub paricipants_df: Vec<DataFrame>,
}

impl ChatAnalyzer {
    pub fn new() -> Self {
        Self {
            chat: None,
            df: None,
            paricipants_df: vec![],
        }
    }

    pub fn chat(mut self, chat: WhatAppChat) -> Self {
        self.chat = Some(chat);
        self
    }

    pub fn build_dfs(&mut self) {
        match &mut self.chat {
            Some(chat) => {
                self.df = match df!(
                    "datetime" => chat.get_datetime(),
                    "sender" => chat.get_senders(),
                    "content" => chat.get_contents()
                ) {
                    Ok(df) => Some(df),
                    Err(_) => {
                        error!("[ChatAnalyzer]: Failed to build dfs from chat");
                        None
                    }
                };
            }
            None => info!("[ChatAnalyzer]: No chat present"),
        }
    }
}
