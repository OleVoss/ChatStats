use whatsapp_analyzer::analyzer::ChatAnalyzer;
use whatsapp_analyzer::WhatAppChat;

pub const TEST_CHAT: &'static str = "
[29.07.23, 17:01:23] Ole Voß: Gehts dir mittlerweile wieder bisschen besser?
[29.07.23, 17:02:15] Ole Voß: Und check ich voll. Is ja auch nur ungeil wenn sich die ganze Zeit beschwert wird
[29.07.23, 17:03:06] Alicia: mach dir um mich meine Gedanken. Wie geht es eher dir damit?
[29.07.23, 18:43:16] Ole Voß: Mach ich trotzdem. Passiert bei Freunden so. 

Ich finds kacke. Kam doch ziemlich plötzlich und bin einfach entäuscht.
[29.07.23, 18:44:14] Ole Voß: So langsam komme ich klar aber gibt genug Momente an denen ichs richtig scheiße finde. Z.b alleine aufwachen und so die kleinen Alltagsmomente, die ich dann nur alleine erlebe.
[29.07.23, 18:44:22] Ole Voß: Aber hey, life goes on und so
[29.07.23, 18:50:52] Alicia: wie lange wart ihr zsm? :(
[29.07.23, 18:51:06] Ole Voß: 10 Monate
[29.07.23, 18:51:41] Ole Voß: Halt jetzt die ganze Zeit in Würzburg. Was halt zusätzlich whack ist. Jetzt muss ich nämlich alleine hier nochmal neu ankommen irgendwie
[29.07.23, 18:52:06] Alicia: oh Men.. das tut mir leid..
[29.07.23, 18:53:10] Ole Voß: Facetime ind Kippe?
[29.07.23, 18:53:14] Ole Voß: *und
[29.07.23, 20:16:56] Alicia: bin auf Arbeit leider 🙄
";

pub fn setup_emtpy_chat() -> WhatAppChat {
    WhatAppChat::new()
}

pub fn setup_analyzer_with_chat() -> ChatAnalyzer {
    let chat = WhatAppChat::from(TEST_CHAT);
    ChatAnalyzer::new().chat(chat)
}

pub fn setup_analyzer_with_dfs() -> ChatAnalyzer {
    let chat = WhatAppChat::from(TEST_CHAT);
    let mut analyzer = ChatAnalyzer::new().chat(chat);
    analyzer.build_dfs();

    return analyzer;
}
