import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

export default function App() {
  const [serverUrl, setServerUrl] = useState("Đang khởi tạo...");
  const [copied, setCopied] = useState(false);

  useEffect(() => {
    invoke("get_server_info")
      .then((url) => setServerUrl(url))
      .catch((err) => {
        console.error("Lỗi IPC Tauri:", err);
        setServerUrl("Không kết nối được Backend Tauri");
      });
  }, []);

  const handleCopy = async () => {
    if (!serverUrl.startsWith("http")) return;
    await navigator.clipboard.writeText(serverUrl);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div style={{ padding: 25, fontFamily: "Segoe UI, sans-serif" }}>
      <h2 style={{ color: "#0f2942", marginTop: 0 }}>CỔNG TIẾP NHẬN BÁO CÁO (MÁY A)</h2>
      <p style={{ fontSize: 13, color: "#666" }}>
        Thư mục lưu gốc: <b>C:\ThuMucTong\</b>
      </p>

      <div style={{ background: "#e9f2ff", padding: 15, borderRadius: 8, border: "1px solid #b8daff" }}>
        <label style={{ display: "block", fontSize: 12, fontWeight: "bold", color: "#0f2942", marginBottom: 6 }}>
          LINK CỔNG TIẾP NHẬN VÀ TẢI FILE:
        </label>
        <div style={{ display: "flex", gap: 8 }}>
          <input
            type="text"
            value={serverUrl}
            readOnly
            style={{ flex: 1, padding: "8px 10px", border: "1px solid #ccc", borderRadius: 4, fontWeight: "bold" }}
          />
          <button
            onClick={handleCopy}
            style={{
              background: copied ? "#28a745" : "#007bff",
              color: "#fff",
              border: "none",
              padding: "8px 16px",
              borderRadius: 4,
              cursor: "pointer",
              fontWeight: "bold"
            }}
          >
            {copied ? "Đã chép" : "Sao chép"}
          </button>
        </div>
      </div>
    </div>
  );
}