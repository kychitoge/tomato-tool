//! 无 UI 下载交互与执行。

use std::fs;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result, anyhow};

use crate::base_system::context::Config;
use crate::download::downloader as dl;
use crate::download::downloader::ChapterRef;

#[derive(Debug, Clone, Copy)]
struct DownloadOptions {
    interactive: bool,
    retry_failed_once: bool,
}

impl DownloadOptions {
    fn interactive() -> Self {
        Self {
            interactive: true,
            retry_failed_once: false,
        }
    }

    fn non_interactive(retry_failed_once: bool) -> Self {
        Self {
            interactive: false,
            retry_failed_once,
        }
    }
}

pub(super) fn download_book(book_id: &str, config: &Config) -> Result<()> {
    download_book_with_options(book_id, config, DownloadOptions::interactive())
}

pub(super) fn download_book_non_interactive(
    book_id: &str,
    config: &Config,
    retry_failed_once: bool,
) -> Result<()> {
    download_book_with_options(
        book_id,
        config,
        DownloadOptions::non_interactive(retry_failed_once),
    )
}

pub(super) fn update_existing_book_non_interactive(
    book_id: &str,
    config: &Config,
    retry_failed_once: bool,
) -> Result<()> {
    ensure_local_download_exists(config, book_id)?;
    download_book_non_interactive(book_id, config, retry_failed_once)
}

fn ensure_local_download_exists(config: &Config, book_id: &str) -> Result<()> {
    if has_local_download_record(config, book_id)? {
        return Ok(());
    }

    Err(anyhow!(
        "Chế độ CLI chỉ hỗ trợ cập nhật truyện đã có sẵn cục bộ: không tìm thấy bản ghi tải xuống của book_id={} trong {}. Hãy dùng Web UI hoặc TUI để tải lần đầu.",
        config.default_save_dir().display(),
        book_id
    ))
}

fn has_local_download_record(config: &Config, book_id: &str) -> Result<bool> {
    Ok(config
        .find_existing_status_folder_by_book_id(book_id, None)
        .with_context(|| format!("Doc thu muc luu that bai: {}", config.default_save_dir().display()))?
        .is_some())
}

fn download_book_with_options(
    book_id: &str,
    config: &Config,
    options: DownloadOptions,
) -> Result<()> {
    let start_time = Instant::now();

    let plan = dl::prepare_download_plan(config, book_id, dl::BookMeta::default())
        .with_context(|| format!("Chuan bi ke hoach tai that bai: book_id={}", book_id))?;

    let book_name = plan
        .meta
        .book_name
        .clone()
        .unwrap_or_else(|| plan.book_id.clone());

    // In thông tin sách (giữ cách hiển thị tương thích với bản cũ)
    println!("\nTen sach: {}", book_name);
    if let Some(author) = plan.meta.author.as_deref() {
        println!("Tac gia: {}", author);
    }
    if let Some(finished) = plan.meta.finished {
        println!("Trang thai hoan thanh: {}", if finished { "Da hoan" } else { "Dang ra" });
    }
    if let Some(count) = plan.meta.chapter_count {
        println!("So chuong: {}", count);
    }
    if !plan.meta.tags.is_empty() {
        println!("The: {}", plan.meta.tags.join("|"));
    }
    if let Some(desc) = plan.meta.description.as_deref() {
        let mut short = desc.to_string();
        if short.chars().count() > 50 {
            short = short.chars().take(50).collect::<String>() + "...";
        }
        println!("Tom tat: {}", short);
    }

    // 初始化 BookManager 并尝试加载历史状态
    let mut manager = dl::init_manager_from_plan(config, &plan)?;
    let resumed =
        manager.load_existing_status(&manager.book_id.clone(), &manager.book_name.clone());
    if resumed {
        println!("\nĐã phát hiện bản ghi tải xuống cũ, có thể tiếp tục tải hoặc chọn tải lại.\n");
    }

    // 若封面已经下载到状态目录，尝试 ASCII 预览
    if let Some(cover) = find_cover_image(manager.book_folder()) {
        let _ = preview_cover_ascii(&cover);
    }

    let total = plan.chapters.len();
    let (downloaded_ok, failed_count) = count_download_state(&manager, &plan.chapters);
    println!(
        "Tong cong {} chuong, tai that bai {} chuong, da tai {} chuong",
        total, failed_count, downloaded_ok
    );

    let mut range: Option<dl::ChapterRange> = None;
    let mode = if options.interactive {
        if downloaded_ok > 0 || failed_count > 0 {
            select_download_mode(failed_count > 0)?
        } else {
            DownloadMode::RangeOrAll
        }
    } else {
        DownloadMode::Resume
    };

    match mode {
        DownloadMode::Cancel => {
            let _ = manager.cleanup_status_folder();
            return Ok(());
        }
        DownloadMode::Full => {
            manager.downloaded.clear();
            println!("Se tai lai toan bo chuong");
        }
        DownloadMode::RangeIgnoreHistory | DownloadMode::RangeOrAll => {
            range = if options.interactive {
                prompt_range(total)?
            } else {
                None
            };
            if matches!(mode, DownloadMode::RangeIgnoreHistory) {
                manager.downloaded.clear();
            }
        }
        DownloadMode::Resume | DownloadMode::FailedOnly => {}
    }

    let chosen_chapters = dl::apply_range(&plan.chapters, range);
    if chosen_chapters.is_empty() {
        println!("Pham vi khong hop le hoac danh sach chuong rong\n");
        let _ = manager.cleanup_status_folder();
        return Ok(());
    }

    let pending = match mode {
        DownloadMode::FailedOnly => dl::pending_failed(&manager, &chosen_chapters),
        _ => dl::pending_resume(&manager, &chosen_chapters),
    };

    if matches!(mode, DownloadMode::Resume) {
        println!(
            "Tiếp tục tải các chương còn lại: {} chương (đã hoàn tất {})",
            pending.len(),
            chosen_chapters.len().saturating_sub(pending.len())
        );
    }

    if pending.is_empty() {
        println!("Không có chương nào cần tải; sẽ chỉ bổ sung bộ nhớ đệm đoạn bình luận và chạy bước hoàn tất.\n");
    }

    println!("\nBat dau tai...");
    let save_dir = manager.default_save_dir();

    let retry_failed = if options.interactive {
        dl::RetryFailed::Decide(Box::new(|pending_len| {
            let ans = super::read_line("Có tải lại các chương lỗi không? [Y/n]: ")
                .map(|s| s.trim().to_ascii_lowercase())
                .unwrap_or_else(|_| "n".to_string());
            if ans == "n" {
                println!("Các chương lỗi đã được giữ trong bộ nhớ đệm/tệp trạng thái.\n");
                return false;
            }
            println!("\nĐang tải lại các chương lỗi: {} chương...", pending_len);
            true
        }))
    } else if options.retry_failed_once {
        let mut retried = false;
        dl::RetryFailed::Decide(Box::new(move |pending_len| {
            if retried {
                return false;
            }
            retried = true;
            println!("\nĐang tải lại các chương lỗi: {} chương...", pending_len);
            true
        }))
    } else {
        dl::RetryFailed::Never
    };

    let exec_mode = match mode {
        DownloadMode::Full => dl::DownloadMode::Full,
        DownloadMode::FailedOnly => dl::DownloadMode::FailedOnly,
        DownloadMode::RangeIgnoreHistory => dl::DownloadMode::RangeIgnoreHistory,
        _ => dl::DownloadMode::Resume,
    };

    dl::download_with_plan_flow(
        config,
        plan,
        Some(manager),
        dl::DownloadFlowOptions {
            mode: exec_mode,
            range,
            retry_failed,
            stage_callback: Some(Box::new(|result| {
                println!(
                    "\nTai xong (giai doan): thanh cong {} chuong | that bai {} chuong | huy {} chuong",
                    result.success, result.failed, result.canceled
                );
            })),
            book_name_asker: None,
            format_asker: None,
        },
        None,
        None,
    )?;

    println!(
        "\nTai xong! Mat {:.1} giay",
        start_time.elapsed().as_secs_f32()
    );
    println!("Da luu vao {}", save_dir.display());
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DownloadMode {
    Resume,
    Full,
    FailedOnly,
    RangeIgnoreHistory,
    RangeOrAll,
    Cancel,
}

fn select_download_mode(has_failed: bool) -> Result<DownloadMode> {
    println!("\n===== Chon che do tai =====");
    println!("1. Tiep tuc tai cac chuong chua xong");
    println!("2. Tai lai toan bo");
    if has_failed {
        println!("3. Chi tai lai cac chuong that bai");
    }
    println!("4. Tai lai theo pham vi chuong (bo qua lich su)");
    println!("q. Huy");
    let sel = super::read_line("Hay chon (mac dinh 1): ")?;
    let sel = sel.trim().to_ascii_lowercase();
    let mode = match sel.as_str() {
        "" | "1" => DownloadMode::Resume,
        "2" => DownloadMode::Full,
        "3" if has_failed => DownloadMode::FailedOnly,
        "4" => DownloadMode::RangeIgnoreHistory,
        "q" => DownloadMode::Cancel,
        _ => DownloadMode::Resume,
    };
    Ok(mode)
}

fn prompt_range(total: usize) -> Result<Option<dl::ChapterRange>> {
    let text = super::read_line("Nhap pham vi chuong dang 10~200 (de trong = tat ca): ")?;
    let text = text.trim();
    if text.is_empty() {
        return Ok(None);
    }
    let Some((a, b)) = text.split_once('~') else {
        println!("Sai dinh dang pham vi, can a~b; se dung tat ca chuong");
        return Ok(None);
    };
    let Ok(mut start) = a.trim().parse::<usize>() else {
        println!("Khong phan tich duoc pham vi; se dung tat ca chuong");
        return Ok(None);
    };
    let Ok(mut end) = b.trim().parse::<usize>() else {
        println!("Khong phan tich duoc pham vi; se dung tat ca chuong");
        return Ok(None);
    };
    if start == 0 {
        start = 1;
    }
    if end == 0 {
        end = 1;
    }
    start = start.min(total).max(1);
    end = end.min(total).max(1);
    if start > end {
        std::mem::swap(&mut start, &mut end);
    }
    println!("Da chon pham vi chuong: {}~{}", start, end);
    Ok(Some(dl::ChapterRange { start, end }))
}

fn count_download_state(
    manager: &crate::book_parser::book_manager::BookManager,
    chapters: &[ChapterRef],
) -> (usize, usize) {
    let mut ok = 0usize;
    let mut failed = 0usize;
    for ch in chapters {
        match manager.downloaded.get(&ch.id) {
            Some((_, Some(_))) => ok += 1,
            Some((_, None)) => failed += 1,
            None => {}
        }
    }
    (ok, failed)
}

fn find_cover_image(folder: &Path) -> Option<PathBuf> {
    let rd = fs::read_dir(folder).ok()?;
    for entry in rd.flatten() {
        let p = entry.path();
        if !p.is_file() {
            continue;
        }
        let ext = p
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if matches!(ext.as_str(), "jpg" | "jpeg" | "png") {
            return Some(p);
        }
    }
    None
}

fn preview_cover_ascii(image_path: &Path) -> Result<()> {
    let (cols, rows) = crossterm::terminal::size().unwrap_or((80, 24));
    let cols = cols.max(40) as u32;
    let rows = rows.max(10) as u32;
    println!(
        "\n{}Xem truoc bia{}",
        "=".repeat((cols as usize).saturating_sub(16) / 2),
        "=".repeat((cols as usize).saturating_sub(16) / 2)
    );

    let img = image::open(image_path)
        .with_context(|| format!("Mo file bia that bai: {}", image_path.display()))?;
    let gray = img.to_luma8();

    // 字符宽高比矫正：字符通常更“高”，所以宽度多取一些、并降低高度
    let target_w = cols;
    let target_h = (rows.saturating_sub(6)).max(8);
    let resized = image::imageops::resize(
        &gray,
        target_w,
        target_h,
        image::imageops::FilterType::Triangle,
    );

    const PALETTE: &[u8] = b" .:-=+*#%@";
    for y in 0..resized.height() {
        let mut line = String::with_capacity(resized.width() as usize);
        for x in 0..resized.width() {
            let v = resized.get_pixel(x, y)[0] as usize;
            let idx = v * (PALETTE.len() - 1) / 255;
            line.push(PALETTE[idx] as char);
        }
        println!("{}", line);
    }
    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::has_local_download_record;
    use crate::base_system::context::Config;

    #[test]
    fn local_download_record_requires_matching_status_files() {
        let temp_dir = tempfile::tempdir().unwrap();
        let mut config = Config::default();
        config.save_path = temp_dir.path().display().to_string();

        let valid = temp_dir.path().join("123_test-book");
        std::fs::create_dir_all(&valid).unwrap();
        std::fs::write(valid.join("status.json"), "{}\n").unwrap();

        let invalid = temp_dir.path().join("123_preview-only");
        std::fs::create_dir_all(&invalid).unwrap();

        assert!(has_local_download_record(&config, "123").unwrap());
        assert!(!has_local_download_record(&config, "456").unwrap());
    }
}
