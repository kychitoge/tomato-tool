/// UI i18n helpers and language-aware defaults.
use std::borrow::Cow;

pub const LANG_ZH_CN: &str = "zh-cn";
pub const LANG_VI: &str = "vi";
pub const LANG_EN: &str = "en";

pub fn normalize_lang_tag(input: &str) -> &'static str {
    let s = input.trim().to_ascii_lowercase();
    match s.as_str() {
        "vi" | "vi-vn" => LANG_VI,
        "en" | "en-us" | "en-gb" => LANG_EN,
        "zh" | "zh-cn" | "zh-hans" => LANG_ZH_CN,
        _ => LANG_ZH_CN,
    }
}

pub fn default_edge_tts_voice(lang: &str) -> &'static str {
    match normalize_lang_tag(lang) {
        LANG_VI => "vi-VN-HoaiMyNeural",
        LANG_EN => "en-US-AriaNeural",
        _ => "zh-CN-XiaoxiaoNeural",
    }
}

pub fn tr(lang: &str, key: &str) -> Cow<'static, str> {
    match (normalize_lang_tag(lang), key) {
        (LANG_VI, "noui.project") => Cow::Borrowed("Dự án"),
        (LANG_VI, "noui.fork") => Cow::Borrowed("Fork từ"),
        (LANG_VI, "noui.author") => Cow::Borrowed("Tác giả"),
        (LANG_VI, "noui.early") => Cow::Borrowed("Mã nguồn giai đoạn đầu"),
        (LANG_VI, "noui.free_title") => Cow::Borrowed("[Tuyên bố miễn phí]"),
        (LANG_VI, "noui.free_text") => Cow::Borrowed(
            "Chương trình này hoàn toàn miễn phí. Nếu bạn thấy kênh phân phối thu phí, vui lòng cảnh giác để tránh bị lừa đảo.",
        ),
        (LANG_VI, "about.project") => Cow::Borrowed("Dự án"),
        (LANG_VI, "about.fork") => Cow::Borrowed("Fork từ"),
        (LANG_VI, "about.author") => Cow::Borrowed("Tác giả"),
        (LANG_VI, "about.free_title") => Cow::Borrowed("===== THÔNG BÁO MIỄN PHÍ ====="),
        (LANG_VI, "about.free_text") => {
            Cow::Borrowed("Chương trình này hoàn toàn miễn phí.")
        }
        (LANG_VI, "web.free_text") => Cow::Borrowed("Chương trình này hoàn toàn miễn phí."),
        (LANG_VI, "web.free_warn") => Cow::Borrowed("Nếu bạn thấy kênh thu phí, vui lòng cảnh giác để tránh bị lừa đảo."),

        (LANG_EN, "noui.project") => Cow::Borrowed("Project"),
        (LANG_EN, "noui.fork") => Cow::Borrowed("Fork from"),
        (LANG_EN, "noui.author") => Cow::Borrowed("Author"),
        (LANG_EN, "noui.early") => Cow::Borrowed("Early-stage code"),
        (LANG_EN, "noui.free_title") => Cow::Borrowed("[Free Notice]"),
        (LANG_EN, "noui.free_text") => Cow::Borrowed("This program is completely free. Beware of paid distribution channels!"),
        (LANG_EN, "about.project") => Cow::Borrowed("Project"),
        (LANG_EN, "about.fork") => Cow::Borrowed("Fork from"),
        (LANG_EN, "about.author") => Cow::Borrowed("Author"),
        (LANG_EN, "about.free_title") => Cow::Borrowed("===== Free Notice ====="),
        (LANG_EN, "about.free_text") => Cow::Borrowed("This program is completely free"),
        (LANG_EN, "web.free_text") => Cow::Borrowed("This program is completely free"),
        (LANG_EN, "web.free_warn") => Cow::Borrowed("If you see paid channels, do not be scammed!"),

        (_, "noui.project") => Cow::Borrowed("项目地址"),
        (_, "noui.fork") => Cow::Borrowed("Fork From"),
        (_, "noui.author") => Cow::Borrowed("作者"),
        (_, "noui.early") => Cow::Borrowed("项目早期代码"),
        (_, "noui.free_title") => Cow::Borrowed("【免费声明】"),
        (_, "noui.free_text") => Cow::Borrowed("本程序完全免费，若发现收费渠道，请勿上当受骗！"),
        (_, "about.project") => Cow::Borrowed("项目地址"),
        (_, "about.fork") => Cow::Borrowed("Fork From"),
        (_, "about.author") => Cow::Borrowed("作者"),
        (_, "about.free_title") => Cow::Borrowed("===== 免费声明 ====="),
        (_, "about.free_text") => Cow::Borrowed("本程序完全免费"),
        (_, "web.free_text") => Cow::Borrowed("本程序完全免费"),
        (_, "web.free_warn") => Cow::Borrowed("若发现收费渠道，请勿上当受骗！"),
        _ => Cow::Owned(key.to_string()),
    }
}
