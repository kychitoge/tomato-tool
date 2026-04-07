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

        (LANG_VI, "tui.home.title") => Cow::Borrowed("Tomato Novel Downloader (TUI)"),
        (LANG_VI, "tui.home.block_title") => Cow::Borrowed("Tomato Novel Downloader"),
        (LANG_VI, "tui.home.output_dir") => Cow::Borrowed("Thư mục xuất"),
        (LANG_VI, "tui.home.notice_keys") => Cow::Borrowed("  |  c: Cấu hình, q: Thoát"),
        (LANG_VI, "tui.home.notice_free") => Cow::Borrowed(
            "  |  Chương trình này hoàn toàn miễn phí; nếu bạn thấy kênh thu phí, hãy cảnh giác để tránh bị lừa đảo!",
        ),
        (LANG_VI, "tui.input.title") => {
            Cow::Borrowed("Nhập tên sách/ID/liên kết (Enter xác nhận, Tab chuyển)")
        }
        (LANG_VI, "tui.menu.title") => Cow::Borrowed("Thao tác (Enter hoặc click)"),
        (LANG_VI, "tui.menu.confirm") => Cow::Borrowed("Xác nhận"),
        (LANG_VI, "tui.menu.config") => Cow::Borrowed("Cấu hình"),
        (LANG_VI, "tui.menu.update") => Cow::Borrowed("Cập nhật"),
        (LANG_VI, "tui.menu.history") => Cow::Borrowed("Lịch sử"),
        (LANG_VI, "tui.menu.about") => Cow::Borrowed("Giới thiệu"),
        (LANG_VI, "tui.menu.quit") => Cow::Borrowed("Thoát"),
        (LANG_VI, "tui.results.title") => Cow::Borrowed("Kết quả tìm kiếm (↑↓ chọn, Enter tải)"),
        (LANG_VI, "tui.results.empty") => Cow::Borrowed("Không có kết quả"),
        (LANG_VI, "tui.status.block_title") => Cow::Borrowed("Trạng thái / Thông báo"),
        (LANG_VI, "tui.status.default") => Cow::Borrowed(
            "Nhập tên sách/ID/liên kết, Enter xác nhận; Tab chuyển focus; q thoát",
        ),
        (LANG_VI, "tui.status.enter_config") => Cow::Borrowed("Vào trang cấu hình"),
        (LANG_VI, "tui.status.about") => Cow::Borrowed("Giới thiệu"),
        (LANG_VI, "tui.status.searching") => Cow::Borrowed("Đang tìm kiếm…"),
        (LANG_VI, "tui.status.preview_cancelled") => Cow::Borrowed("Đã hủy xem trước đang chờ tải"),
        (LANG_VI, "tui.status.clipboard.android_not_ready") => Cow::Borrowed(
            "Clipboard Android chưa sẵn sàng: cần Termux + termux-api (termux-clipboard-get)",
        ),
        (LANG_VI, "tui.status.clipboard.backend_missing") => Cow::Borrowed(
            "Bản build hiện tại chưa kèm clipboard backend (bật clipboard-arboard)",
        ),
        (LANG_VI, "tui.status.clipboard.read_failed_prefix") => Cow::Borrowed("Đọc clipboard thất bại"),
        (LANG_VI, "tui.status.clipboard.disabled") => Cow::Borrowed("Bản build hiện tại chưa bật clipboard"),
        (LANG_VI, "tui.status.switch_failed_prefix") => Cow::Borrowed("Chuyển đổi thất bại"),
        (LANG_VI, "tui.status.old_cli_switched") => Cow::Borrowed(
            "Đã chuyển sang CLI cũ (thân thiện screen reader), hãy tự khởi động lại chương trình.",
        ),
        (LANG_VI, "tui.status.range_invalid_prefix") => Cow::Borrowed("Khoảng không hợp lệ"),
        (LANG_VI, "tui.status.empty_input_prompt") => Cow::Borrowed(
            "Hãy nhập tên sách, liên kết hoặc book_id, rồi nhấn Enter.",
        ),
        (LANG_VI, "tui.status.prepare_download_prefix") => Cow::Borrowed("Chuẩn bị tải sách"),
        (LANG_VI, "tui.status.resolving_short_link") => Cow::Borrowed("Đang phân giải link rút gọn…"),
        (LANG_VI, "tui.detail.selected") => Cow::Borrowed("Đã chọn"),
        (LANG_VI, "tui.detail.author") => Cow::Borrowed("Tác giả"),
        (LANG_VI, "tui.detail.id") => Cow::Borrowed("ID"),
        (LANG_VI, "tui.detail.words") => Cow::Borrowed("Số chữ"),
        (LANG_VI, "tui.detail.score") => Cow::Borrowed("Điểm"),
        (LANG_VI, "tui.detail.reads") => Cow::Borrowed("Lượt đọc"),
        (LANG_VI, "tui.detail.category") => Cow::Borrowed("Thể loại"),
        (LANG_VI, "tui.detail.alias") => Cow::Borrowed("Tên khác"),
        (LANG_VI, "tui.detail.original") => Cow::Borrowed("Tên gốc"),
        (LANG_VI, "tui.detail.first_chapter") => Cow::Borrowed("Chương đầu"),
        (LANG_VI, "tui.detail.last_chapter") => Cow::Borrowed("Chương cuối"),
        (LANG_VI, "tui.detail.chapters") => Cow::Borrowed("Số chương"),
        (LANG_VI, "tui.detail.status") => Cow::Borrowed("Tình trạng"),
        (LANG_VI, "tui.detail.status_finished") => Cow::Borrowed("Hoàn"),
        (LANG_VI, "tui.detail.status_ongoing") => Cow::Borrowed("Đang ra"),
        (LANG_VI, "tui.detail.tags") => Cow::Borrowed("Tag"),
        (LANG_VI, "tui.detail.desc") => Cow::Borrowed("Mô tả"),
        (LANG_VI, "tui.detail.desc_none") => Cow::Borrowed("Chưa có"),
        (LANG_VI, "tui.detail.desc_unloaded") => Cow::Borrowed("Chưa tải"),

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

        (LANG_EN, "tui.home.title") => Cow::Borrowed("Tomato Novel Downloader (TUI)"),
        (LANG_EN, "tui.home.block_title") => Cow::Borrowed("Tomato Novel Downloader"),
        (LANG_EN, "tui.home.output_dir") => Cow::Borrowed("Output"),
        (LANG_EN, "tui.home.notice_keys") => Cow::Borrowed("  |  c: Config, q: Quit"),
        (LANG_EN, "tui.home.notice_free") => Cow::Borrowed(
            "  |  This program is completely free. Beware of paid distribution channels!",
        ),
        (LANG_EN, "tui.input.title") => {
            Cow::Borrowed("Enter book name/ID/link (Enter confirm, Tab switch)")
        }
        (LANG_EN, "tui.menu.title") => Cow::Borrowed("Actions (Enter or click)"),
        (LANG_EN, "tui.menu.confirm") => Cow::Borrowed("Confirm"),
        (LANG_EN, "tui.menu.config") => Cow::Borrowed("Config"),
        (LANG_EN, "tui.menu.update") => Cow::Borrowed("Update"),
        (LANG_EN, "tui.menu.history") => Cow::Borrowed("History"),
        (LANG_EN, "tui.menu.about") => Cow::Borrowed("About"),
        (LANG_EN, "tui.menu.quit") => Cow::Borrowed("Quit"),
        (LANG_EN, "tui.results.title") => Cow::Borrowed("Search results (Up/Down, Enter to download)"),
        (LANG_EN, "tui.results.empty") => Cow::Borrowed("No results"),
        (LANG_EN, "tui.status.block_title") => Cow::Borrowed("Status / Messages"),
        (LANG_EN, "tui.status.default") => Cow::Borrowed(
            "Enter book name/ID/link, Enter confirm; Tab switch focus; q quit",
        ),
        (LANG_EN, "tui.status.enter_config") => Cow::Borrowed("Open config editor"),
        (LANG_EN, "tui.status.about") => Cow::Borrowed("About"),
        (LANG_EN, "tui.status.searching") => Cow::Borrowed("Searching…"),
        (LANG_EN, "tui.status.preview_cancelled") => Cow::Borrowed("Cancelled pending preview download"),
        (LANG_EN, "tui.status.clipboard.android_not_ready") => Cow::Borrowed(
            "Android clipboard not ready: requires Termux + termux-api (termux-clipboard-get)",
        ),
        (LANG_EN, "tui.status.clipboard.backend_missing") => Cow::Borrowed(
            "This build does not include a clipboard backend (enable clipboard-arboard)",
        ),
        (LANG_EN, "tui.status.clipboard.read_failed_prefix") => Cow::Borrowed("Failed to read clipboard"),
        (LANG_EN, "tui.status.clipboard.disabled") => Cow::Borrowed("Clipboard support is disabled in this build"),
        (LANG_EN, "tui.status.switch_failed_prefix") => Cow::Borrowed("Switch failed"),
        (LANG_EN, "tui.status.old_cli_switched") => Cow::Borrowed(
            "Switched to legacy CLI (screen-reader friendly). Please restart the app.",
        ),
        (LANG_EN, "tui.status.range_invalid_prefix") => Cow::Borrowed("Invalid range"),
        (LANG_EN, "tui.status.empty_input_prompt") => Cow::Borrowed(
            "Enter book name, link, or book_id, then press Enter.",
        ),
        (LANG_EN, "tui.status.prepare_download_prefix") => Cow::Borrowed("Preparing to download book"),
        (LANG_EN, "tui.status.resolving_short_link") => Cow::Borrowed("Resolving short link…"),
        (LANG_EN, "tui.detail.selected") => Cow::Borrowed("Selected"),
        (LANG_EN, "tui.detail.author") => Cow::Borrowed("Author"),
        (LANG_EN, "tui.detail.id") => Cow::Borrowed("ID"),
        (LANG_EN, "tui.detail.words") => Cow::Borrowed("Words"),
        (LANG_EN, "tui.detail.score") => Cow::Borrowed("Score"),
        (LANG_EN, "tui.detail.reads") => Cow::Borrowed("Reads"),
        (LANG_EN, "tui.detail.category") => Cow::Borrowed("Category"),
        (LANG_EN, "tui.detail.alias") => Cow::Borrowed("Alias"),
        (LANG_EN, "tui.detail.original") => Cow::Borrowed("Original"),
        (LANG_EN, "tui.detail.first_chapter") => Cow::Borrowed("First"),
        (LANG_EN, "tui.detail.last_chapter") => Cow::Borrowed("Last"),
        (LANG_EN, "tui.detail.chapters") => Cow::Borrowed("Chapters"),
        (LANG_EN, "tui.detail.status") => Cow::Borrowed("Status"),
        (LANG_EN, "tui.detail.status_finished") => Cow::Borrowed("Finished"),
        (LANG_EN, "tui.detail.status_ongoing") => Cow::Borrowed("Ongoing"),
        (LANG_EN, "tui.detail.tags") => Cow::Borrowed("Tags"),
        (LANG_EN, "tui.detail.desc") => Cow::Borrowed("Description"),
        (LANG_EN, "tui.detail.desc_none") => Cow::Borrowed("None"),
        (LANG_EN, "tui.detail.desc_unloaded") => Cow::Borrowed("Not loaded"),

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

        (_, "tui.home.title") => Cow::Borrowed("番茄小说下载器 TUI"),
        (_, "tui.home.block_title") => Cow::Borrowed("番茄小说下载器"),
        (_, "tui.home.output_dir") => Cow::Borrowed("输出目录"),
        (_, "tui.home.notice_keys") => Cow::Borrowed("  |  c: 配置, q: 退出"),
        (_, "tui.home.notice_free") => Cow::Borrowed("  |  本程序完全免费，若发现收费渠道，请勿上当受骗！"),
        (_, "tui.input.title") => Cow::Borrowed("输入书名/ID/链接 (Enter 确认, Tab 切换)"),
        (_, "tui.menu.title") => Cow::Borrowed("操作 (Enter 或鼠标点击)"),
        (_, "tui.menu.confirm") => Cow::Borrowed("确定"),
        (_, "tui.menu.config") => Cow::Borrowed("配置"),
        (_, "tui.menu.update") => Cow::Borrowed("更新"),
        (_, "tui.menu.history") => Cow::Borrowed("历史"),
        (_, "tui.menu.about") => Cow::Borrowed("关于"),
        (_, "tui.menu.quit") => Cow::Borrowed("退出"),
        (_, "tui.results.title") => Cow::Borrowed("搜索结果 (上下选择, Enter 下载)"),
        (_, "tui.results.empty") => Cow::Borrowed("无搜索结果"),
        (_, "tui.status.block_title") => Cow::Borrowed("状态 / 消息"),
        (_, "tui.status.default") => Cow::Borrowed("输入书名/ID/链接，Enter 确认，Tab 切换焦点，q 退出"),
        (_, "tui.status.enter_config") => Cow::Borrowed("进入配置编辑"),
        (_, "tui.status.about") => Cow::Borrowed("关于"),
        (_, "tui.status.searching") => Cow::Borrowed("搜索中…"),
        (_, "tui.status.preview_cancelled") => Cow::Borrowed("已取消待下载的预览"),
        (_, "tui.status.clipboard.android_not_ready") => Cow::Borrowed(
            "Android 剪贴板未就绪：需要 Termux + termux-api（termux-clipboard-get）",
        ),
        (_, "tui.status.clipboard.backend_missing") => Cow::Borrowed(
            "当前构建未包含剪贴板后端（启用 clipboard-arboard）",
        ),
        (_, "tui.status.clipboard.read_failed_prefix") => Cow::Borrowed("读取剪贴板失败"),
        (_, "tui.status.clipboard.disabled") => Cow::Borrowed("当前构建未启用剪贴板支持"),
        (_, "tui.status.switch_failed_prefix") => Cow::Borrowed("切换失败"),
        (_, "tui.status.old_cli_switched") => Cow::Borrowed(
            "已切换到旧版CLI(读屏友好)，请手动重启程序。",
        ),
        (_, "tui.status.range_invalid_prefix") => Cow::Borrowed("范围无效"),
        (_, "tui.status.empty_input_prompt") => Cow::Borrowed("请输入书名、链接或 book_id，按 Enter 开始。"),
        (_, "tui.status.prepare_download_prefix") => Cow::Borrowed("准备下载书籍"),
        (_, "tui.status.resolving_short_link") => Cow::Borrowed("正在解析短链接…"),
        (_, "tui.detail.selected") => Cow::Borrowed("选中"),
        (_, "tui.detail.author") => Cow::Borrowed("作者"),
        (_, "tui.detail.id") => Cow::Borrowed("ID"),
        (_, "tui.detail.words") => Cow::Borrowed("字数"),
        (_, "tui.detail.score") => Cow::Borrowed("评分"),
        (_, "tui.detail.reads") => Cow::Borrowed("阅读"),
        (_, "tui.detail.category") => Cow::Borrowed("分类"),
        (_, "tui.detail.alias") => Cow::Borrowed("别名"),
        (_, "tui.detail.original") => Cow::Borrowed("原名"),
        (_, "tui.detail.first_chapter") => Cow::Borrowed("首章"),
        (_, "tui.detail.last_chapter") => Cow::Borrowed("末章"),
        (_, "tui.detail.chapters") => Cow::Borrowed("章节"),
        (_, "tui.detail.status") => Cow::Borrowed("状态"),
        (_, "tui.detail.status_finished") => Cow::Borrowed("完结"),
        (_, "tui.detail.status_ongoing") => Cow::Borrowed("连载"),
        (_, "tui.detail.tags") => Cow::Borrowed("标签"),
        (_, "tui.detail.desc") => Cow::Borrowed("简介"),
        (_, "tui.detail.desc_none") => Cow::Borrowed("暂无"),
        (_, "tui.detail.desc_unloaded") => Cow::Borrowed("未加载"),
        _ => Cow::Owned(key.to_string()),
    }
}
