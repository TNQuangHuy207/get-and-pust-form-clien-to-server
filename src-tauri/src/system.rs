use std::fs;
use std::path::Path;
use std::process::Command;
use local_ip_address::local_ip;

pub fn init_environment() {
    // 1. Khởi tạo cấu trúc thư mục C:\ThuMucTong[cite: 1]
    let base_dir = "C:\\ThuMucTong";[cite: 1]
    let folders = vec!["BaoCaoThietBi", "KhoPhanMem", "TaiLieuKhac"];[cite: 1]

    for folder in folders {
        let path = format!("{}\\{}", base_dir, folder);
        if !Path::new(&path).exists() {
            let _ = fs::create_dir_all(&path);
        }
    }

    // 2. Tự động giải phóng Port 8080 và mở Rule Tường lửa[cite: 1]
    configure_network();
}

fn configure_network() {
    // Giải phóng port 8080 cũ nếu đang bị chiếm dụng[cite: 1]
    let _ = Command::new("powershell")
        .args(["-Command", "Get-NetTCPConnection -LocalPort 8080 -ErrorAction SilentlyContinue | ForEach-Object { Stop-Process -Id $_.OwningProcess -Force -ErrorAction SilentlyContinue }"])[cite: 1]
        .output();

    // Thêm Firewall rule cho Port 8080[cite: 1]
    let _ = Command::new("netsh")
        .args(["advfirewall", "firewall", "add", "rule", "name=Tool_Nhan_File_8080", "dir=in", "action=allow", "protocol=TCP", "localport=8080"])[cite: 1]
        .output();
}

pub fn get_local_server_url() -> String {
    match local_ip() {
        Ok(ip) => format!("http://{}:8080/", ip),[cite: 1]
        Err(_) => "http://127.0.0.1:8080/".to_string(),
    }
}