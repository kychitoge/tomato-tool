//! 无 UI（旧 CLI）交互入口。
//!
//! 使用标准输入输出进行交互，并在进入前尽量恢复终端模式。

use std::io::{self, BufRead, Write};

use anyhow::Result;

use crossterm::event::DisableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{LeaveAlternateScreen, disable_raw_mode};

use crate::base_system::context::Config;

mod app_update;
mod config;
mod download;
mod history;
mod update;

fn show_config_menu(config: &mut Config) -> Result<()> {
    config::show_config_menu(config)
}

pub(crate) fn download_book(book_id: &str, config: &Config) -> Result<()> {
    download::download_book(book_id, config)
}

pub(crate) fn update_existing_book_non_interactive(
    book_id: &str,
    config: &Config,
    retry_failed: bool,
) -> Result<()> {
    download::update_existing_book_non_interactive(book_id, config, retry_failed)
}

pub fn run(config: &mut Config) -> Result<()> {
    // In case the previous run exited while in TUI raw mode (e.g., Ctrl+C),
    // best-effort restore the console so stdin line input works in PowerShell.
    let _ = disable_raw_mode();
    let mut out = io::stdout();
    let _ = execute!(out, DisableMouseCapture, LeaveAlternateScreen);

    println!(
        "Chao mung ban den voi Tomato Novel Downloader! v{}\n\
    Du an: https://github.com/zhongbai2333/Tomato-Novel-Downloader \n\
    Fork tu: https://github.com/Dlmily/Tomato-Novel-Downloader-Lite \n\
    Tac gia: zhongbai233 (https://github.com/zhongbai2333) \n\
    Ma nguon giai doan dau: Dlmily (https://github.com/Dlmily) \n\
    \n\
    Mo ta: Du an nay fork tu ban cua Dlmily va da duoc tai cau truc + toi uu, bo sung nhieu tinh nang nhu ho tro EPUB, tiep tuc tai tot hon va quan ly loi tot hon. \n\
    Du an nay su dung API ben thu ba, khong su dung API chinh thuc. Neu can, hay tham khao du an cua Dlmily. \n\
    Du an chi dung cho muc dich hoc tap ve crawler xu ly du lieu web va nghien cuu lien quan. Khong dung cho bat ky hanh vi vi pham phap luat hoac xam pham quyen loi nguoi khac.",
        env!("CARGO_PKG_VERSION")
    );

    #[cfg(feature = "official-api")]
    println!(
                "\n[Tuyen bo mien phi] Chuong trinh nay hoan toan mien phi, neu co kenh thu phi thi hay canh giac!\n\
            Kho chinh thuc: https://github.com/zhongbai2333/Tomato-Novel-Downloader"
    );

    // 每次启动检查程序更新（不影响后续流程，失败直接忽略）。
    app_update::startup_check();

    loop {
        let prompt = format!(
            "CLI cu da tat tinh nang tai moi; hay nhap lenh (s cau hinh / h lich su tai / u cap nhat sach / c kiem tra cap nhat / U tu cap nhat chuong trinh / q thoat, mac dinh luu tai {}):",
            config.default_save_dir().display()
        );
        let input = read_line(&prompt)?;
        let text = input.trim();
        if text.is_empty() {
            continue;
        }
        if text.eq_ignore_ascii_case("q") {
            println!("Da thoat.");
            break;
        }
        if text.eq_ignore_ascii_case("s") {
            show_config_menu(config)?;
            continue;
        }
        if text.eq_ignore_ascii_case("h") {
            history::show_history_menu()?;
            continue;
        }
        if text.eq_ignore_ascii_case("u") {
            if let Some(book_id) = update::update_menu(config)? {
                println!("Da chon cap nhat book_id={}\n", book_id);
                // 直接进入该书下载流程
                match download_book(&book_id, config) {
                    Ok(()) => println!("Tai xong\n"),
                    Err(err) => println!("Tai that bai: {}\n", err),
                }
            }
            continue;
        }

        if text.eq_ignore_ascii_case("c") {
            app_update::check_update_menu()?;
            continue;
        }

        if text == "U" {
            let _ = crate::base_system::self_update::check_for_updates(
                env!("CARGO_PKG_VERSION"),
                false,
            );
            continue;
        }

        println!(
            "Che do CLI cu da tat tinh nang tai sach moi.\nNeu muon them sach moi, hay dung TUI hoac Web UI; CLI cu chi giu lenh 'u' de cap nhat sach da co san.\n"
        );
    }

    Ok(())
}

fn read_line(prompt: &str) -> Result<String> {
    print!("{}", prompt);
    io::stdout().flush().ok();
    let stdin = io::stdin();
    let mut line = String::new();
    stdin.lock().read_line(&mut line)?;
    Ok(line)
}
