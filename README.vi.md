# Tomato Novel Downloader — Hướng dẫn nhanh (Tiếng Việt)

## Giới thiệu
Tomato Novel Downloader là ứng dụng (Rust) hỗ trợ tải nội dung tiểu thuyết từ hệ sinh thái Fanqie và xuất ra định dạng đọc offline (EPUB/TXT), kèm tuỳ chọn tạo sách nói (TTS) tuỳ cấu hình.

Repo này chứa mã nguồn của ứng dụng và các thành phần giao diện:
- Web UI (khuyến nghị)
- TUI (terminal UI)
- CLI (không tương tác, chủ yếu để cập nhật)

## Bắt đầu nhanh
- Chạy TUI: mở file `.exe` không tham số.
- Chạy Web UI:
  - `Tomato-Novel-Downloader.exe --server`
  - Mặc định: `http://127.0.0.1:18423/`

## Chuyển ngôn ngữ
- Mở `config.yml`, chỉnh `ui_language`:
  - `vi` (Tiếng Việt)
  - `en` (English)
  - `zh-cn` (简体中文)
- Khởi động lại chương trình để áp dụng.

## CLI (không tương tác)
CLI chỉ hỗ trợ cập nhật sách đã tải trước đó:
- `Tomato-Novel-Downloader.exe --update <book_id>`

## Tài liệu chi tiết
- Xem hướng dẫn đầy đủ tại docs/HUONG_DAN_SU_DUNG.md

## Ghi chú về nhánh/bản lang-mod
Bản `lang-mod` ưu tiên tính ổn định khi thiếu Official-API, đồng thời phát triển i18n/TTS đa ngôn ngữ.

## Hỗ trợ
Khi cần hỗ trợ, vui lòng cung cấp `logs/latest.log` và mô tả bạn đang dùng Web UI/TUI/CLI.