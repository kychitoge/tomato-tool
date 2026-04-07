# Tomato Novel Downloader bản Việt hoá

> Bản Việt hoá này được xây dựng từ repository gốc: [Tomato-Novel-Downloader](https://github.com/zhongbai2333/Tomato-Novel-Downloader)

> Giữ nguyên tôn trọng bản quyền tác giả gốc. Đây là bản phục vụ sử dụng và đọc hiểu dễ hơn cho người dùng tiếng Việt.

Tomato Novel Downloader là phiên bản tải truyện Fanqie/番茄小说 đã được tái cấu trúc bằng `Rust`, phát triển tiếp từ dự án gốc của tác giả [zhongbai2333](https://github.com/zhongbai2333).

So với các nhánh trước, dự án hiện đã được viết lại gần như toàn bộ bằng `Rust`, đồng thời bổ sung thêm các tính năng như: hỗ trợ tải EPUB, tiếp tục tải tốt hơn khi gián đoạn, quản lý lỗi tốt hơn, tìm kiếm sách, Web UI, và nhiều cải tiến vận hành khác.

## Điểm cần biết

- Bản này được Việt hoá để phục vụ trải nghiệm sử dụng.
- Không chỉnh sửa thư mục `img/`.
- Các file như `README.md`, `Dockerfile`, `Cargo.toml`, `build.rs`, `installer.sh` vẫn được giữ theo hướng English hoặc nội dung gốc khi cần thiết cho khả năng tương thích.
- Mục tiêu chính là Việt hoá phần giao diện người dùng và thông báo sử dụng, không làm thay đổi logic nghiệp vụ không cần thiết.

## Chế độ build

Dự án hỗ trợ hai chế độ build:

- Chế độ mặc định (`official-api`): giữ năng lực Official-API (tìm kiếm / mục lục / đoạn bình luận), đồng thời vẫn tương thích với chế độ nội dung từ bên thứ ba.
- Chế độ `no-official-api`: **không phụ thuộc Official-API crate**; thông tin mục lục / sách đi qua phân tích web; **nội dung chính buộc dùng pool địa chỉ API bên thứ ba**.

Vì lý do an toàn cho API bên thứ ba, một phần mã liên quan đến interface nội bộ đó không được mở công khai, bao gồm địa chỉ và token. Mong bạn thông cảm.

Để hỗ trợ người dùng cần giao diện cũ, dự án vẫn giữ lại CLI legacy. Cách bật:

- Lần đầu mở chương trình, nhấn 3 lần `o` rồi Enter, hoặc nhấn một lần mũi tên xuống rồi nhấn 3 lần `o` để bật giao diện CLI cũ.
- Khi chuyển thành công, chương trình sẽ phát ra âm thanh báo hiệu.

---

## Cách sử dụng

Tải file thực thi phù hợp với hệ điều hành của bạn từ trang [Releases](https://github.com/zhongbai2333/Tomato-Novel-Downloader/releases) rồi chạy.

Khi tải sách mới lần đầu, nên ưu tiên dùng TUI hoặc Web UI. CLI chỉ còn giữ khả năng cập nhật sách đã có sẵn trên máy.

### Chế độ dòng lệnh (không tương tác)

Nếu bạn muốn dùng downloader trong script tự động, ví dụ để cập nhật truyện trên Kindle, có thể dùng tham số dòng lệnh để cập nhật các sách **đã tải về trước đó**:

- Cập nhật một sách cụ thể:

    ```sh
    Tomato-Novel-Downloader.exe --update <book_id>
    ```

    Ví dụ:

    ```sh
    Tomato-Novel-Downloader.exe --update 7318247498772674083
    ```

Lưu ý:

- Chế độ dòng lệnh là chế độ không tương tác, sẽ bắt đầu cập nhật ngay, không cần nhập tay.
- Sử dụng đường dẫn lưu mặc định và cấu hình tải trong file `config.yml`.
- **CLI đã vô hiệu hoá khả năng `--download` để tạo tải mới**, nhằm giảm nguy cơ bị lạm dụng hàng loạt.
- `--update` chỉ cho phép cập nhật các sách **đã tồn tại bản ghi tải về cục bộ** trong thư mục lưu mặc định.
- Nếu sách chưa có bản ghi cục bộ, CLI sẽ từ chối chạy và yêu cầu dùng Web UI / TUI để tải lần đầu.
- Chỉ chấp nhận `book_id`, không hỗ trợ tìm kiếm.

### Giao diện CLI cũ

- CLI cũ hiện đã **vô hiệu hoá tải mới / tìm kiếm để tải**.
- Chỉ còn các khả năng: cập nhật sách đã có, xem lịch sử tải, chỉnh cấu hình, kiểm tra cập nhật chương trình.
- Nếu cần tải sách mới lần đầu, hãy dùng TUI mặc định hoặc Web UI (`--server`).

### Chế độ Web UI (`--server`)

Nếu bạn muốn thao tác bằng trình duyệt trong mạng nội bộ (tìm kiếm, bắt đầu tải, xem tác vụ, tải file / đóng gói thư mục tải xuống), có thể bật Web UI:

- Khởi động Web UI:

    ```sh
    Tomato-Novel-Downloader.exe --server
    ```

- Địa chỉ lắng nghe mặc định (`127.0.0.1:18423`):

    Có thể đổi bằng biến môi trường, ví dụ để truy cập từ mạng LAN:

    ```sh
    TOMATO_WEB_ADDR=0.0.0.0:18423
    ```

    Ví dụ lắng nghe IPv6 (lưu ý IPv6 cần đặt trong ngoặc vuông):

    ```sh
    TOMATO_WEB_ADDR=[::]:18423
    ```

    Lắng nghe nhiều địa chỉ cùng lúc (phân tách bằng dấu phẩy hoặc chấm phẩy), ví dụ IPv4 + IPv6:

    ```sh
    TOMATO_WEB_ADDR=0.0.0.0:18423,[::]:18423
    ```

- Chế độ khoá bằng mật khẩu (ngăn người lạ sử dụng):

    ```sh
    Tomato-Novel-Downloader.exe --server --password mật_khẩu_của_bạn
    ```

    Hoặc dùng biến môi trường:

    ```sh
    TOMATO_WEB_PASSWORD=mật_khẩu_của_bạn
    ```

- Thư mục dữ liệu (dùng cho Docker hoặc quản lý tập trung cấu hình / log):

    Dùng tham số `--data-dir` để chỉ định thư mục dữ liệu; chương trình sẽ đặt `config.yml` và thư mục `logs` trong đó:

    ```sh
    Tomato-Novel-Downloader.exe --server --data-dir /data
    ```

    Ví dụ dùng Docker:

    ```sh
    docker run -v /host/data:/data my-tomato-image --server --data-dir /data
    ```

    Cách này giúp mount dữ liệu dễ dàng, hỗ trợ lưu bền cấu hình và log.

Các tính năng của Web UI (HTML thuần, không cần build frontend riêng):

- Tìm sách và tạo tác vụ tải
- Danh sách tác vụ / làm mới tiến độ / huỷ tác vụ
- Duyệt thư viện tải theo thư mục, không còn flatten toàn bộ file
- Tải file trực tiếp
- Đóng gói cả thư mục thành file zip để tải xuống, giữ nguyên cấu trúc thư mục
- Trang cấu hình: có thể chỉnh một số tuỳ chọn đầu ra và lưu ngược lại vào `config.yml`

Lưu ý: Web UI chủ yếu phục vụ self-host / mạng nội bộ. Nếu muốn public ra Internet, nên đặt sau reverse proxy / HTTPS và bật khoá mật khẩu.

---

## Docker image

Đã có sẵn image Docker cho Web UI:

- Địa chỉ image: [DockerHub](https://hub.docker.com/r/zhongbai233/tomato-novel-downloader-webui)
- Ý nghĩa tag:
  - `latest`: bản **glibc** mặc định, phù hợp server / desktop thông thường
  - `latest-musl`: bản **musl**, phù hợp **router / NAS** hoặc hệ thống nhẹ

Ví dụ chạy với glibc, map cổng và persist dữ liệu:

```sh
docker run -d \
    --name tomato-novel-webui \
    -p 18423:18423 \
    -v /host/data:/data \
    -e TOMATO_WEB_ADDR=0.0.0.0:18423 \
    -e TOMATO_WEB_PASSWORD=mật_khẩu_của_bạn \
    zhongbai233/tomato-novel-downloader-webui:latest --server --data-dir /data
```

Nếu bạn dùng router hoặc NAS, hãy dùng bản musl:

```sh
docker run -d \
    --name tomato-novel-webui \
    -p 18423:18423 \
    -v /host/data:/data \
    -e TOMATO_WEB_ADDR=0.0.0.0:18423 \
    -e TOMATO_WEB_PASSWORD=mật_khẩu_của_bạn \
    zhongbai233/tomato-novel-downloader-webui:latest-musl --server --data-dir /data
```

Bạn có thể dùng `TOMATO_WEB_ADDR`, `TOMATO_WEB_PASSWORD` và `--data-dir` để điều khiển địa chỉ lắng nghe, mật khẩu và thư mục dữ liệu.

---

## Chế độ build (Cargo features)

Project có hai feature loại trừ nhau: `official-api` và `no-official-api` (không thể bật cùng lúc).

### Chế độ mặc định: `official-api`

- Build (mặc định đã bật):

```sh
cargo build --release
```

- Hành vi:
  - Tìm kiếm hoạt động được (TUI / Web UI / mục tìm kiếm của CLI cũ).
  - Đoạn bình luận EPUB / thu thập tài nguyên hoạt động được, tuỳ theo cấu hình.
  - Có thể chuyển giữa chế độ “official / third-party” khi lấy nội dung chính thông qua cấu hình (`use_official_api`).

### Chế độ `no-official-api` (Issue #187)

- Build:

```sh
cargo build --release --no-default-features --features no-official-api
```

- Khác biệt chính:
  - **Không phụ thuộc** crate `tomato-novel-official-api`, nên có thể build khi không có môi trường Official-API.
  - Thông tin mục lục và sách: dùng phân tích web (`FanqieWebNetwork`).
  - **Lấy nội dung chính: bắt buộc dùng chế độ bên thứ ba**.
  - Tìm kiếm: không khả dụng.
  - Đoạn bình luận: không khả dụng.

---

## Tạo sách nói bằng Edge TTS

Từ phiên bản hiện tại, chương trình tích hợp [msedge-tts](https://github.com/hs-cn/msedge-tts) để tổng hợp giọng nói, cho phép tạo sách nói sau khi tải văn bản xong:

- Trong menu cấu hình (dù là UI mới hay CLI cũ), bật `是否生成有声小说` để tự tạo file âm thanh sau mỗi lần tải hoàn tất.
- Giọng mặc định là `zh-CN-XiaoxiaoNeural`, bạn có thể tuỳ chỉnh tốc độ đọc, âm lượng, cao độ và định dạng đầu ra (`mp3` hoặc `wav`). Giá trị cao độ nên dùng dạng có đơn vị như `+2Hz`, `-1st`; nếu để trống hoặc nhập 0 thì sẽ bỏ qua chỉnh cao độ.
- Có thể chỉnh số lượng tác vụ đồng thời trong mục `有声小说并发数` (mặc định là 2). Khi tạo sẽ có thanh tiến trình; hãy chọn mức phù hợp với mạng và máy của bạn.
- Âm thanh sẽ được lưu trong thư mục `{tên_sách}_audio` ở thư mục đầu ra, và đặt tên theo thứ tự chương, ví dụ `0001-第一章.mp3`.
- `msedge-tts` cần gọi dịch vụ online của Microsoft, vì vậy môi trường chạy phải có thể truy cập Internet.

Nếu tạo thất bại, bạn có thể xem log để biết chi tiết lỗi.

---

## Câu hỏi thường gặp

1. Trước đây đã có một downloader rồi, tại sao còn làm thêm cái khác?

    ~~Mục tiêu ban đầu của chương trình là tối giản hoá mã nguồn của Tomato Novel Downloader để dễ vận hành, ổn định và nhanh hơn.~~
    Sau khi tái cấu trúc, dự án hiện có kích thước lớn hơn bản gốc, không còn “nhẹ” như trước, nhưng đổi lại thao tác dễ dùng, ít cần cấu hình và chạy ngay.

2. Có chạy được trên điện thoại không?

    **Chỉ hỗ trợ thiết bị Android (Termux)**.
    Tuy nhiên, vì **TUI/CLI không thân thiện với màn hình nhỏ**, nên trên điện thoại vẫn khuyến nghị dùng **Web UI (`--server`)**: khởi động dịch vụ trong Termux rồi thao tác bằng trình duyệt trên điện thoại (hoặc thiết bị khác trong cùng LAN).

    Trong release có sẵn bản build Android arm64: `TomatoNovelDownloader-Android_arm64-[số phiên bản hiện tại]`, có thể chạy trực tiếp trong Termux.

    Ngoài ra, nếu bạn muốn trong TUI dùng `Ctrl+V` để dán từ clipboard hệ thống, cần cài Termux API:

    - Cài app: Termux:API
    - Cài lệnh: `pkg install termux-api`
    - Kiểm tra: `termux-clipboard-get` xuất ra nội dung bình thường

    Để hỗ trợ người mới, bạn có thể cài bằng script:

    ```sh
    bash <(curl -sL https://raw.githubusercontent.com/zhongbai2333/Tomato-Novel-Downloader/main/installer.sh)
    ```

    Người dùng trong nước có thể dùng:

    ```sh
    bash <(curl -sL https://dl.zhongbai233.com/installer.sh)
    ```

    Sau khi cài xong, nên khởi động bằng Web UI (ví dụ):

    ```sh
    TOMATO_WEB_ADDR=0.0.0.0:18423 TOMATO_WEB_PASSWORD=mật_khẩu_của_bạn tomato-novel-downloader --server
    ```

    Sau đó mở trình duyệt:

    - Máy hiện tại: `http://127.0.0.1:18423/`
    - Thiết bị khác trong LAN: `http://<IP LAN của điện thoại>:18423/`

3. Máy tính thì chạy thế nào?

    Trên `Windows`, chỉ cần nhấp đúp vào `TomatoNovelDownloader-Win64-[số phiên bản hiện tại].exe`

    Trên `Linux` và `MacOS`, chạy bằng terminal. Bạn cũng có thể dùng script cài nhanh:

    ```sh
    bash <(curl -sL https://raw.githubusercontent.com/zhongbai2333/Tomato-Novel-Downloader/main/installer.sh)
    ```

    Người dùng trong nước có thể dùng:

    ```sh
    bash <(curl -sL https://dl.zhongbai233.com/installer.sh)
    ```

4. Book ID là gì? Lấy ở đâu?

    Có hai cách khuyên dùng:

    - Dùng trực tiếp chức năng “tìm sách” của Web UI, không cần tự tìm ID.
    - Nếu bạn đã có link chia sẻ hoặc thông tin sách, thường sẽ có một dãy số rất dài (Book ID). Chỉ cần copy dãy đó.

5. Tôi là người mới hoàn toàn, tải chương trình ở đâu?

    Vào link [Releases](https://github.com/zhongbai2333/Tomato-Novel-Downloader/releases), tìm bản mới nhất, mở phần `Assets`, rồi tải đúng file dành cho hệ điều hành của bạn.

## Lưu ý quan trọng

Vì chương trình phụ thuộc API, tương lai có thể có lúc API đột ngột ngừng hoạt động. Nếu gặp trường hợp đó, vui lòng báo ngay ở trang “Issues”.

Nếu khi sử dụng gặp lỗi tải chương, chưa chắc là API hỏng. Có thể do lượng người dùng quá đông khiến API tạm ngưng hoặc sách bạn cần tải chưa được cập nhật trên API.

Đừng nghĩ rằng tăng số luồng sẽ tải nhanh hơn: việc đó chỉ làm tăng áp lực lên máy chủ.

Khi sử dụng, cũng không nên bật VPN hoặc proxy làm ảnh hưởng tới kết nối mạng.

Nếu vẫn gặp lỗi, hãy kiểm tra số lượng chương cần tải. Không khuyến nghị vượt quá 1500 chương.

> Nhấn mạnh: không được dùng chương trình này cho mục đích vi phạm pháp luật, ví dụ phát tán nội dung đã tải, chia sẻ cho người khác sử dụng sai mục đích, hoặc dùng API trái phép. Nội dung tải về chỉ nên phục vụ đọc cá nhân. Sau khi xem xong nên xoá file để tránh rủi ro bản quyền. Tác giả và cộng đồng đóng góp không chịu trách nhiệm cho mọi thiệt hại, tranh chấp pháp lý hoặc hậu quả phát sinh từ việc sử dụng chương trình.

## Tuyên bố miễn trừ trách nhiệm

  Chương trình này chỉ phục vụ mục đích học tập về Rust, kỹ thuật crawler mạng, xử lý dữ liệu web và các nghiên cứu liên quan. Vui lòng không sử dụng nó cho bất kỳ hoạt động nào vi phạm pháp luật hoặc xâm phạm quyền lợi của người khác.
  
  Người dùng tự chịu mọi trách nhiệm pháp lý và rủi ro phát sinh từ việc sử dụng chương trình. Tác giả và cộng đồng đóng góp không chịu trách nhiệm cho bất kỳ tổn thất, thiệt hại hoặc hậu quả pháp lý nào.
  
  Trước khi sử dụng, hãy chắc chắn rằng bạn tuân thủ các quy định pháp luật hiện hành và chính sách sử dụng của website mục tiêu. Nếu có bất kỳ thắc mắc hoặc lo ngại nào, hãy tham khảo ý kiến của luật sư chuyên môn.

## Lời cảm ơn

Cảm ơn bạn đã chọn sử dụng chương trình này. Nếu thấy hữu ích, bạn có thể để star. Nếu có góp ý, hãy gửi ở trang “Issues”. Sự ủng hộ của bạn là động lực lớn nhất để dự án tiếp tục được cập nhật.

Giai đoạn đầu dự án · Cảm ơn dự án nền tảng ban đầu của tác giả Dimily

Giai đoạn đầu dự án · Cảm ơn API từ GitHub user @helloplhm-qwq

Giai đoạn đầu dự án · Cảm ơn API từ QQ user @终忆

Giai đoạn đầu dự án · Cảm ơn API từ GitHub user @jingluopro
