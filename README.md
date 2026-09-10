<div align="center">

# 🚀 Tauri File Server

**Ứng dụng Cổng Tiếp Nhận Báo Cáo & Chia Sẻ File Nội Bộ**

[![Tauri](https://img.shields.io/badge/Tauri-v2.0-blue?style=for-the-badge&logo=tauri)](https://tauri.app/)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![React](https://img.shields.io/badge/React-18-61DAFB?style=for-the-badge&logo=react)](https://react.dev/)
[![License](https://img.shields.io/badge/License-MIT-green?style=for-the-badge)](LICENSE)

</div>

---

## 📌 Giới thiệu

**Tauri File Server** là phần mềm desktop nhẹ, hiệu năng cao được xây dựng nhằm hỗ trợ truyền tải, nhận file và báo cáo giữa máy Client và Server trong mạng nội bộ.

- **Frontend:** React + Vite
- **Backend:** Rust + Tauri v2
- **Mục đích:** Cổng tiếp nhận báo cáo trung tâm, đảm bảo tốc độ và độ tin cậy cao.

---

## ✨ Tính năng chính

- ⚡ **Siêu nhẹ & Nhanh:** Sử dụng Rust backend giúp tối ưu dung lượng RAM và CPU.
- 📁 **Truyền nhận File:** Hỗ trợ gửi/nhận báo cáo và dữ liệu tập tin trực tiếp.
- 💻 **Giao diện thân thiện:** Thiết kế tối giản, dễ thao tác cho người dùng cuối.
- 🛠️ **Hỗ trợ Portable:** Chạy trực tiếp qua file `.exe` không cần cài đặt rườm rà.

---

## 🛠️ Hướng dẫn Cài đặt & Phát triển

### Yêu cầu hệ thống
- [Node.js](https://nodejs.org/) (phiên bản LTS)
- [Rust](https://www.rust-lang.org/) (mới nhất)
- Cấu hình môi trường Tauri (đã cài đặt WebView2 trên Windows)

### Các bước cài đặt mã nguồn

1. **Clone repository:**
   ```bash
   git clone [https://github.com/TNQuangHuy207/get-and-pust-form-clien-to-server.git](https://github.com/TNQuangHuy207/get-and-pust-form-clien-to-server.git)
   cd get-and-pust-form-clien-to-server


# 🚀 Cổng Tiếp Nhận Báo Cáo & Truyền File Nội Bộ (Tauri File Server)

> 💡 **Ứng dụng siêu nhẹ, hiệu năng cao giúp gửi/nhận file và báo cáo giữa các máy tính trong mạng nội bộ.**

---

> [!WARNING]
> ### ⚠️ LƯU Ý QUAN TRỌNG VỀ KẾT NỐI MẠNG (LAN / Wi-Fi)
> * **Ứng dụng CHỈ HOẠT ĐỘNG khi tất cả các máy tính DÙNG CHUNG 1 LỚP MẠNG.**
> * **Hiểu đơn giản:** Máy A (Máy nhận) và các máy gửi **bắt buộc phải kết nối chung 1 mạng Wi-Fi** (hoặc cắm chung 1 đường dây mạng LAN).
> * Nếu khác Wi-Fi hoặc dùng dữ liệu 4G/5G riêng biệt, các máy sẽ **KHÔNG THỂ** tìm thấy nhau để truyền dữ liệu.
> * *Ứng dụng truyền file trực tiếp qua mạng nội bộ nên không cần mạng Internet vẫn chạy bình thường.*

---

## 📥 Tải Phần Mềm

Chọn phiên bản phù hợp với nhu cầu của bạn bên dưới:

| Loại phiên bản | Mô tả | Liên kết tải |
| :--- | :--- | :--- |
| ⚡ **File Chạy Trực Tiếp (Portable)** | Tải về click đúp chạy ngay, không cần cài đặt. | [⬇️ Tải tauri-file-server.exe]([sha256:9f6119d6a247b17e2d6e78bbd3e7dc091768cde28e6cdadbff7ea0e2b8576082](https://github.com/TNQuangHuy207/get-and-pust-form-clien-to-server/releases/download/v1.0.0/tauri-file-server.exe)) |
| 📦 **File Cài Đặt (Installer)** | Tự động tạo Shortcut ngoài Desktop & Start Menu. | [⬇️ Tải tauri-file-server-setup.exe]([sha256:316aced26babaaaf367a19bbcf2d4cd49e3e59677744c4ccdc95f8abd56c0549](https://github.com/TNQuangHuy207/get-and-pust-form-clien-to-server/releases/download/v1.0.0/tauri-file-server_0.1.0_x64-setup.exe)) |

---

## 📖 Hướng Dẫn Sử Dụng

### Bước 1: Kiểm tra kết nối
1. Đảm bảo tất cả máy tính tham gia đã **bắt chung một mạng Wi-Fi**.
2. Mở ứng dụng trên **Máy A (Máy tiếp nhận báo cáo)** trước.
3. Khi mở phần mềm lần đầu, nếu Windows hiện thông báo **Windows Defender Firewall**, hãy tích chọn **Private networks** và bấm **Allow access (Cho phép)**.

### Bước 2: Truyền / Nhận dữ liệu
* **Tại Máy A (Server):** Giữ ứng dụng luôn bật để mở cổng tiếp nhận.
* **Tại Máy Client (Máy gửi):** Mở ứng dụng -> Chọn file / nhập báo cáo -> Bấm **Gửi**.

---

## 🛠️ Giải Quyết Lỗi Thường Gặp

<details>
<summary><b>1. Hai máy bắt chung Wi-Fi nhưng không gửi được file?</b></summary>

- Kiểm tra lại tường lửa (Windows Firewall). Bạn vào **Control Panel > Windows Defender Firewall** để đảm bảo app không bị chặn.
- Đảm bảo chế độ mạng Wi-Fi trên Windows đang để là **Private Network** thay vì **Public Network**.
</details>

<details>
<summary><b>2. Mất mạng Internet thì ứng dụng có chạy được không?</b></summary>

- **Có!** Ứng dụng chỉ cần router Wi-Fi phát tín hiệu mạng nội bộ, không phụ thuộc vào đường truyền Internet ra ngoài.
</details>
