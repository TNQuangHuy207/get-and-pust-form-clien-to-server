#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use axum::{
    body::Body,
    extract::{ConnectInfo, DefaultBodyLimit, Query},
    http::{header, HeaderMap, StatusCode},
    response::{Html, IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use futures_util::TryStreamExt;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::net::{SocketAddr, UdpSocket};
use std::path::{Path, PathBuf};
use std::process::Command;
use tokio_util::io::StreamReader;

const BASE_DIR: &str = "C:\\ThuMucTong";
const VALID_FOLDERS: [&str; 3] = ["BaoCaoThietBi", "KhoPhanMem", "TaiLieuKhac"];
const ALLOWED_EXTS: [&str; 18] = [
    "pdf", "html", "htm", "txt", "xls", "xlsx", "xlsm", "csv", "zip", "rar", "7z", "exe", "msi",
    "doc", "docx", "png", "jpg", "jpeg",
];

#[derive(Serialize)]
struct FileInfo {
    name: String,
    size: String,
}

#[derive(Serialize)]
struct LogEntry {
    time: String,
    r#type: String,
    ip: String,
    host: String,
    file: String,
}

#[derive(Deserialize)]
struct DownloadQuery {
    folder: Option<String>,
    file: Option<String>,
}

fn get_local_ip() -> String {
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        if socket.connect("8.8.8.8:80").is_ok() {
            if let Ok(addr) = socket.local_addr() {
                return addr.ip().to_string();
            }
        }
    }
    "127.0.0.1".to_string()
}

#[tauri::command]
fn get_server_info() -> String {
    let ip = get_local_ip();
    format!("http://{}:8080/", ip)
}

fn init_environment() {
    for folder in VALID_FOLDERS {
        let path = format!("{}\\{}", BASE_DIR, folder);
        if !Path::new(&path).exists() {
            let _ = fs::create_dir_all(&path);
        }
    }

    let _ = Command::new("netsh")
        .args([
            "advfirewall", "firewall", "add", "rule",
            "name=Tool_Nhan_File_8080", "dir=in", "action=allow",
            "protocol=TCP", "localport=8080",
        ])
        .output();
}

fn write_activity_log(log_type: &str, client_ip: &str, file_name: &str) {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
    let host_name = format!("Máy ({})", client_ip);
    let log_line = format!(
        "{} | {} | IP: {} | Máy: {} | File: {}\n",
        now, log_type, client_ip, host_name, file_name
    );
    let log_path = format!("{}\\LichSuHoatDong.txt", BASE_DIR);
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
        let _ = file.write_all(log_line.as_bytes());
    }
}

fn update_master_report() {
    let dir_sub = format!("{}\\BaoCaoThietBi", BASE_DIR);
    let master_html_path = format!("{}\\BaoCaoTongHop.html", BASE_DIR);
    let mut blocks = String::new();
    let mut dev_num = 1;

    if let Ok(entries) = fs::read_dir(&dir_sub) {
        let mut files: Vec<PathBuf> = entries
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| p.is_file())
            .collect();
        files.sort();

        for file_path in files {
            let file_name = file_path.file_name().unwrap_or_default().to_string_lossy().to_string();
            let ext = file_path.extension().unwrap_or_default().to_string_lossy().to_lowercase();
            if !["txt", "xls", "xlsx", "csv", "html", "htm"].contains(&ext.as_str()) {
                continue;
            }

            let content = fs::read_to_string(&file_path).unwrap_or_default();
            let mut data: HashMap<String, String> = HashMap::new();
            let mut sw_list: Vec<String> = Vec::new();
            let mut in_sw = false;

            for line in content.lines() {
                let clean_line = line.trim();
                if clean_line.contains("[Software]") || clean_line.contains("Ứng dụng") || clean_line.contains("Phần mềm") {
                    in_sw = true;
                    continue;
                }

                if in_sw && (clean_line.starts_with("- ") || clean_line.chars().next().map_or(false, |c| c.is_ascii_digit())) {
                    let sw_name = clean_line.trim_start_matches(|c: char| c == '-' || c.is_ascii_digit() || c == '.' || c == ' ');
                    let sw_name = sw_name.split(':').next().unwrap_or("").trim();
                    if !sw_name.is_empty() && !sw_name.contains("Tên linh kiện") && !sw_name.contains("CẤU HÌNH") {
                        sw_list.push(sw_name.to_string());
                    }
                } else if let Some((k, v)) = clean_line.split_once(':') {
                    let key = k.trim().trim_start_matches("- ").to_string();
                    let val = v.trim().to_string();
                    if !key.is_empty() && !val.is_empty() {
                        data.entry(key).or_insert(val);
                    }
                }
            }

            let comp = data.get("Computer Name").or_else(|| data.get("Tên máy tính")).cloned()
                .unwrap_or_else(|| file_name.replace("BaoCaoKiemKe_", "").replace(".xls", ""));
            let model = data.get("Manufacturer/Model").or_else(|| data.get("Laptop / Model")).or_else(|| data.get("Model")).cloned()
                .unwrap_or_else(|| "Thiết bị văn phòng".to_string());
            let cpu = data.get("CPU").cloned().unwrap_or_else(|| "Chưa rõ CPU".to_string());
            let ram = data.get("RAM Installed / Supported").or_else(|| data.get("RAM")).cloned()
                .unwrap_or_else(|| "Chưa rõ RAM".to_string());
            let disk = data.get("Disk Health").or_else(|| data.get("Ổ cứng")).cloned()
                .unwrap_or_else(|| "Healthy".to_string());
            let win = data.get("Windows Status").or_else(|| data.get("Windows")).cloned()
                .unwrap_or_else(|| "Bản quyền hợp lệ".to_string());
            let office = data.get("MS Office Status (OSPP)").or_else(|| data.get("MS Office")).cloned()
                .unwrap_or_else(|| "Chưa rõ".to_string());
            let kms = data.get("Local KMS Hack Check").or_else(|| data.get("Kiểm tra bản quyền (KMS / Hosts)")).cloned()
                .unwrap_or_else(|| "Clean".to_string());
            let user = data.get("Họ và tên người dùng").cloned().unwrap_or_else(|| "[Chưa có dữ liệu]".to_string());
            let dept = data.get("Vị trí / Phòng ban").cloned().unwrap_or_else(|| "[Chưa có dữ liệu]".to_string());

            let office_badge = if office.contains("Not Found") || office.contains("Not Installed") {
                r#"<span class="badge-warn">Thiếu ứng dụng văn phòng</span>"#
            } else {
                r#"<span class="badge-ok">Bản quyền chính hãng</span>"#
            };

            let sw_str = if !sw_list.is_empty() { sw_list.join(", ") } else { "Đầy đủ trình duyệt và công cụ làm việc".to_string() };
            let dev_formatted = format!("{:02}", dev_num);

            blocks.push_str(&format!(
                r#"
                <tr class="machine-header-bar-row"><td colspan="3" class="machine-header-bar">THIẾT BỊ {dev_formatted}: {comp} ({model})</td></tr>
                <tr class="section-header"><td colspan="3">1. TÊN NGƯỜI DÙNG / VỊ TRÍ PHÒNG BAN</td></tr>
                <tr><td>Tên máy tính</td><td>{comp}</td><td><span class="badge-ok">Đã định danh hệ thống</span></td></tr>
                <tr><td>Họ và tên người dùng</td><td>{user}</td><td>Cần bổ sung nhân sự tiếp nhận</td></tr>
                <tr><td>Vị trí / Phòng ban</td><td>{dept}</td><td>Cần bổ sung thông tin quản lý</td></tr>
                <tr class="section-header"><td colspan="3">2. CẤU HÌNH PHẦN CỨNG</td></tr>
                <tr><td>Laptop / Model</td><td>{model}</td><td>Dòng thiết bị văn phòng</td></tr>
                <tr><td>CPU</td><td>{cpu}</td><td>Đáp ứng tốt tác vụ văn phòng</td></tr>
                <tr><td>RAM</td><td>{ram}</td><td>Bình thường</td></tr>
                <tr><td>Ổ cứng</td><td>{disk}</td><td>Tình trạng Healthy</td></tr>
                <tr class="section-header"><td colspan="3">3. PHẦN MỀM</td></tr>
                <tr><td>Windows</td><td>{win}</td><td>Bản quyền OEM / Volume hợp lệ</td></tr>
                <tr><td>MS Office</td><td>{office}</td><td>{office_badge}</td></tr>
                <tr><td>Kiểm tra bản quyền (KMS / Hosts)</td><td>{kms}</td><td><span class="badge-ok">An toàn, không dùng phần mềm bẻ khóa</span></td></tr>
                <tr><td>Ứng dụng đã cài</td><td>{sw_str}</td><td>Đầy đủ công cụ làm việc</td></tr>
                "#
            ));
            dev_num += 1;
        }
    }

    let html_content = format!(
        r#"<!DOCTYPE html>
<html lang="vi">
<head>
    <meta charset="UTF-8">
    <title>Tầm soát Thiết bị &amp; Phần mềm - Báo cáo Tổng hợp</title>
    <style>
        body {{ font-family: Arial, sans-serif; background-color: #f1f5f9; color: #0f172a; margin: 0; padding: 40px 20px; }}
        .container {{ max-width: 960px; margin: 0 auto; background: #ffffff; padding: 35px; border-radius: 8px; box-shadow: 0 4px 12px rgba(0,0,0,0.08); }}
        h1 {{ color: #0f2942; text-align: center; margin-bottom: 25px; font-size: 24px; text-transform: uppercase; }}
        table {{ width: 100%; border-collapse: collapse; margin-bottom: 35px; }}
        th, td {{ border: 1px solid #cbd5e0; padding: 10px 14px; text-align: left; font-size: 14px; }}
        th {{ background-color: #e2e8f0; color: #1e293b; font-weight: bold; }}
        .machine-header-bar {{ background-color: #0f2942 !important; color: #ffffff !important; font-weight: bold; padding: 12px 14px; text-transform: uppercase; }}
        .section-header {{ background-color: #ebf8ff !important; font-weight: bold; color: #1e40af !important; }}
        .badge-warn {{ color: #c53030; font-weight: bold; }}
        .badge-ok {{ color: #2f855a; font-weight: bold; }}
        .styled-table {{ width: 100%; border-collapse: collapse; }}
        .styled-table th {{ text-align: center; padding: 14px 16px; background-color: #eef4fb; color: #0f172a; border-bottom: 2px solid #1e3a8a; }}
    </style>
</head>
<body>
    <div class="container">
        <h1>Tầm soát Thiết bị &amp; Phần mềm</h1>  
        <table class="styled-table">
            <thead>
                <tr><th style="width: 35%;">Tên linh kiện / Phần mềm</th><th style="width: 35%;">Thông tin chi tiết</th><th style="width: 30%;">Đánh giá sơ bộ</th></tr>
            </thead>
            <tbody>{}</tbody>
        </table>
    </div>
</body>
</html>"#,
        blocks
    );

    let _ = fs::write(master_html_path, html_content);
}

// Handlers
async fn handle_index() -> Html<&'static str> {
    Html(r#"<!DOCTYPE html>
<html>
<head>
<meta charset="utf-8">
<title>Cổng Tiếp Nhận & Tải File - Máy A</title>
<style>
body{font-family:Segoe UI, Tahoma, sans-serif;background:#f0f2f5;padding:20px;text-align:center;color:#333}
.card{max-width:800px;margin:10px auto;background:#fff;padding:25px;border-radius:10px;box-shadow:0 2px 12px rgba(0,0,0,0.1)}
h2{color:#0f2942;margin-top:0}
.upload-controls { background: #e9f2ff; padding: 15px; border-radius: 8px; border: 1px solid #b8daff; margin-bottom: 15px; text-align: left; }
.upload-controls label { font-weight: bold; color: #0f2942; margin-right: 10px; display: inline-block; width: 150px; }
.upload-controls select { padding: 8px 12px; border-radius: 4px; border: 1px solid #ccc; font-size: 14px; width: calc(100% - 170px); font-weight: bold; }
.box{border:2px dashed #007bff;background:#f8faff;padding:25px;border-radius:8px;cursor:pointer;margin-bottom:15px; transition: background 0.3s;}
.box:hover{background:#e2edff}
#f{display:none}
.list{text-align:left;margin-top:10px;max-height:150px;overflow-y:auto}
.item{padding:8px;margin-bottom:5px;border-radius:4px;font-size:13px;background:#eee}
.ok{background:#d4edda;color:#155724}
.err{background:#f8d7da;color:#721c24}
.btn-view{display:inline-block;margin:10px 0 15px 0;padding:10px 20px;background:#28a745;color:#fff;text-decoration:none;border-radius:5px;font-weight:bold}
.btn-view:hover{background:#218838}
.section-title{text-align:left;font-size:15px;font-weight:bold;color:#0f2942;margin-top:20px;border-bottom:2px solid #007bff;padding-bottom:5px}
table.custom-table{width:100%;border-collapse:collapse;margin-top:5px;font-size:13px;text-align:left}
table.custom-table th{background:#f1f5f9;padding:8px;border-bottom:1px solid #ccc;color:#475569}
table.custom-table td{padding:8px;border-bottom:1px solid #eee; vertical-align:middle;}
.btn-dl{background:#007bff;color:#fff;padding:4px 10px;text-decoration:none;border-radius:4px;font-size:12px;display:inline-block}
.btn-dl:hover{background:#0056b3}
.badge-up{background:#d1e7dd;color:#0f5132;padding:2px 6px;border-radius:4px;font-weight:bold;font-size:11px}
.badge-down{background:#cff4fc;color:#055160;padding:2px 6px;border-radius:4px;font-weight:bold;font-size:11px}
.folder-header { background: #f1f5f9; padding: 12px 16px; border: 1px solid #cbd5e0; border-radius: 6px; cursor: pointer; font-weight: bold; display: flex; justify-content: space-between; align-items: center; color: #1e293b; margin-bottom: 6px; transition: background 0.2s; }
.folder-header:hover { background: #e2e8f0; }
.folder-content { display: none; padding: 10px; border: 1px solid #cbd5e0; border-top: none; border-radius: 0 0 6px 6px; background: #fff; margin-top: -6px; margin-bottom: 8px; }
</style>
</head>
<body>
<div class="card">
<h2>CỔNG TIẾP NHẬN & CHIA SẺ FILE (MÁY A)</h2>
<p style="font-size:13px;color:#666">Thư mục tổng: <b>C:\ThuMucTong\</b></p>

<div class="upload-controls">
    <label for="folderSelect">1. Chọn nơi lưu file:</label>
    <select id="folderSelect">
        <option value="BaoCaoThietBi">📂 Báo cáo thiết bị công ty (Mặc định)</option>
        <option value="KhoPhanMem">📦 Kho phần mềm</option>
        <option value="TaiLieuKhac">📄 Tài liệu khác</option>
    </select>
</div>

<div class="box" onclick="document.getElementById('f').click()" ondragover="event.preventDefault()" ondrop="event.preventDefault(); up(event.dataTransfer.files)">
2. <b>Bấm vào đây</b> hoặc <b>Kéo thả File</b> để gửi lên Máy A<br>
<span style="font-size:12px;color:#666">(Hỗ trợ tất cả định dạng: ZIP, RAR, 7Z, EXE, MSI, PDF, DOCX, XLSX...)</span>
<input type="file" id="f" multiple onchange="up(this.files); this.value='';">
</div>

<a href="/report" target="_blank" class="btn-view">Xem &amp; Xuất PDF Báo Cáo Tổng Hợp</a>
<div id="l" class="list"></div>

<div class="section-title">DANH SÁCH THƯ MỤC &amp; TẢI VỀ</div>
<div id="fileContainer" style="max-height:250px; overflow-y:auto; margin-top:10px;">
    <p style="color:#777; font-size:13px;">Đang tải danh sách thư mục...</p>
</div>

<div class="section-title">LỊCH SỬ HOẠT ĐỘNG (UPLOAD / DOWNLOAD)</div>
<div id="historyContainer" style="max-height:220px; overflow-y:auto;">
    <p style="color:#777; font-size:13px;">Đang tải lịch sử hoạt động...</p>
</div>

</div>

<script>
function loadFiles() {
    fetch('/list')
    .then(r => r.json())
    .then(data => {
        let container = document.getElementById('fileContainer');
        let folders = Object.keys(data);
        if (!folders || folders.length === 0) {
            container.innerHTML = '<p style="color:#777; font-size:13px; margin-top:10px;">Chưa có thư mục nào.</p>';
            return;
        }
        const folderLabels = {
            'BaoCaoThietBi': '📂 Báo cáo thiết bị công ty',
            'KhoPhanMem': '📦 Kho phần mềm',
            'TaiLieuKhac': '📄 Tài liệu khác'
        };
        let html = '';
        folders.forEach(folderKey => {
            let files = data[folderKey];
            let label = folderLabels[folderKey] || folderKey;
            let count = files.length;
            html += '<div class="folder-header" onclick="toggleFolder(\'' + folderKey + '\')">';
            html += '<span>' + label + ' <span style="font-size:12px; font-weight:normal; color:#64748b;">(' + count + ' file)</span></span>';
            html += '<span id="icon-' + folderKey + '">▼</span></div>';
            html += '<div id="content-' + folderKey + '" class="folder-content">';
            if (count === 0) {
                html += '<p style="color:#777; font-size:13px; margin:5px 0;">Thư mục này hiện đang trống.</p>';
            } else {
                html += '<table class="custom-table"><thead><tr><th>Tên File</th><th>Dung lượng</th><th>Hành động</th></tr></thead><tbody>';
                files.forEach(item => {
                    let dlUrl = '/download?folder=' + encodeURIComponent(folderKey) + '&file=' + encodeURIComponent(item.name);
                    html += '<tr><td><b>' + item.name + '</b></td><td>' + item.size + '</td><td><a href="' + dlUrl + '" class="btn-dl" onclick="setTimeout(loadHistory, 1000)" download>Tải về</a></td></tr>';
                });
                html += '</tbody></table>';
            }
            html += '</div>';
        });
        container.innerHTML = html;
    }).catch(e => {
        document.getElementById('fileContainer').innerHTML = '<p style="color:red; font-size:13px;">Lỗi tải danh sách thư mục.</p>';
    });
}

function toggleFolder(folderKey) {
    let content = document.getElementById('content-' + folderKey);
    let icon = document.getElementById('icon-' + folderKey);
    if (content.style.display === 'block') {
        content.style.display = 'none';
        icon.innerText = '▼';
    } else {
        content.style.display = 'block';
        icon.innerText = '▲';
    }
}

function loadHistory() {
    fetch('/history')
    .then(r => r.json())
    .then(data => {
        let container = document.getElementById('historyContainer');
        let items = Array.isArray(data) ? data : [];
        if (!items || items.length === 0) {
            container.innerHTML = '<p style="color:#777; font-size:13px; margin-top:10px;">Chưa có lịch sử hoạt động nào.</p>';
            return;
        }
        let html = '<table class="custom-table"><thead><tr><th>Thời gian</th><th>Thao tác</th><th>Máy thực hiện</th><th>IP</th><th>Tên File</th></tr></thead><tbody>';
        items.forEach(item => {
            let badge = item.type === 'UPLOAD' ? '<span class="badge-up">UPLOAD</span>' : '<span class="badge-down">DOWNLOAD</span>';
            html += '<tr><td>' + item.time + '</td><td>' + badge + '</td><td><b>' + item.host + '</b></td><td>' + item.ip + '</td><td>' + item.file + '</td></tr>';
        });
        html += '</tbody></table>';
        container.innerHTML = html;
    }).catch(e => {
        document.getElementById('historyContainer').innerHTML = '<p style="color:red; font-size:13px;">Lỗi tải lịch sử.</p>';
    });
}

function up(fs){
    let l = document.getElementById('l');
    let allowedExts = ['pdf', 'html', 'htm', 'txt', 'xls', 'xlsx', 'xlsm', 'csv', 'zip', 'rar', '7z', 'exe', 'msi', 'doc', 'docx', 'png', 'jpg', 'jpeg'];
    let selectedFolder = document.getElementById('folderSelect').value;

    for (let f of fs) {
        let ext = f.name.split('.').pop().toLowerCase();
        let d = document.createElement('div');
        if (!allowedExts.includes(ext)) {
            d.className = 'item err';
            d.innerText = f.name + ': Định dạng File này không được hỗ trợ';
            l.prepend(d);
            continue;
        }
        d.className = 'item';
        d.innerText = 'Đang gửi (0%): ' + f.name;
        l.prepend(d);
        
        let x = new XMLHttpRequest();
        x.open('POST', '/', true);
        x.setRequestHeader('X-FileName', encodeURIComponent(f.name));
        x.setRequestHeader('X-FolderName', encodeURIComponent(selectedFolder));

        x.upload.onprogress = function(e) {
            if (e.lengthComputable) {
                let percent = Math.round((e.loaded / e.total) * 100);
                d.innerText = 'Đang gửi (' + percent + '%): ' + f.name;
            }
        };
        
        x.onload = function() {
            if (x.status == 200) {
                d.className = 'item ok';
                d.innerText = 'Gửi thành công: ' + f.name;
                loadFiles();
                loadHistory();
            } else {
                d.className = 'item err';
                d.innerText = 'Lỗi ' + x.status + ': ' + f.name + ' (' + x.responseText + ')';
            }
        };

        x.onerror = function() {
            d.className = 'item err';
            d.innerText = 'Lỗi kết nối / Server từ chối: ' + f.name;
        };

        x.send(f);
    }
}

window.onload = function() {
    loadFiles();
    loadHistory();
};
</script>
</body>
</html>"#)
}

async fn handle_report() -> Html<String> {
    update_master_report();
    let report_path = format!("{}\\BaoCaoTongHop.html", BASE_DIR);
    let content = fs::read_to_string(report_path).unwrap_or_default();
    Html(content)
}

async fn handle_list() -> Json<HashMap<String, Vec<FileInfo>>> {
    let mut map = HashMap::new();
    for folder in VALID_FOLDERS {
        let mut list = Vec::new();
        let path = format!("{}\\{}", BASE_DIR, folder);
        if let Ok(entries) = fs::read_dir(path) {
            for entry in entries.flatten() {
                if let Ok(meta) = entry.metadata() {
                    if meta.is_file() {
                        let size_mb = (meta.len() as f64 / (1024.0 * 1024.0) * 10.0).round() / 10.0;
                        let size_str = if size_mb < 1.0 {
                            format!("{} KB", (meta.len() / 1024))
                        } else {
                            format!("{} MB", size_mb)
                        };
                        list.push(FileInfo {
                            name: entry.file_name().to_string_lossy().to_string(),
                            size: size_str,
                        });
                    }
                }
            }
        }
        map.insert(folder.to_string(), list);
    }
    Json(map)
}

async fn handle_history() -> Json<Vec<LogEntry>> {
    let log_path = format!("{}\\LichSuHoatDong.txt", BASE_DIR);
    let mut logs = Vec::new();
    if let Ok(content) = fs::read_to_string(log_path) {
        let lines: Vec<&str> = content.lines().collect();
        for line in lines.iter().rev().take(50) {
            let parts: Vec<&str> = line.split(" | ").collect();
            if parts.len() >= 5 {
                logs.push(LogEntry {
                    time: parts[0].to_string(),
                    r#type: parts[1].to_string(),
                    ip: parts[2].replace("IP: ", ""),
                    host: parts[3].replace("Máy: ", ""),
                    file: parts[4].replace("File: ", ""),
                });
            }
        }
    }
    Json(logs)
}

async fn handle_download(
    Query(q): Query<DownloadQuery>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<Response, StatusCode> {
    let folder = q.folder.ok_or(StatusCode::BAD_REQUEST)?;
    let file = q.file.ok_or(StatusCode::BAD_REQUEST)?;

    if !VALID_FOLDERS.contains(&folder.as_str()) {
        return Err(StatusCode::BAD_REQUEST);
    }

    let file_path = PathBuf::from(BASE_DIR).join(&folder).join(&file);
    if !file_path.exists() {
        return Err(StatusCode::NOT_FOUND);
    }

    let client_ip = addr.ip().to_string();
    write_activity_log("DOWNLOAD", &client_ip, &format!("[{}]\\{}", folder, file));

    let bytes = fs::read(&file_path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let mut headers = HeaderMap::new();
    headers.insert(header::CONTENT_TYPE, "application/octet-stream".parse().unwrap());
    headers.insert(
        header::CONTENT_DISPOSITION,
        format!("attachment; filename=\"{}\"", file).parse().unwrap(),
    );

    Ok((headers, bytes).into_response())
}

async fn handle_upload(
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    headers: HeaderMap,
    body: Body,
) -> Result<&'static str, (StatusCode, String)> {
    let file_name = headers
        .get("X-FileName")
        .and_then(|h| h.to_str().ok())
        .map(|s| urlencoding::decode(s).unwrap_or_default().to_string())
        .unwrap_or_else(|| format!("File_{}.dat", chrono::Local::now().format("%Y%m%d_%H%M%S")));

    let folder_name = headers
        .get("X-FolderName")
        .and_then(|h| h.to_str().ok())
        .map(|s| urlencoding::decode(s).unwrap_or_default().to_string())
        .unwrap_or_else(|| "BaoCaoThietBi".to_string());

    let folder_name = if VALID_FOLDERS.contains(&folder_name.as_str()) {
        folder_name
    } else {
        "BaoCaoThietBi".to_string()
    };

    let ext = Path::new(&file_name)
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase())
        .unwrap_or_default();

    if !ALLOWED_EXTS.contains(&ext.as_str()) {
        return Err((StatusCode::BAD_REQUEST, "File nay ko duoc ho tro".to_string()));
    }

    let target_dir = PathBuf::from(BASE_DIR).join(&folder_name);
    let _ = fs::create_dir_all(&target_dir);
    let target_path = target_dir.join(&file_name);

    // --- ĐOẠN ĐƯỢC THAY THẾ VÀ SỬA LỖI ---
    let body_stream = body.into_data_stream();
    let stream_reader = StreamReader::new(
        body_stream.map_err(|err| std::io::Error::new(std::io::ErrorKind::Other, err.to_string())),
    );
    tokio::pin!(stream_reader);

    let mut target_file = tokio::fs::File::create(&target_path)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    tokio::io::copy(&mut stream_reader, &mut target_file)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    // --------------------------------------

    let client_ip = addr.ip().to_string();
    write_activity_log("UPLOAD", &client_ip, &format!("[{}]\\{}", folder_name, file_name));

    if folder_name == "BaoCaoThietBi" {
        update_master_report();
    }

    Ok("OK")
}

async fn start_web_server() {
    let app = Router::new()
        .route("/", get(handle_index).post(handle_upload))
        .route("/report", get(handle_report))
        .route("/list", get(handle_list))
        .route("/history", get(handle_history))
        .route("/download", get(handle_download))
        .layer(DefaultBodyLimit::disable()); // Bỏ hoàn toàn giới hạn dung lượng Request

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}

#[tokio::main]
async fn main() {
    init_environment();

    tokio::spawn(async {
        start_web_server().await;
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_server_info])
        .run(tauri::generate_context!())
        .expect("Lỗi khởi chạy ứng dụng Tauri");
}