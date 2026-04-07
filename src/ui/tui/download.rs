//! TUI 下载页。
//!
//! 处理用户输入、启动下载任务、展示进度与状态。

use std::sync::{Arc, atomic::AtomicBool};
use std::thread;

use anyhow::Result;
use tracing::{debug, info, warn};

use crate::download::downloader::{
    self, ChapterRange, DownloadFlowOptions, DownloadMode, ProgressSnapshot, RetryFailed, SavePhase,
};

use super::{App, Focus, PendingDownload, View, WorkerMsg, start_spinner};

pub(super) fn request_cancel_download(app: &mut App) {
    if let Some(flag) = app.download_cancel_flag.as_ref() {
        flag.store(true, std::sync::atomic::Ordering::SeqCst);
        app.status = "Đã yêu cầu dừng tải xuống…".to_string();
        app.push_message("Đã gửi tín hiệu dừng, tác vụ hiện tại sẽ kết thúc sau");
    } else {
        app.status = "Hiện không có tải xuống nào đang chạy".to_string();
    }
    app.stop_button_area = None;
}

pub(super) fn start_download_task(
    app: &mut App,
    pending: PendingDownload,
    range: Option<ChapterRange>,
) -> Result<()> {
    // keep pending info for preview overlay while download runs
    app.pending_download = Some(pending.clone());
    app.preview_modal_open = false;
    app.preview_desc_scroll = 0;
    app.preview_desc_scroll_max = 0;
    app.preview_modal_scroll = 0;
    app.preview_modal_scroll_max = 0;
    app.last_preview_desc_area = None;
    app.download_progress = Some(ProgressSnapshot {
        group_done: 0,
        group_total: downloader::dynamic_group_count(pending.plan.chapters.len()),
        saved_chapters: pending.downloaded_count,
        chapter_total: pending.plan.chapters.len(),
        save_phase: SavePhase::TextSave,
        comment_fetch: 0,
        comment_total: if app.config.enable_segment_comments {
            pending.plan.chapters.len()
        } else {
            0
        },
        comment_saved: 0,
    });
    app.messages.clear();
    app.results.clear();
    app.list_state.select(None);
    app.cover_lines.clear();
    app.cover_title.clear();

    let book_id = pending.plan.book_id.clone();
    let title = pending
        .plan
        .meta
        .book_name
        .clone()
        .unwrap_or_else(|| book_id.clone());

    app.status = format!("Bat dau tai: 《{}》 ({})", title, book_id);
    info!(target: "ui", book_id = %book_id, "Bat dau tac vu tai");
    debug!(
        target: "ui",
        book_id = %book_id,
        save_path = %app.config.save_path,
        format = %app.config.novel_format,
        workers = app.config.max_workers,
        "Tham so tai"
    );

    start_spinner(app, format!("Dang tai: {book_id}"));
    let tx = app.worker_tx.clone();
    let progress_tx = app.worker_tx.clone();
    let cfg = app.config.clone();
    let cancel_flag = Arc::new(AtomicBool::new(false));
    app.download_cancel_flag = Some(cancel_flag.clone());
    app.stop_button_area = None;
    thread::spawn(move || {
        let progress_cb = move |snap: ProgressSnapshot| {
            let _ = progress_tx.send(WorkerMsg::DownloadProgress(snap));
        };
        let ask_tx = tx.clone();
        let book_name_asker = move |manager: &crate::book_parser::book_manager::BookManager| {
            let options = downloader::collect_book_name_options(manager);
            if options.len() <= 1 {
                return None;
            }
            let (resp_tx, resp_rx) = std::sync::mpsc::channel();
            let _ = ask_tx.send(WorkerMsg::AskBookName {
                options,
                respond_to: resp_tx,
            });
            resp_rx.recv().ok().flatten()
        };
        let format_ask_tx = tx.clone();
        let format_asker = move |_manager: &crate::book_parser::book_manager::BookManager| {
            let options = vec![
                downloader::BookNameOption {
                    label: "Định dạng TXT".to_string(),
                    value: "txt".to_string(),
                },
                downloader::BookNameOption {
                    label: "Định dạng EPUB".to_string(),
                    value: "epub".to_string(),
                },
            ];
            let (resp_tx, resp_rx) = std::sync::mpsc::channel();
            let _ = format_ask_tx.send(WorkerMsg::AskFormat {
                options,
                respond_to: resp_tx,
            });
            resp_rx.recv().ok().flatten()
        };
        let result = downloader::download_with_plan_flow(
            &cfg,
            pending.plan,
            None,
            DownloadFlowOptions {
                mode: DownloadMode::Resume,
                range,
                retry_failed: {
                    let mut retried = false;
                    RetryFailed::Decide(Box::new(move |_pending_len| {
                        if retried {
                            return false;
                        }
                        retried = true;
                        true
                    }))
                },
                stage_callback: None,
                book_name_asker: Some(Box::new(book_name_asker)),
                format_asker: Some(Box::new(format_asker)),
            },
            Some(Box::new(progress_cb)),
            Some(cancel_flag),
        );
        let msg = WorkerMsg::DownloadDone { book_id, result };
        let _ = tx.send(msg);
    });
    Ok(())
}

pub(super) fn apply_download_progress(app: &mut App, snap: ProgressSnapshot) {
    app.download_progress = Some(snap);
}

pub(super) fn apply_download_done(app: &mut App, book_id: String, result: Result<()>) {
    match result {
        Ok(()) => {
            app.status = format!("Tải xuống hoàn tất: {book_id}");
            app.push_message("Tải xuống hoàn tất");
            info!(target: "ui", book_id = %book_id, "Tai xong");
            app.pending_download = None;
            app.preview_range.clear();
            app.preview_buttons.select(Some(0));
            app.preview_modal_open = false;
            app.download_progress = None;
            app.view = View::Home;
            app.focus = Focus::Input;
            app.download_cancel_flag = None;
            app.stop_button_area = None;
        }
        Err(err) => {
            app.status = format!("Tải xuống thất bại: {err}");
            app.push_message(format!("Tải xuống thất bại: {err}"));
            warn!(target: "ui", book_id = %book_id, "Tai that bai: {err}");
            app.pending_download = None;
            app.preview_range.clear();
            app.preview_buttons.select(Some(0));
            app.preview_modal_open = false;
            app.download_progress = None;
            app.view = View::Home;
            app.focus = Focus::Input;
            app.download_cancel_flag = None;
            app.stop_button_area = None;
        }
    }
}
