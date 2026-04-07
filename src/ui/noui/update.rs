//! 无 UI 的更新检查与提示。

use std::path::Path;

use crate::base_system::novel_updates;
use anyhow::Result;

use crate::base_system::context::Config;

#[derive(Debug, Clone)]
pub(super) struct UpdateEntry {
    pub(super) book_id: String,
    pub(super) label: String,
}

pub(super) fn update_menu(config: &Config) -> Result<Option<String>> {
    let save_dir = config.default_save_dir();
    if !save_dir.exists() {
        println!(
            "Không có truyện nào để cập nhật (thư mục lưu không tồn tại): {}\n",
            save_dir.display()
        );
        return Ok(None);
    }

    let (updates, no_updates) = scan_updates(config, &save_dir)?;
    if updates.is_empty() && no_updates.is_empty() {
        println!("Không có truyện nào để cập nhật\n");
        return Ok(None);
    }

    loop {
        println!("\n===== Danh sách truyện có thể cập nhật =====");
        for (idx, u) in updates.iter().enumerate() {
            println!("{}. {}", idx + 1, u.label);
        }
        let opt_no_update = if no_updates.is_empty() {
            None
        } else {
            let n = updates.len() + 1;
            println!("{}. Không có cập nhật ({})", n, no_updates.len());
            Some(n)
        };
        println!("q. Thoát\n");

        let sel = super::read_line("Nhập số thứ tự: ")?;
        let sel = sel.trim().to_ascii_lowercase();
        if sel == "q" {
            println!("Đã hủy cập nhật\n");
            return Ok(None);
        }
        let Ok(n) = sel.parse::<usize>() else {
            println!("Lỗi: Hãy nhập số thứ tự hoặc q để thoát.\n");
            continue;
        };

        if n >= 1 && n <= updates.len() {
            return Ok(Some(updates[n - 1].book_id.clone()));
        }

        if let Some(no_idx) = opt_no_update
            && n == no_idx
            && let Some(book_id) = select_from_list(&no_updates, "Sach khong co cap nhat")?
        {
            return Ok(Some(book_id));
        }
        if let Some(no_idx) = opt_no_update
            && n == no_idx
        {
            continue;
        }

        let max = opt_no_update.unwrap_or(updates.len());
        println!("Lỗi: Hãy nhập số từ 1 đến {} hoặc q để thoát.\n", max);
    }
}

fn select_from_list(list: &[UpdateEntry], title: &str) -> Result<Option<String>> {
    loop {
        println!("\n===== {} =====", title);
        for (idx, u) in list.iter().enumerate() {
            println!("{}. {}", idx + 1, u.label);
        }
        println!("q. Hủy và quay lại menu trước\n");

        let sel = super::read_line("Nhập số thứ tự: ")?;
        let sel = sel.trim().to_ascii_lowercase();
        if sel == "q" {
            return Ok(None);
        }
        let Ok(n) = sel.parse::<usize>() else {
            println!("Lỗi: Hãy nhập số thứ tự hoặc q để quay lại.\n");
            continue;
        };
        if n >= 1 && n <= list.len() {
            return Ok(Some(list[n - 1].book_id.clone()));
        }
        println!("Lỗi: Hãy nhập số từ 1 đến {} hoặc q để quay lại.\n", list.len());
    }
}

fn scan_updates(_config: &Config, save_dir: &Path) -> Result<(Vec<UpdateEntry>, Vec<UpdateEntry>)> {
    let scan = novel_updates::scan_novel_updates(save_dir)?;

    let to_entry = |it: novel_updates::NovelUpdateRow| {
        let ignore_marker = if it.is_ignored { "[Đã bỏ qua] " } else { "" };
        UpdateEntry {
            book_id: it.book_id.clone(),
            label: format!(
                "{}《{}》({}) — Chương mới: {}",
                ignore_marker, it.book_name, it.book_id, it.new_count
            ),
        }
    };

    Ok((
        scan.updates.into_iter().map(to_entry).collect(),
        scan.no_updates.into_iter().map(to_entry).collect(),
    ))
}
