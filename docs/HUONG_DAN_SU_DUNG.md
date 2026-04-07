# Hướng dẫn sử dụng Tomato Novel Downloader (lang-mod)

## 1) Repo này là gì?
Tomato Novel Downloader là công cụ tải nội dung tiểu thuyết từ hệ sinh thái Fanqie (番茄小说) và xuất ra các định dạng phục vụ đọc offline (ví dụ: EPUB/TXT) và/hoặc tạo sách nói (TTS) tuỳ cấu hình.

Repo này là mã nguồn của ứng dụng (Rust), gồm 3 cách sử dụng chính:
- **Web UI** (khuyến nghị): thao tác bằng trình duyệt, theo dõi tiến độ bằng “job”.
- **TUI**: giao diện terminal tương tác.
- **CLI (không tương tác)**: chỉ hỗ trợ **cập nhật** các sách đã từng tải trước đó.

Lưu ý về bản `lang-mod`:
- Ưu tiên khả năng chạy thực tế trong bối cảnh thiếu Official-API.
- Mục tiêu: xây dựng hệ thống i18n (đa ngôn ngữ) gọn, dễ bảo trì; và map TTS theo ngôn ngữ.

## 2) Cài đặt và chạy chương trình
- Tải file `.exe` đúng nền tảng từ mục Releases (hoặc thư mục phát hành trong dự án nội bộ).
- Chạy trực tiếp file `.exe`.
- Lần chạy đầu tiên sẽ tự tạo `config.yml` (cùng thư mục chạy, hoặc trong thư mục do `--data-dir` chỉ định).

## 3) Chuyển ngôn ngữ (i18n)
Ngôn ngữ giao diện được điều khiển bởi cấu hình:
- Mở `config.yml`
- Chỉnh khoá `ui_language` thành một trong các giá trị:
  - `vi` (Tiếng Việt)
  - `en` (English)
  - `zh-cn` (简体中文)

Sau khi lưu `config.yml`, hãy khởi động lại chương trình (TUI/Web UI) để giao diện áp dụng ngôn ngữ mới.

## 4) Sử dụng Web UI (khuyến nghị)
### 4.1 Bật chế độ server
Chạy:

```bat
Tomato-Novel-Downloader.exe --server
```

Mặc định Web UI lắng nghe tại `127.0.0.1:18423`.

### 4.2 Đổi địa chỉ/port lắng nghe
Thiết lập biến môi trường:

```bat
set TOMATO_WEB_ADDR=127.0.0.1:18423
```

Ví dụ mở ra LAN:

```bat
set TOMATO_WEB_ADDR=0.0.0.0:18423
```

### 4.3 Khoá bằng mật khẩu (khuyến nghị nếu dùng trong LAN)

```bat
Tomato-Novel-Downloader.exe --server --password "MatKhauCuaBan"
```

Hoặc dùng biến môi trường:

```bat
set TOMATO_WEB_PASSWORD=MatKhauCuaBan
```

### 4.4 Thư mục dữ liệu (config/log)

```bat
Tomato-Novel-Downloader.exe --server --data-dir D:\TomatoData
```

Khi đó `config.yml` và thư mục `logs/` sẽ nằm trong `D:\TomatoData`.

### 4.5 Quy trình sử dụng cơ bản
- Mở trình duyệt tới địa chỉ Web UI.
- Tìm kiếm/nhập `book_id` (tuỳ màn hình).
- Tạo job tải.
- Theo dõi tiến độ trong trang Jobs.

## 5) Sử dụng TUI
- Chạy file `.exe` (không tham số) để vào TUI.
- Thao tác theo hướng dẫn trên màn hình.

Ghi chú:
- Một số thao tác phụ thuộc chế độ build (xem mục 8).

## 6) Sử dụng CLI (không tương tác)
CLI hiện chỉ hỗ trợ cập nhật sách **đã có dữ liệu tải trước đó** trong thư mục lưu.

Ví dụ cập nhật một sách theo `book_id`:

```bat
Tomato-Novel-Downloader.exe --update 7318247498772674083
```

Tuỳ chọn:
- `--retry-failed`: thử lại một lần với các chương thất bại trong lần chạy đó.

Lưu ý quan trọng:
- CLI không hỗ trợ tạo tải mới (để hạn chế lạm dụng tự động hoá). Hãy dùng Web UI hoặc TUI để tải lần đầu.

## 7) Các lỗi hay gặp và cách xử lý nhanh
### 7.1 Không mở được Web UI / báo lỗi bind port
Triệu chứng: báo lỗi “bind failed” (port đã bị dùng).
- Đổi port bằng `TOMATO_WEB_ADDR`, ví dụ `127.0.0.1:19700`.

### 7.2 Job thất bại do mạng hoặc bị giới hạn
- Thử chạy lại job.
- Kiểm tra kết nối Internet.
- Xem `logs/latest.log` để biết nguyên nhân chi tiết.

### 7.3 Không tải được nội dung khi không cấu hình API bên thứ ba
Trong bản `lang-mod`, nếu không có `api_endpoints` chương trình sẽ **tự chuyển sang chế độ đọc trang reader công khai** để lấy nội dung (fallback). Nếu vẫn thất bại:
- Kiểm tra lại `book_id`/chương có tồn tại.
- Thử lại sau (có thể do upstream thay đổi).

### 7.4 TTS không chạy hoặc bị lỗi
- Đảm bảo đã bật `enable_audiobook` trong `config.yml`.
- Nếu mạng chặn dịch vụ TTS, cần kiểm tra lại môi trường mạng.

## 8) Ghi chú về chế độ build (Official / No-Official)
Dự án có 2 feature loại trừ nhau:
- `official-api`: dùng Official-API (khi có đủ crate/phụ thuộc).
- `no-official-api`: không phụ thuộc Official-API.

Trong nhánh/bản `lang-mod`, mặc định ưu tiên hướng **không phụ thuộc Official-API** để đảm bảo build và chạy.

## 9) Vị trí cấu hình và log
- `config.yml`: file cấu hình chính.
- `logs/latest.log`: log mới nhất.

Nếu bạn cần hỗ trợ, hãy gửi kèm `logs/latest.log` và thông tin bạn đang dùng Web UI/TUI/CLI.