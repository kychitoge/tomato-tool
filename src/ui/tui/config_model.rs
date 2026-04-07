//! TUI 配置模型与编辑逻辑。
//!
//! 将 `Config` 映射为可展示/可编辑的字段列表，并负责写回 `config.yml`。

use std::path::Path;

use anyhow::{Result, anyhow};

use crate::base_system::config::{ConfigSpec, write_with_comments};
use crate::base_system::context::Config;

use super::App;

#[derive(Debug, Clone, Copy)]
pub(in crate::ui) enum ConfigField {
    SavePath,
    NovelFormat,
    BulkFiles,
    AutoClearDump,
    AutoOpenDownloadedFiles,
    AllowOverwriteFiles,
    PreferredBookNameField,
    OldCli,
    FirstLineIndentEm,
    EnableSegmentComments,
    UseOfficialApi,
    ApiEndpoints,
    MaxWorkers,
    RequestTimeout,
    MaxRetries,
    MinConnectTimeout,
    MinWait,
    MaxWait,
    EnableAudiobook,
    AudiobookVoice,
    AudiobookRate,
    AudiobookVolume,
    AudiobookPitch,
    AudiobookFormat,
    AudiobookConcurrency,
    AudiobookTtsProvider,
    AudiobookTtsApiUrl,
    AudiobookTtsApiToken,
    AudiobookTtsModel,
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
}

#[derive(Debug, Clone)]
pub(in crate::ui) struct ConfigEntry {
    pub(in crate::ui) title: &'static str,
    pub(in crate::ui) field: ConfigField,
}

#[derive(Debug, Clone)]
pub(in crate::ui) struct ConfigCategory {
    pub(in crate::ui) title: &'static str,
    pub(in crate::ui) entries: Vec<ConfigEntry>,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::ui) struct VoicePreset {
    pub(in crate::ui) name: &'static str,
    pub(in crate::ui) label: &'static str,
}

pub(in crate::ui) const AUDIOBOOK_VOICE_PRESETS: &[VoicePreset] = &[
    VoicePreset {
        name: "zh-CN-XiaoxiaoNeural",
        label: "zh-CN-XiaoxiaoNeural (nu)",
    },
    VoicePreset {
        name: "zh-CN-XiaoyiNeural",
        label: "zh-CN-XiaoyiNeural (nu)",
    },
    VoicePreset {
        name: "zh-CN-YunjianNeural",
        label: "zh-CN-YunjianNeural (nam)",
    },
    VoicePreset {
        name: "zh-CN-YunxiNeural",
        label: "zh-CN-YunxiNeural (nam)",
    },
    VoicePreset {
        name: "zh-CN-YunxiaNeural",
        label: "zh-CN-YunxiaNeural (nam)",
    },
    VoicePreset {
        name: "zh-CN-YunyangNeural",
        label: "zh-CN-YunyangNeural (nam)",
    },
    VoicePreset {
        name: "zh-CN-liaoning-XiaobeiNeural",
        label: "zh-CN-liaoning-XiaobeiNeural (nu)",
    },
    VoicePreset {
        name: "zh-CN-shaanxi-XiaoniNeural",
        label: "zh-CN-shaanxi-XiaoniNeural (nu)",
    },
    VoicePreset {
        name: "zh-HK-HiuGaaiNeural",
        label: "zh-HK-HiuGaaiNeural (nu)",
    },
    VoicePreset {
        name: "zh-HK-HiuMaanNeural",
        label: "zh-HK-HiuMaanNeural (nu)",
    },
    VoicePreset {
        name: "zh-HK-WanLungNeural",
        label: "zh-HK-WanLungNeural (nam)",
    },
    VoicePreset {
        name: "zh-TW-HsiaoChenNeural",
        label: "zh-TW-HsiaoChenNeural (nu)",
    },
];

pub(in crate::ui) const BOOK_NAME_FIELD_PRESETS: &[VoicePreset] = &[
    VoicePreset {
        name: "book_name",
        label: "Ten sach mac dinh",
    },
    VoicePreset {
        name: "original_book_name",
        label: "Ten sach goc",
    },
    VoicePreset {
        name: "book_short_name",
        label: "Ten sach ngan",
    },
    VoicePreset {
        name: "ask_after_download",
        label: "Chon sau khi tai xong",
    },
];

pub(in crate::ui) const NOVEL_FORMAT_PRESETS: &[VoicePreset] = &[
    VoicePreset {
        name: "txt",
        label: "Dinh dang txt",
    },
    VoicePreset {
        name: "epub",
        label: "Dinh dang epub",
    },
    VoicePreset {
        name: "ask_after_download",
        label: "Chon sau khi tai xong",
    },
];

pub(in crate::ui) fn cfg_field_is_combo(field: ConfigField) -> bool {
    matches!(
        field,
        ConfigField::AudiobookVoice
            | ConfigField::PreferredBookNameField
            | ConfigField::NovelFormat
    )
}

pub(in crate::ui) fn cfg_combo_presets(field: ConfigField) -> Option<&'static [VoicePreset]> {
    match field {
        ConfigField::AudiobookVoice => Some(AUDIOBOOK_VOICE_PRESETS),
        ConfigField::PreferredBookNameField => Some(BOOK_NAME_FIELD_PRESETS),
        ConfigField::NovelFormat => Some(NOVEL_FORMAT_PRESETS),
        _ => None,
    }
}

pub(in crate::ui) fn build_config_categories() -> Vec<ConfigCategory> {
    vec![
        ConfigCategory {
            title: "Co ban va dinh dang",
            entries: vec![
                ConfigEntry {
                    title: "Duong dan luu",
                    field: ConfigField::SavePath,
                },
                ConfigEntry {
                    title: "Dinh dang sach",
                    field: ConfigField::NovelFormat,
                },
                ConfigEntry {
                    title: "Thut le dong dau (em)",
                    field: ConfigField::FirstLineIndentEm,
                },
                ConfigEntry {
                    title: "Luu file roi",
                    field: ConfigField::BulkFiles,
                },
                ConfigEntry {
                    title: "Tu dong don cache",
                    field: ConfigField::AutoClearDump,
                },
                ConfigEntry {
                    title: "Tu dong mo sau khi tai xong",
                    field: ConfigField::AutoOpenDownloadedFiles,
                },
                ConfigEntry {
                    title: "Cho phep ghi de file da ton tai",
                    field: ConfigField::AllowOverwriteFiles,
                },
                ConfigEntry {
                    title: "Truong uu tien ten sach",
                    field: ConfigField::PreferredBookNameField,
                },
                ConfigEntry {
                    title: "CLI UI phien ban cu",
                    field: ConfigField::OldCli,
                },
            ],
        },
        ConfigCategory {
            title: "Mang va dieu phoi",
            entries: vec![
                ConfigEntry {
                    title: "So luong thread toi da",
                    field: ConfigField::MaxWorkers,
                },
                ConfigEntry {
                    title: "Timeout request (s)",
                    field: ConfigField::RequestTimeout,
                },
                ConfigEntry {
                    title: "So lan thu lai toi da",
                    field: ConfigField::MaxRetries,
                },
                ConfigEntry {
                    title: "Timeout ket noi toi thieu (s)",
                    field: ConfigField::MinConnectTimeout,
                },
                ConfigEntry {
                    title: "Thoi gian cho toi thieu (ms)",
                    field: ConfigField::MinWait,
                },
                ConfigEntry {
                    title: "Thoi gian cho toi da (ms)",
                    field: ConfigField::MaxWait,
                },
            ],
        },
        ConfigCategory {
            title: "API",
            entries: vec![
                ConfigEntry {
                    title: "Su dung API chinh thuc",
                    field: ConfigField::UseOfficialApi,
                },
                ConfigEntry {
                    title: "Danh sach API (tach boi dau phay)",
                    field: ConfigField::ApiEndpoints,
                },
            ],
        },
        ConfigCategory {
            title: "Binh luan doan",
            entries: vec![
                ConfigEntry {
                    title: "Bat binh luan doan",
                    field: ConfigField::EnableSegmentComments,
                },
                ConfigEntry {
                    title: "Gioi han so binh luan moi doan",
                    field: ConfigField::SegmentCommentsTopN,
                },
                ConfigEntry {
                    title: "So thread song song cho binh luan doan",
                    field: ConfigField::SegmentCommentsWorkers,
                },
            ],
        },
        ConfigCategory {
            title: "Tai media",
            entries: vec![
                ConfigEntry {
                    title: "Tai anh binh luan",
                    field: ConfigField::DownloadCommentImages,
                },
                ConfigEntry {
                    title: "Tai avatar binh luan",
                    field: ConfigField::DownloadCommentAvatars,
                },
                ConfigEntry {
                    title: "So thread tai media",
                    field: ConfigField::MediaDownloadWorkers,
                },
                ConfigEntry {
                    title: "Domain anh bi chan",
                    field: ConfigField::BlockedMediaDomains,
                },
                ConfigEntry {
                    title: "Bat buoc chuyen sang JPEG",
                    field: ConfigField::ForceConvertImagesToJpeg,
                },
                ConfigEntry {
                    title: "Neu that bai thi thu lai roi chuyen JPEG",
                    field: ConfigField::JpegRetryConvert,
                },
                ConfigEntry {
                    title: "Chat luong JPEG (0-100)",
                    field: ConfigField::JpegQuality,
                },
                ConfigEntry {
                    title: "Chuyen HEIC sang JPEG",
                    field: ConfigField::ConvertHeicToJpeg,
                },
                ConfigEntry {
                    title: "Giu lai HEIC goc",
                    field: ConfigField::KeepHeicOriginal,
                },
                ConfigEntry {
                    title: "Gioi han media moi chuong",
                    field: ConfigField::MediaLimitPerChapter,
                },
                ConfigEntry {
                    title: "Kich thuoc media toi da (px)",
                    field: ConfigField::MediaMaxDimensionPx,
                },
            ],
        },
        ConfigCategory {
            title: "Sach noi",
            entries: vec![
                ConfigEntry {
                    title: "Bat sach noi",
                    field: ConfigField::EnableAudiobook,
                },
                ConfigEntry {
                    title: "Giong doc",
                    field: ConfigField::AudiobookVoice,
                },
                ConfigEntry {
                    title: "Loai dich vu TTS (edge/third_party)",
                    field: ConfigField::AudiobookTtsProvider,
                },
                ConfigEntry {
                    title: "Dia chi API TTS ben thu ba",
                    field: ConfigField::AudiobookTtsApiUrl,
                },
                ConfigEntry {
                    title: "Token TTS ben thu ba",
                    field: ConfigField::AudiobookTtsApiToken,
                },
                ConfigEntry {
                    title: "Model TTS ben thu ba",
                    field: ConfigField::AudiobookTtsModel,
                },
                ConfigEntry {
                    title: "Dieu chinh toc do noi",
                    field: ConfigField::AudiobookRate,
                },
                ConfigEntry {
                    title: "Dieu chinh am luong",
                    field: ConfigField::AudiobookVolume,
                },
                ConfigEntry {
                    title: "Dieu chinh cao do",
                    field: ConfigField::AudiobookPitch,
                },
                ConfigEntry {
                    title: "Dinh dang xuat (mp3/wav)",
                    field: ConfigField::AudiobookFormat,
                },
                ConfigEntry {
                    title: "So chuong tao song song",
                    field: ConfigField::AudiobookConcurrency,
                },
            ],
        },
    ]
}

pub(in crate::ui) fn current_cfg_value(app: &App, field: ConfigField) -> String {
    match field {
        ConfigField::SavePath => app.config.save_path.clone(),
        ConfigField::NovelFormat => {
            if app.config.ask_format_after_download {
                novel_format_to_chinese("ask_after_download").to_string()
            } else {
                novel_format_to_chinese(&app.config.novel_format).to_string()
            }
        }
        ConfigField::FirstLineIndentEm => format!("{:.2}", app.config.first_line_indent_em),
        ConfigField::BulkFiles => app.config.bulk_files.to_string(),
        ConfigField::AutoClearDump => app.config.auto_clear_dump.to_string(),
        ConfigField::AutoOpenDownloadedFiles => app.config.auto_open_downloaded_files.to_string(),
        ConfigField::AllowOverwriteFiles => app.config.allow_overwrite_files.to_string(),
        ConfigField::PreferredBookNameField => {
            book_name_field_to_chinese(&app.config.preferred_book_name_field).to_string()
        }
        ConfigField::OldCli => app.config.old_cli.to_string(),
        ConfigField::EnableSegmentComments => app.config.enable_segment_comments.to_string(),
        ConfigField::UseOfficialApi => app.config.use_official_api.to_string(),
        ConfigField::ApiEndpoints => app.config.api_endpoints.join(","),
        ConfigField::MaxWorkers => app.config.max_workers.to_string(),
        ConfigField::RequestTimeout => app.config.request_timeout.to_string(),
        ConfigField::MaxRetries => app.config.max_retries.to_string(),
        ConfigField::MinConnectTimeout => format!("{:.2}", app.config.min_connect_timeout),
        ConfigField::MinWait => app.config.min_wait_time.to_string(),
        ConfigField::MaxWait => app.config.max_wait_time.to_string(),
        ConfigField::EnableAudiobook => app.config.enable_audiobook.to_string(),
        ConfigField::AudiobookVoice => app.config.audiobook_voice.clone(),
        ConfigField::AudiobookRate => app.config.audiobook_rate.clone(),
        ConfigField::AudiobookVolume => app.config.audiobook_volume.clone(),
        ConfigField::AudiobookPitch => app.config.audiobook_pitch.clone(),
        ConfigField::AudiobookFormat => app.config.audiobook_format.clone(),
        ConfigField::AudiobookConcurrency => app.config.audiobook_concurrency.to_string(),
        ConfigField::AudiobookTtsProvider => app.config.audiobook_tts_provider.clone(),
        ConfigField::AudiobookTtsApiUrl => app.config.audiobook_tts_api_url.clone(),
        ConfigField::AudiobookTtsApiToken => app.config.audiobook_tts_api_token.clone(),
        ConfigField::AudiobookTtsModel => app.config.audiobook_tts_model.clone(),
        ConfigField::SegmentCommentsTopN => app.config.segment_comments_top_n.to_string(),
        ConfigField::SegmentCommentsWorkers => app.config.segment_comments_workers.to_string(),
        ConfigField::DownloadCommentImages => app.config.download_comment_images.to_string(),
        ConfigField::DownloadCommentAvatars => app.config.download_comment_avatars.to_string(),
        ConfigField::MediaDownloadWorkers => app.config.media_download_workers.to_string(),
        ConfigField::BlockedMediaDomains => app.config.blocked_media_domains.join(","),
        ConfigField::ForceConvertImagesToJpeg => {
            app.config.force_convert_images_to_jpeg.to_string()
        }
        ConfigField::JpegRetryConvert => app.config.jpeg_retry_convert.to_string(),
        ConfigField::JpegQuality => app.config.jpeg_quality.to_string(),
        ConfigField::ConvertHeicToJpeg => app.config.convert_heic_to_jpeg.to_string(),
        ConfigField::KeepHeicOriginal => app.config.keep_heic_original.to_string(),
        ConfigField::MediaLimitPerChapter => app.config.media_limit_per_chapter.to_string(),
        ConfigField::MediaMaxDimensionPx => app.config.media_max_dimension_px.to_string(),
    }
}

pub(in crate::ui) fn cfg_field_is_bool(field: ConfigField) -> bool {
    matches!(
        field,
        ConfigField::BulkFiles
            | ConfigField::AutoClearDump
            | ConfigField::AutoOpenDownloadedFiles
            | ConfigField::AllowOverwriteFiles
            | ConfigField::OldCli
            | ConfigField::EnableSegmentComments
            | ConfigField::UseOfficialApi
            | ConfigField::EnableAudiobook
            | ConfigField::DownloadCommentImages
            | ConfigField::DownloadCommentAvatars
            | ConfigField::ForceConvertImagesToJpeg
            | ConfigField::JpegRetryConvert
            | ConfigField::ConvertHeicToJpeg
            | ConfigField::KeepHeicOriginal
    )
}

fn cfg_field_current_bool(app: &App, field: ConfigField) -> Option<bool> {
    let val = match field {
        ConfigField::BulkFiles => app.config.bulk_files,
        ConfigField::AutoClearDump => app.config.auto_clear_dump,
        ConfigField::AutoOpenDownloadedFiles => app.config.auto_open_downloaded_files,
        ConfigField::AllowOverwriteFiles => app.config.allow_overwrite_files,
        ConfigField::OldCli => app.config.old_cli,
        ConfigField::EnableSegmentComments => app.config.enable_segment_comments,
        ConfigField::UseOfficialApi => app.config.use_official_api,
        ConfigField::EnableAudiobook => app.config.enable_audiobook,
        ConfigField::DownloadCommentImages => app.config.download_comment_images,
        ConfigField::DownloadCommentAvatars => app.config.download_comment_avatars,
        ConfigField::ForceConvertImagesToJpeg => app.config.force_convert_images_to_jpeg,
        ConfigField::JpegRetryConvert => app.config.jpeg_retry_convert,
        ConfigField::ConvertHeicToJpeg => app.config.convert_heic_to_jpeg,
        ConfigField::KeepHeicOriginal => app.config.keep_heic_original,
        _ => return None,
    };
    Some(val)
}

pub(in crate::ui) fn start_cfg_edit(app: &mut App) {
    let Some(cat_idx) = app.cfg_cat_state.selected() else {
        return;
    };
    let Some(entry_idx) = app.cfg_entry_state.selected() else {
        return;
    };
    let Some(category) = app.cfg_categories.get(cat_idx) else {
        return;
    };
    if entry_idx >= category.entries.len() {
        return;
    }
    let entry = &category.entries[entry_idx];
    app.cfg_editing = Some((cat_idx, entry_idx));
    app.cfg_edit_buffer = current_cfg_value(app, entry.field);
    if cfg_field_is_bool(entry.field) {
        let selected = match cfg_field_current_bool(app, entry.field) {
            Some(true) => Some(0),
            Some(false) => Some(1),
            None => Some(0),
        };
        app.cfg_bool_state.select(selected);
    }
    if cfg_field_is_combo(entry.field) {
        app.cfg_combo_focus = super::ConfigComboFocus::List;
        if let Some(presets) = cfg_combo_presets(entry.field) {
            let idx = presets
                .iter()
                .position(|p| p.name.eq_ignore_ascii_case(&app.cfg_edit_buffer))
                .or(Some(0));
            app.cfg_combo_state.select(idx);
        }
    }
    app.status = format!("Dang chinh sua [{}]: {}", category.title, entry.title);
}

pub(in crate::ui) fn apply_cfg_edit(app: &mut App, cat_idx: usize, entry_idx: usize) -> Result<()> {
    let Some(category) = app.cfg_categories.get(cat_idx) else {
        return Ok(());
    };
    if entry_idx >= category.entries.len() {
        return Ok(());
    }
    let field = category.entries[entry_idx].field;
    let entry_title = category.entries[entry_idx].title;
    let raw = app.cfg_edit_buffer.trim();

    let mut note: Option<String> = None;

    match field {
        ConfigField::SavePath => {
            app.config.save_path = raw.to_string();
        }
        ConfigField::NovelFormat => {
            let field_name = if let Some(english) = chinese_to_novel_format(raw) {
                english
            } else {
                let lower = raw.to_ascii_lowercase();
                if lower == "txt" || lower == "epub" || lower == "ask_after_download" {
                    lower
                } else {
                    app.status = "Hay chon: Dinh dang txt, Dinh dang epub hoac Chon sau khi tai xong".to_string();
                    return Ok(());
                }
            };
            if field_name == "ask_after_download" {
                app.config.ask_format_after_download = true;
            } else {
                app.config.ask_format_after_download = false;
                app.config.novel_format = field_name;
                if app.config.novel_format == "txt" && app.config.enable_segment_comments {
                    app.config.enable_segment_comments = false;
                    note = Some("Da tat binh luan doan de tuong thich txt".to_string());
                }
            }
        }
        ConfigField::FirstLineIndentEm => {
            let val: f32 = raw.parse().map_err(|_| anyhow!("Vui long nhap so"))?;
            if val.is_sign_negative() {
                app.status = "Do thut le khong duoc am".to_string();
                return Ok(());
            }
            app.config.first_line_indent_em = val;
        }
        ConfigField::BulkFiles => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.bulk_files = val;
        }
        ConfigField::AutoClearDump => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.auto_clear_dump = val;
        }
        ConfigField::AutoOpenDownloadedFiles => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.auto_open_downloaded_files = val;
        }
        ConfigField::AllowOverwriteFiles => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.allow_overwrite_files = val;
        }
        ConfigField::PreferredBookNameField => {
            // 尝试从中文转换，如果失败则尝试直接使用英文
            let field_name = if let Some(english) = chinese_to_book_name_field(raw) {
                english
            } else {
                // 如果不是中文，检查是否是有效的英文字段名
                let lower = raw.to_ascii_lowercase();
                if lower == "book_name"
                    || lower == "original_book_name"
                    || lower == "book_short_name"
                    || lower == "ask_after_download"
                {
                    lower
                } else {
                    app.status = "Hay chon: Ten sach mac dinh, Ten sach goc, Ten sach ngan hoac Chon sau khi tai xong".to_string();
                    return Ok(());
                }
            };
            app.config.preferred_book_name_field = field_name;
        }
        ConfigField::OldCli => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.old_cli = val;
        }
        ConfigField::EnableSegmentComments => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            if val && !app.config.novel_format.eq_ignore_ascii_case("epub") {
                app.status = "Binh luan doan chi ho tro epub, hay doi dinh dang sang epub truoc".to_string();
                return Ok(());
            }
            app.config.enable_segment_comments = val;
        }
        ConfigField::UseOfficialApi => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.use_official_api = val;
        }
        ConfigField::ApiEndpoints => {
            let list = parse_string_list(raw);
            app.config.api_endpoints = list;
        }
        ConfigField::MaxWorkers => {
            let val: usize = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen duong"))?;
            if val == 0 {
                app.status = "So thread toi da phai lon hon 0".to_string();
                return Ok(());
            }
            app.config.max_workers = val;
        }
        ConfigField::RequestTimeout => {
            let val: u64 = raw.parse().map_err(|_| anyhow!("Vui long nhap so giay"))?;
            if val == 0 {
                app.status = "Thoi gian timeout phai lon hon 0".to_string();
                return Ok(());
            }
            app.config.request_timeout = val;
        }
        ConfigField::MaxRetries => {
            let val: u32 = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen"))?;
            app.config.max_retries = val;
        }
        ConfigField::MinConnectTimeout => {
            let val: f64 = raw.parse().map_err(|_| anyhow!("Vui long nhap so"))?;
            if val <= 0.0 {
                app.status = "Timeout ket noi phai lon hon 0".to_string();
                return Ok(());
            }
            app.config.min_connect_timeout = val;
        }
        ConfigField::MinWait => {
            let val: u64 = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen mili-giay"))?;
            if val > app.config.max_wait_time {
                app.status = "Thoi gian cho toi thieu khong duoc lon hon thoi gian cho toi da".to_string();
                return Ok(());
            }
            app.config.min_wait_time = val;
        }
        ConfigField::MaxWait => {
            let val: u64 = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen mili-giay"))?;
            if val < app.config.min_wait_time {
                app.status = "Thoi gian cho toi da phai khong nho hon thoi gian cho toi thieu".to_string();
                return Ok(());
            }
            app.config.max_wait_time = val;
        }
        ConfigField::EnableAudiobook => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.enable_audiobook = val;
        }
        ConfigField::AudiobookVoice => {
            app.config.audiobook_voice = raw.to_string();
        }
        ConfigField::AudiobookRate => {
            app.config.audiobook_rate = raw.to_string();
        }
        ConfigField::AudiobookVolume => {
            app.config.audiobook_volume = raw.to_string();
        }
        ConfigField::AudiobookPitch => {
            app.config.audiobook_pitch = raw.to_string();
        }
        ConfigField::AudiobookFormat => {
            let lower = raw.to_ascii_lowercase();
            if lower != "mp3" && lower != "wav" {
                app.status = "Dinh dang chi ho tro mp3 hoac wav".to_string();
                return Ok(());
            }
            app.config.audiobook_format = lower;
        }
        ConfigField::AudiobookConcurrency => {
            let val: usize = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen duong"))?;
            if val == 0 {
                app.status = "So chuong xu ly song song phai lon hon 0".to_string();
                return Ok(());
            }
            app.config.audiobook_concurrency = val;
        }
        ConfigField::AudiobookTtsProvider => {
            app.config.audiobook_tts_provider = raw.to_string();
        }
        ConfigField::AudiobookTtsApiUrl => {
            app.config.audiobook_tts_api_url = raw.to_string();
        }
        ConfigField::AudiobookTtsApiToken => {
            app.config.audiobook_tts_api_token = raw.to_string();
        }
        ConfigField::AudiobookTtsModel => {
            app.config.audiobook_tts_model = raw.to_string();
        }
        ConfigField::SegmentCommentsTopN => {
            let val: usize = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen"))?;
            if val == 0 {
                app.status = "Gioi han so binh luan phai lon hon 0".to_string();
                return Ok(());
            }
            app.config.segment_comments_top_n = val;
        }
        ConfigField::SegmentCommentsWorkers => {
            let val: usize = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen duong"))?;
            if val == 0 {
                app.status = "So thread binh luan doan phai lon hon 0".to_string();
                return Ok(());
            }
            app.config.segment_comments_workers = val;
        }
        ConfigField::DownloadCommentImages => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.download_comment_images = val;
        }
        ConfigField::DownloadCommentAvatars => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.download_comment_avatars = val;
        }
        ConfigField::MediaDownloadWorkers => {
            let val: usize = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen duong"))?;
            if val == 0 {
                app.status = "So thread media phai lon hon 0".to_string();
                return Ok(());
            }
            app.config.media_download_workers = val;
        }
        ConfigField::BlockedMediaDomains => {
            app.config.blocked_media_domains = parse_string_list(raw);
        }
        ConfigField::ForceConvertImagesToJpeg => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.force_convert_images_to_jpeg = val;
        }
        ConfigField::JpegRetryConvert => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.jpeg_retry_convert = val;
        }
        ConfigField::JpegQuality => {
            let val: u8 = raw
                .parse()
                .map_err(|_| anyhow!("Vui long nhap so nguyen trong khoang 0-100"))?;
            if val > 100 {
                app.status = "Chat luong JPEG phai trong khoang 0-100".to_string();
                return Ok(());
            }
            app.config.jpeg_quality = val;
        }
        ConfigField::ConvertHeicToJpeg => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.convert_heic_to_jpeg = val;
        }
        ConfigField::KeepHeicOriginal => {
            let val = parse_bool(raw).ok_or_else(|| anyhow!("Vui long nhap true/false"))?;
            app.config.keep_heic_original = val;
        }
        ConfigField::MediaLimitPerChapter => {
            let val: usize = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen"))?;
            app.config.media_limit_per_chapter = val;
        }
        ConfigField::MediaMaxDimensionPx => {
            let val: u32 = raw.parse().map_err(|_| anyhow!("Vui long nhap so nguyen"))?;
            app.config.media_max_dimension_px = val;
        }
    }

    let path = Path::new(Config::FILE_NAME);
    write_with_comments(&app.config, path).map_err(|e| anyhow!(e.to_string()))?;
    match note {
        Some(extra) => app.status = format!("Da luu: {} ({})", entry_title, extra),
        None => app.status = format!("Da luu: {}", entry_title),
    }
    Ok(())
}

fn parse_bool(input: &str) -> Option<bool> {
    match input.to_ascii_lowercase().as_str() {
        "true" | "1" | "yes" | "y" | "on" => Some(true),
        "false" | "0" | "no" | "n" | "off" => Some(false),
        _ => None,
    }
}

fn parse_string_list(input: &str) -> Vec<String> {
    input
        .split([',', ';', '\n'])
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect()
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

/// 将小说格式英文名转换为中文显示名
fn novel_format_to_chinese(field: &str) -> &'static str {
    match field {
        "txt" => "Dinh dang txt",
        "epub" => "Dinh dang epub",
        "ask_after_download" => "Chon sau khi tai xong",
        _ => "Dinh dang txt",
    }
}

/// 将中文显示名转换为小说格式英文名
fn chinese_to_novel_format(chinese: &str) -> Option<String> {
    match chinese {
        "Dinh dang txt" => Some("txt".to_string()),
        "Dinh dang epub" => Some("epub".to_string()),
        "Chon sau khi tai xong" => Some("ask_after_download".to_string()),
        _ => None,
    }
}
