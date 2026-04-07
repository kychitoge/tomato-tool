pub(crate) const INDEX_HTML_RAW: &str = include_str!("templates/index.html");
pub(crate) const APP_JS: &str = include_str!("templates/app.js");
pub(crate) const APP_CSS: &str = include_str!("templates/app.css");
pub(crate) const APP_FAVICON_ICO: &[u8] = include_bytes!("../../../img/Tomato-downloader-ico.ico");

#[cfg(feature = "official-api")]
use crate::base_system::i18n;

/// 仅当启用 official-api feature 时才注入免费声明。
/// 未启用时展开为空串，占位符从 HTML 中抹除。
#[cfg(feature = "official-api")]
pub(crate) fn free_notice_html(lang: &str) -> String {
    format!(
        r#"<div class="free-notice">{} &middot; <a href="https://github.com/kychitoge/tomato-tool" target="_blank" rel="noopener">开源仓库</a><br />{}</div>"#,
        i18n::tr(lang, "web.free_text"),
        i18n::tr(lang, "web.free_warn")
    )
}

#[cfg(not(feature = "official-api"))]
pub(crate) fn free_notice_html(_lang: &str) -> String {
    String::new()
}

/// 移动端插入到状态页底部的免费声明（仅 official-api feature 启用时非空）。
#[cfg(feature = "official-api")]
pub(crate) fn free_notice_mobile_html(lang: &str) -> String {
    format!(
        r#"<div class="free-notice free-notice-mobile">{} &middot; <a href="https://github.com/kychitoge/tomato-tool" target="_blank" rel="noopener">开源仓库</a><br />{}</div>"#,
        i18n::tr(lang, "web.free_text"),
        i18n::tr(lang, "web.free_warn")
    )
}

#[cfg(not(feature = "official-api"))]
pub(crate) fn free_notice_mobile_html(_lang: &str) -> String {
    String::new()
}
