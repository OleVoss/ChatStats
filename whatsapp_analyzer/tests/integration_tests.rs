use polars::{
    lazy::{dsl::col, frame::IntoLazy},
    prelude::*,
};
use whatsapp_analyzer::analyzer;

mod common;

#[test]
fn impots_chat_check_len() {
    let mut chat = common::setup_emtpy_chat();
    chat.parse_messages(common::TEST_CHAT);
    assert_eq!(chat.all_messages.len(), 13);
}

#[test]
fn builds_df_from_chat() {
    let mut analyzer = common::setup_analyzer_with_chat();
    analyzer.build_dfs();
    assert_eq!(analyzer.df.unwrap().shape(), (13, 3))
}

#[test]
fn df_analysis() {
    let mut analyzer = common::setup_analyzer_with_dfs();
    match analyzer.df {
        None => (),
        Some(df) => {
            println!(
                "Alicia: {:?}",
                df.clone()
                    .lazy()
                    .filter(col("sender").eq(lit("Alicia")))
                    .collect()
            );
        }
    }
}
