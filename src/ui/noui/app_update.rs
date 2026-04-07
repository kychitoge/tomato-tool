//! 无 UI（旧 CLI）下的程序更新检查与提示。

use anyhow::Result;

use crate::base_system::app_update;

pub(super) fn startup_check() {
    let report = match app_update::check_update_report_blocking_with_timeout(
        env!("CARGO_PKG_VERSION"),
        std::time::Duration::from_secs(3),
    ) {
        Ok(r) => r,
        Err(_) => return,
    };

    if !app_update::should_notify_startup(&report) {
        return;
    }

    println!(
        "\nGợi ý: Phát hiện phiên bản mới {} (hiện tại {}). Nhập c để xem nhật ký cập nhật; nhập U để tự cập nhật (nếu có).\n",
        report.latest.tag_name, report.current_tag
    );

    if let Some(body) = report.latest.body.as_deref() {
        let preview = preview_notes(body, 8, 800);
        if !preview.trim().is_empty() {
            println!("Nhật ký cập nhật (trích đoạn):\n{}\n", preview);
        }
    }
}

pub(super) fn check_update_menu() -> Result<()> {
    let report = app_update::check_update_report_blocking(env!("CARGO_PKG_VERSION"))?;

    println!("\n===== Kiểm tra cập nhật chương trình =====");
    println!("Phiên bản hiện tại: {}", report.current_tag);
    println!("Phiên bản mới nhất: {}", report.latest.tag_name);

    if report.is_new_version {
        println!("Trạng thái: Có phiên bản mới");
    } else {
        println!("Trạng thái: Đã là phiên bản mới nhất");
    }

    if let Some(url) = report.latest.html_url.as_deref()
        && !url.trim().is_empty()
    {
        println!("Release: {}", url);
    }

    if let Some(body) = report.latest.body.as_deref() {
        let text = body.trim();
        if !text.is_empty() {
            println!("\n----- Nhật ký cập nhật -----\n{}\n--------------------", text);
        }
    }

    if report.is_new_version {
        let dismissed = app_update::dismissed_release_tag();
        if dismissed.as_deref() == Some(&report.latest.tag_name) {
            println!("Gợi ý: Bạn đã đặt bỏ qua nhắc nhở cho bản này (vẫn có thể kiểm tra thủ công).");
        }

        let ans = super::read_line("Có đặt không nhắc lại cho bản này không? [y/N]: ")?;
        let ans = ans.trim().to_ascii_lowercase();
        if ans == "y" || ans == "yes" {
            app_update::dismiss_release_tag(&report.latest.tag_name)?;
            println!("Đã đặt: không nhắc lại {}\n", report.latest.tag_name);
        }
    }

    Ok(())
}

fn preview_notes(body: &str, max_lines: usize, max_chars: usize) -> String {
    let mut out = String::new();
    for (i, line) in body.lines().enumerate() {
        if i >= max_lines {
            out.push('…');
            break;
        }
        let line = line.trim_end();
        if line.is_empty() {
            continue;
        }
        if !out.is_empty() {
            out.push('\n');
        }
        out.push_str(line);
        if out.len() >= max_chars {
            // 在字符边界安全截断，避免中文等多字节字符被截断导致 panic
            let mut end = max_chars;
            while !out.is_char_boundary(end) && end > 0 {
                end -= 1;
            }
            out.truncate(end);
            out.push('…');
            break;
        }
    }
    out
}
