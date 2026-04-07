//! 无 UI 配置编辑器。
//!
//! 提供交互式菜单修改 `config.yml`。

use std::fs;
use std::path::Path;

use anyhow::{Context, Result, anyhow};

use crate::base_system::config::{ConfigSpec, write_with_comments};
use crate::base_system::context::Config;

#[derive(Debug, Clone, Copy)]
enum ConfigValueType {
    Bool,
    Int,
    Float,
    String,
    List,
    Selection,
}

#[derive(Debug, Clone, Copy)]
enum ConfigField {
    SavePath,
    NovelFormat,
    BulkFiles,
    AutoClearDump,
    AllowOverwriteFiles,
    PreferredBookNameField,
    EnableAudiobook,
    AudiobookVoice,
    AudiobookRate,
    AudiobookVolume,
    AudiobookPitch,
    AudiobookConcurrency,
    AudiobookFormat,
    MaxWorkers,
    RequestTimeout,
    MaxRetries,
    MinWaitTime,
    MaxWaitTime,
    MinConnectTimeout,
    UseOfficialApi,
    ApiEndpoints,
    EnableSegmentComments,
    SegmentCommentsTopN,
    SegmentCommentsWorkers,
    DownloadCommentImages,
    DownloadCommentAvatars,
    MediaDownloadWorkers,
    BlockedMediaDomains,
    ForceConvertImagesToJpeg,
    JpegRetryConvert,
    JpegQuality,
    ConvertHeicToJpeg,
    KeepHeicOriginal,
    MediaLimitPerChapter,
    MediaMaxDimensionPx,
    FirstLineIndentEm,
    OldCli,
}

#[derive(Debug, Clone, Copy)]
struct ConfigOption {
    name: &'static str,
    field: ConfigField,
    ty: ConfigValueType,
}

pub(super) fn show_config_menu(config: &mut Config) -> Result<()> {
    // 参照 old_main.py 的 option_defs 顺序
    const OPTS: &[ConfigOption] = &[
        ConfigOption {
            name: "Duong dan luu",
            field: ConfigField::SavePath,
            ty: ConfigValueType::String,
        },
        ConfigOption {
            name: "Dinh dang luu sach (txt/epub)",
            field: ConfigField::NovelFormat,
            ty: ConfigValueType::String,
        },
        ConfigOption {
            name: "Co luu sach theo dang file roi hay khong",
            field: ConfigField::BulkFiles,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Co tu dong don file cache hay khong",
            field: ConfigField::AutoClearDump,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Co cho phep ghi de file da ton tai hay khong",
            field: ConfigField::AllowOverwriteFiles,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Truong uu tien ten sach",
            field: ConfigField::PreferredBookNameField,
            ty: ConfigValueType::Selection,
        },
        ConfigOption {
            name: "Co tao sach noi hay khong",
            field: ConfigField::EnableAudiobook,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Giong doc sach noi",
            field: ConfigField::AudiobookVoice,
            ty: ConfigValueType::String,
        },
        ConfigOption {
            name: "Toc do doc sach noi (vd +0%)",
            field: ConfigField::AudiobookRate,
            ty: ConfigValueType::String,
        },
        ConfigOption {
            name: "Am luong sach noi (vd +0%)",
            field: ConfigField::AudiobookVolume,
            ty: ConfigValueType::String,
        },
        ConfigOption {
            name: "Cao do sach noi (vd +2Hz/-1st, co the de trong)",
            field: ConfigField::AudiobookPitch,
            ty: ConfigValueType::String,
        },
        ConfigOption {
            name: "So luong song song cho sach noi",
            field: ConfigField::AudiobookConcurrency,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Dinh dang sach noi (mp3/wav)",
            field: ConfigField::AudiobookFormat,
            ty: ConfigValueType::String,
        },
        ConfigOption {
            name: "So thread toi da",
            field: ConfigField::MaxWorkers,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Timeout request (giay)",
            field: ConfigField::RequestTimeout,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "So lan thu lai toi da",
            field: ConfigField::MaxRetries,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Thoi gian cho toi thieu (ms)",
            field: ConfigField::MinWaitTime,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Thoi gian cho toi da (ms)",
            field: ConfigField::MaxWaitTime,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Timeout ket noi toi thieu",
            field: ConfigField::MinConnectTimeout,
            ty: ConfigValueType::Float,
        },
        ConfigOption {
            name: "Co su dung API chinh thuc hay khong",
            field: ConfigField::UseOfficialApi,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Danh sach API tuy chinh (tach boi dau phay)",
            field: ConfigField::ApiEndpoints,
            ty: ConfigValueType::List,
        },
        ConfigOption {
            name: "Co tai binh luan doan hay khong",
            field: ConfigField::EnableSegmentComments,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "So binh luan toi da moi doan",
            field: ConfigField::SegmentCommentsTopN,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "So thread song song cho binh luan doan",
            field: ConfigField::SegmentCommentsWorkers,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Co tai anh trong phan binh luan hay khong",
            field: ConfigField::DownloadCommentImages,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Co tai avatar trong phan binh luan hay khong",
            field: ConfigField::DownloadCommentAvatars,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "So thread tai anh binh luan",
            field: ConfigField::MediaDownloadWorkers,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Blacklist domain anh (tach boi dau phay)",
            field: ConfigField::BlockedMediaDomains,
            ty: ConfigValueType::List,
        },
        ConfigOption {
            name: "Bat buoc chuyen tat ca anh sang JPEG",
            field: ConfigField::ForceConvertImagesToJpeg,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Thu chuyen sang JPEG voi anh khong phai JPEG",
            field: ConfigField::JpegRetryConvert,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Chat luong JPEG (0-100)",
            field: ConfigField::JpegQuality,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Chuyen HEIC sang JPEG",
            field: ConfigField::ConvertHeicToJpeg,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Giu lai file HEIC goc",
            field: ConfigField::KeepHeicOriginal,
            ty: ConfigValueType::Bool,
        },
        ConfigOption {
            name: "Gioi han so media moi chuong (0 = khong gioi han)",
            field: ConfigField::MediaLimitPerChapter,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Gioi han canh dai nhat cua anh theo pixel (>0 moi co hieu luc)",
            field: ConfigField::MediaMaxDimensionPx,
            ty: ConfigValueType::Int,
        },
        ConfigOption {
            name: "Thut le dong dau EPUB (em)",
            field: ConfigField::FirstLineIndentEm,
            ty: ConfigValueType::Float,
        },
        ConfigOption {
            name: "Co dung giao dien CLI cu hay khong (can khoi dong lai)",
            field: ConfigField::OldCli,
            ty: ConfigValueType::Bool,
        },
    ];

    loop {
        println!("\n=== Tuy chon cau hinh ===");
        for (idx, opt) in OPTS.iter().enumerate() {
            let mut name = opt.name.to_string();
            if matches!(opt.field, ConfigField::EnableSegmentComments)
                && config.novel_format.eq_ignore_ascii_case("txt")
            {
                name.push_str(" (TXT khong ho tro)");
            }
            println!(
                "{}. {}: {}",
                idx + 1,
                name,
                config_value_display(config, opt.field)
            );
        }
        println!("0. Quay ve menu chinh");

        let choice = super::read_line("\nHay chon so thu tu muc cau hinh can sua: ")?;
        let choice = choice.trim();
        if choice == "0" {
            break;
        }
        let Ok(idx) = choice.parse::<usize>() else {
            println!("Vui long nhap so thu tu");
            continue;
        };
        if idx == 0 || idx > OPTS.len() {
            println!("So thu tu vuot qua pham vi");
            continue;
        }
        let opt = OPTS[idx - 1];
        let cur = config_value_display(config, opt.field);

        let new_text = if matches!(opt.ty, ConfigValueType::Selection) {
            // 选项模式：列出可选值让用户选择
            match show_selection_prompt(opt.field, &cur)? {
                Some(v) => v,
                None => {
                    println!("Da huy chinh sua");
                    continue;
                }
            }
        } else {
            let input = super::read_line(&format!(
                "Hien tai {} = {}\nNhap gia tri moi (de trong de huy): ",
                opt.name, cur
            ))?;
            let trimmed = input.trim().to_string();
            if trimmed.is_empty() {
                println!("Da huy chinh sua");
                continue;
            }
            trimmed
        };

        apply_config_edit(config, opt, &new_text)?;

        // 持久化到 config.yml
        write_with_comments(config, Path::new(<Config as ConfigSpec>::FILE_NAME))
            .map_err(|e| anyhow!(e.to_string()))?;
        println!(
            "Da cap nhat {} = {}",
            opt.name,
            config_value_display(config, opt.field)
        );
    }

    Ok(())
}

fn config_value_display(config: &Config, field: ConfigField) -> String {
    match field {
        ConfigField::SavePath => config.save_path.clone(),
        ConfigField::NovelFormat => config.novel_format.clone(),
        ConfigField::BulkFiles => config.bulk_files.to_string(),
        ConfigField::AutoClearDump => config.auto_clear_dump.to_string(),
        ConfigField::AllowOverwriteFiles => config.allow_overwrite_files.to_string(),
        ConfigField::PreferredBookNameField => {
            book_name_field_to_chinese(&config.preferred_book_name_field).to_string()
        }
        ConfigField::EnableAudiobook => config.enable_audiobook.to_string(),
        ConfigField::AudiobookVoice => config.audiobook_voice.clone(),
        ConfigField::AudiobookRate => config.audiobook_rate.clone(),
        ConfigField::AudiobookVolume => config.audiobook_volume.clone(),
        ConfigField::AudiobookPitch => config.audiobook_pitch.clone(),
        ConfigField::AudiobookConcurrency => config.audiobook_concurrency.to_string(),
        ConfigField::AudiobookFormat => config.audiobook_format.clone(),
        ConfigField::MaxWorkers => config.max_workers.to_string(),
        ConfigField::RequestTimeout => config.request_timeout.to_string(),
        ConfigField::MaxRetries => config.max_retries.to_string(),
        ConfigField::MinWaitTime => config.min_wait_time.to_string(),
        ConfigField::MaxWaitTime => config.max_wait_time.to_string(),
        ConfigField::MinConnectTimeout => config.min_connect_timeout.to_string(),
        ConfigField::UseOfficialApi => config.use_official_api.to_string(),
        ConfigField::ApiEndpoints => config.api_endpoints.join(","),
        ConfigField::EnableSegmentComments => config.enable_segment_comments.to_string(),
        ConfigField::SegmentCommentsTopN => config.segment_comments_top_n.to_string(),
        ConfigField::SegmentCommentsWorkers => config.segment_comments_workers.to_string(),
        ConfigField::DownloadCommentImages => config.download_comment_images.to_string(),
        ConfigField::DownloadCommentAvatars => config.download_comment_avatars.to_string(),
        ConfigField::MediaDownloadWorkers => config.media_download_workers.to_string(),
        ConfigField::BlockedMediaDomains => config.blocked_media_domains.join(","),
        ConfigField::ForceConvertImagesToJpeg => config.force_convert_images_to_jpeg.to_string(),
        ConfigField::JpegRetryConvert => config.jpeg_retry_convert.to_string(),
        ConfigField::JpegQuality => config.jpeg_quality.to_string(),
        ConfigField::ConvertHeicToJpeg => config.convert_heic_to_jpeg.to_string(),
        ConfigField::KeepHeicOriginal => config.keep_heic_original.to_string(),
        ConfigField::MediaLimitPerChapter => config.media_limit_per_chapter.to_string(),
        ConfigField::MediaMaxDimensionPx => config.media_max_dimension_px.to_string(),
        ConfigField::FirstLineIndentEm => config.first_line_indent_em.to_string(),
        ConfigField::OldCli => config.old_cli.to_string(),
    }
}

fn apply_config_edit(config: &mut Config, opt: ConfigOption, text: &str) -> Result<()> {
    match opt.ty {
        ConfigValueType::Bool => {
            let v = matches!(
                text.to_ascii_lowercase().as_str(),
                "true" | "1" | "yes" | "y"
            );
            set_bool(config, opt.field, v)?;
        }
        ConfigValueType::Int => {
            let v: i64 = text
                .parse()
                .map_err(|_| anyhow!("Chuyen kieu that bai: can so nguyen"))?;
            set_int(config, opt.field, v)?;
        }
        ConfigValueType::Float => {
            let v: f64 = text
                .parse()
                .map_err(|_| anyhow!("Chuyen kieu that bai: can so thap phan"))?;
            set_float(config, opt.field, v)?;
        }
        ConfigValueType::String => {
            set_string(config, opt.field, text)?;
        }
        ConfigValueType::Selection => {
            set_string(config, opt.field, text)?;
        }
        ConfigValueType::List => {
            let parts: Vec<String> = text
                .split([',', '\n'])
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect();
            set_list(config, opt.field, parts)?;
        }
    }
    Ok(())
}

fn set_bool(config: &mut Config, field: ConfigField, v: bool) -> Result<()> {
    match field {
        ConfigField::BulkFiles => config.bulk_files = v,
        ConfigField::AutoClearDump => config.auto_clear_dump = v,
        ConfigField::AllowOverwriteFiles => config.allow_overwrite_files = v,
        ConfigField::EnableAudiobook => config.enable_audiobook = v,
        ConfigField::UseOfficialApi => config.use_official_api = v,
        ConfigField::EnableSegmentComments => {
            if v && config.novel_format.eq_ignore_ascii_case("txt") {
                config.novel_format = "epub".to_string();
                println!("Da tu dong doi dinh dang luu sang EPUB de bat tinh nang binh luan doan.");
            }
            config.enable_segment_comments = v;
        }
        ConfigField::DownloadCommentImages => config.download_comment_images = v,
        ConfigField::DownloadCommentAvatars => config.download_comment_avatars = v,
        ConfigField::ForceConvertImagesToJpeg => config.force_convert_images_to_jpeg = v,
        ConfigField::JpegRetryConvert => config.jpeg_retry_convert = v,
        ConfigField::ConvertHeicToJpeg => config.convert_heic_to_jpeg = v,
        ConfigField::KeepHeicOriginal => config.keep_heic_original = v,
        ConfigField::OldCli => config.old_cli = v,
        _ => return Err(anyhow!("Truong nay khong phai bool")),
    }
    Ok(())
}

fn set_int(config: &mut Config, field: ConfigField, v: i64) -> Result<()> {
    match field {
        ConfigField::MaxWorkers => {
            if v <= 0 {
                return Err(anyhow!("So thread toi da phai lon hon 0"));
            }
            config.max_workers = v as usize;
        }
        ConfigField::RequestTimeout => {
            if v <= 0 {
                return Err(anyhow!("Timeout request phai lon hon 0"));
            }
            config.request_timeout = v as u64;
        }
        ConfigField::MaxRetries => {
            if v < 0 {
                return Err(anyhow!("So lan thu lai toi da khong duoc am"));
            }
            config.max_retries = v as u32;
        }
        ConfigField::MinWaitTime => {
            if v < 0 {
                return Err(anyhow!("Thoi gian cho toi thieu khong duoc am"));
            }
            let v = v as u64;
            if v > config.max_wait_time {
                return Err(anyhow!("Thoi gian cho toi thieu khong duoc vuot qua thoi gian cho toi da"));
            }
            config.min_wait_time = v;
        }
        ConfigField::MaxWaitTime => {
            if v < 0 {
                return Err(anyhow!("Thoi gian cho toi da khong duoc am"));
            }
            let v = v as u64;
            if v < config.min_wait_time {
                return Err(anyhow!("Thoi gian cho toi da khong duoc nho hon thoi gian cho toi thieu"));
            }
            config.max_wait_time = v;
        }
        ConfigField::AudiobookConcurrency => {
            if v <= 0 {
                return Err(anyhow!("Do song song cua sach noi phai lon hon 0"));
            }
            config.audiobook_concurrency = v as usize;
        }
        ConfigField::SegmentCommentsTopN => {
            if v <= 0 {
                return Err(anyhow!("Gioi han so binh luan doan phai lon hon 0"));
            }
            config.segment_comments_top_n = v as usize;
        }
        ConfigField::SegmentCommentsWorkers => {
            if v <= 0 {
                return Err(anyhow!("So thread binh luan doan phai lon hon 0"));
            }
            config.segment_comments_workers = v as usize;
        }
        ConfigField::MediaDownloadWorkers => {
            if v <= 0 {
                return Err(anyhow!("So thread media phai lon hon 0"));
            }
            config.media_download_workers = v as usize;
        }
        ConfigField::JpegQuality => {
            if !(0..=100).contains(&v) {
                return Err(anyhow!("Chat luong JPEG phai nam trong khoang 0-100"));
            }
            config.jpeg_quality = v as u8;
        }
        ConfigField::MediaLimitPerChapter => {
            if v < 0 {
                return Err(anyhow!("Gioi han media moi chuong khong duoc am"));
            }
            config.media_limit_per_chapter = v as usize;
        }
        ConfigField::MediaMaxDimensionPx => {
            if v < 0 {
                return Err(anyhow!("Gioi han canh dai nhat cua anh theo pixel khong duoc am"));
            }
            config.media_max_dimension_px = v as u32;
        }
        _ => return Err(anyhow!("Truong nay khong phai int")),
    }
    Ok(())
}

fn set_float(config: &mut Config, field: ConfigField, v: f64) -> Result<()> {
    match field {
        ConfigField::MinConnectTimeout => {
            if v <= 0.0 {
                return Err(anyhow!("Timeout ket noi toi thieu phai lon hon 0"));
            }
            config.min_connect_timeout = v;
        }
        ConfigField::FirstLineIndentEm => {
            if v < 0.0 {
                return Err(anyhow!("Do thut le khong duoc am"));
            }
            config.first_line_indent_em = v as f32;
        }
        _ => return Err(anyhow!("Truong nay khong phai float")),
    }
    Ok(())
}

fn set_string(config: &mut Config, field: ConfigField, v: &str) -> Result<()> {
    match field {
        ConfigField::SavePath => {
            let p = v.trim();
            if p.is_empty() {
                return Err(anyhow!("Duong dan luu khong duoc de trong"));
            }
            fs::create_dir_all(p).with_context(|| format!("Tao thu muc that bai: {}", p))?;
            config.save_path = p.to_string();
        }
        ConfigField::NovelFormat => {
            let lower = v.trim().to_ascii_lowercase();
            if lower != "txt" && lower != "epub" {
                return Err(anyhow!("Dinh dang luu chi ho tro txt/epub"));
            }
            if lower == "txt" && config.enable_segment_comments {
                config.enable_segment_comments = false;
                println!("Da tu dong tat binh luan doan de tuong thich dinh dang TXT.");
            }
            config.novel_format = lower;
        }
        ConfigField::AudiobookVoice => config.audiobook_voice = v.to_string(),
        ConfigField::AudiobookRate => config.audiobook_rate = v.to_string(),
        ConfigField::AudiobookVolume => config.audiobook_volume = v.to_string(),
        ConfigField::AudiobookPitch => config.audiobook_pitch = v.to_string(),
        ConfigField::AudiobookFormat => {
            let lower = v.trim().to_ascii_lowercase();
            if lower != "mp3" && lower != "wav" {
                return Err(anyhow!("Dinh dang sach noi chi ho tro mp3/wav"));
            }
            config.audiobook_format = lower;
        }
        ConfigField::PreferredBookNameField => {
            // 尝试从中文转换，如果失败则尝试直接使用英文
            let field_name = if let Some(english) = chinese_to_book_name_field(v.trim()) {
                english
            } else {
                // 如果不是中文，检查是否是有效的英文字段名
                let lower = v.trim().to_ascii_lowercase();
                if lower == "book_name"
                    || lower == "original_book_name"
                    || lower == "book_short_name"
                    || lower == "ask_after_download"
                {
                    lower
                } else {
                    return Err(anyhow!(
                        "Truong uu tien ten sach chi ho tro: Ten sach mac dinh, Ten sach goc, Ten sach ngan, Chon sau khi tai xong"
                    ));
                }
            };
            config.preferred_book_name_field = field_name;
        }
        _ => return Err(anyhow!("Truong nay khong phai string")),
    }
    Ok(())
}

fn set_list(config: &mut Config, field: ConfigField, v: Vec<String>) -> Result<()> {
    match field {
        ConfigField::ApiEndpoints => config.api_endpoints = v,
        ConfigField::BlockedMediaDomains => config.blocked_media_domains = v,
        _ => return Err(anyhow!("Truong nay khong phai list")),
    }
    Ok(())
}

/// 将书名字段的英文名转换为中文显示名
fn book_name_field_to_chinese(field: &str) -> &'static str {
    match field {
        "book_name" => "Ten sach mac dinh",
        "original_book_name" => "Ten sach goc",
        "book_short_name" => "Ten sach ngan",
        "ask_after_download" => "Chon sau khi tai xong",
        _ => "Ten sach mac dinh",
    }
}

/// 将中文显示名转换为书名字段的英文名
fn chinese_to_book_name_field(chinese: &str) -> Option<String> {
    match chinese {
        "Ten sach mac dinh" => Some("book_name".to_string()),
        "Ten sach goc" => Some("original_book_name".to_string()),
        "Ten sach ngan" => Some("book_short_name".to_string()),
        "Chon sau khi tai xong" => Some("ask_after_download".to_string()),
        _ => None,
    }
}

/// 选项模式：展示可选项让用户选择编号
fn show_selection_prompt(field: ConfigField, current: &str) -> Result<Option<String>> {
    match field {
        ConfigField::PreferredBookNameField => {
            const OPTIONS: &[(&str, &str)] = &[
                ("Ten sach mac dinh", "book_name"),
                ("Ten sach goc", "original_book_name"),
                ("Ten sach ngan", "book_short_name"),
                ("Chon sau khi tai xong", "ask_after_download"),
            ];
            println!("\nHien tai: {}", current);
            for (idx, (label, _)) in OPTIONS.iter().enumerate() {
                println!("  {}. {}", idx + 1, label);
            }
            println!("  0. Huy");
            let choice = super::read_line("Hay chon: ")?;
            let choice = choice.trim();
            if choice == "0" || choice.is_empty() {
                return Ok(None);
            }
            let Ok(idx) = choice.parse::<usize>() else {
                println!("Vui long nhap so thu tu");
                return Ok(None);
            };
            if idx == 0 || idx > OPTIONS.len() {
                println!("So thu tu vuot qua pham vi");
                return Ok(None);
            }
            Ok(Some(OPTIONS[idx - 1].0.to_string()))
        }
        _ => Ok(None),
    }
}
