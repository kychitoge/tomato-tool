//! noUI 下载历史查看。

use anyhow::Result;

use crate::base_system::download_history::read_download_history;

pub(super) fn show_history_menu() -> Result<()> {
    let mut keyword: Option<String> = None;

    loop {
        let items = read_download_history(50, keyword.as_deref());
        println!("\n===== Lịch sử tải xuống (50 bản ghi gần nhất) =====");
        if let Some(k) = keyword.as_deref() {
            println!("Từ khóa lọc: {}", k);
        }

        if items.is_empty() {
            println!("Không có bản ghi");
        } else {
            for (i, it) in items.iter().enumerate() {
                println!(
                    "{:>2}. [{}] 《{}》({}) | Tác giả: {} | {} | Trạng thái: {}",
                    i + 1,
                    it.timestamp,
                    it.book_name,
                    it.book_id,
                    if it.author.trim().is_empty() {
                        "Không rõ"
                    } else {
                        it.author.trim()
                    },
                    it.progress,
                    it.status
                );
            }
        }

        println!("\nThao tác: Enter=làm mới, f=đặt từ khóa lọc, c=xóa lọc, q=quay lại");
        let cmd = super::read_line("Chọn: ")?;
        let cmd = cmd.trim();
        if cmd.is_empty() {
            continue;
        }
        if cmd.eq_ignore_ascii_case("q") {
            break;
        }
        if cmd.eq_ignore_ascii_case("f") {
            let q = super::read_line("Nhập từ khóa tên sách/tác giả/ID: ")?;
            let q = q.trim();
            if q.is_empty() {
                println!("Từ khóa trống, giữ nguyên bộ lọc hiện tại.\n");
            } else {
                keyword = Some(q.to_string());
            }
            continue;
        }
        if cmd.eq_ignore_ascii_case("c") {
            keyword = None;
            continue;
        }
    }

    Ok(())
}
