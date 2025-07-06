# 🌐 Onion Link Manager  
![Rust](https://img.shields.io/badge/language-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-blue)  

*[Rust 1.70+] A sleek GUI to organize and access your Tor onion links*  

---

## ✨ Features  
- **Manage** your onion links with ease  
- **One-click** opening in Tor Browser  
- **Copy** URLs with visual feedback  
- **Persistent** storage (JSON backed)  
- **Modern UI** powered by egui  
- **Lightweight** and fast  

---

## 🚀 Installation  

### For Debian/Ubuntu  
```bash  
wget https://github.com/MichMazbout/onion-link-manager/onion-link-manager.deb  
sudo dpkg -i onion-link-manager.deb
```
From Source
```bash
git clone https://github.com/MichMazbout/onion-link-manager  
cd onion-link-manager  
cargo build --release  # Requires Rust 1.70+
./target/release/onion-link-manager
```
## 🖥️ Usage Guide

### Adding a New Link
1. **Enter Title**: Type a descriptive name in the "Title" field  
   Example: `DuckDuckGo Mirror`
2. **Enter URL**: Paste the onion address  
   Example: `http://3g2upl4pq6kufc4m.onion`
3. **Save**: Click the "Add" button or press `Enter`

### Managing Existing Links
| Action        | How To                            | Visual Feedback            |
|---------------|-----------------------------------|----------------------------|
| **Open**      | Click the URL text                | Launches in Tor Browser    |
| **Copy**      | Click the "Copy" button           | "Copied!" appears briefly  |
| **Edit**      | Click "Edit" → Modify → "Save"    | Fields become interactive  |
| **Delete**    | Click "Delete"                    | Immediate removal          |

### Example Session
```bash
# Typical user flow:
1. Add "SecureDrop" with URL "http://xxxxxxxx.onion"
2. Click URL to verify it loads in Tor Browser
3. Click "Copy" to share with colleagues
4. Later: Click "Edit" to update the URL when it changes
