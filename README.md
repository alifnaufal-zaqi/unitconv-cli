# unitconv-cli

**unitconv-cli** adalah aplikasi Command-Line Interface (CLI) untuk mengonversi satuan suhu dan panjang yang ditulis dengan Rust.  
Aplikasi ini mendukung konversi suhu (_Celsius, Fahrenheit, Kelvin_) dan panjang (_cm, inch, km, miles_), serta menyimpan riwayat konversi ke file `history.json`.

---

## 🔹 Fitur

- Konversi antar satuan suhu:
  - Celsius ↔ Fahrenheit
  - Celsius ↔ Kelvin
  - Fahrenheit ↔ Kelvin
- Konversi antar satuan panjang:
  - cm, inch, km, miles (semua arah konversi)
- Menyimpan riwayat konversi ke file `history.json`
- Perintah `history` untuk menampilkan daftar semua konversi sebelumnya
- Subcommand `list` untuk menampilkan semua satuan yang didukung
- Output hasil konversi dengan format `{nilai} {satuan}`

---

## 🚀 Instalasi

1. Clone repository ini:

```bash
git clone https://github.com/alifnaufal-zaqi/unitconv-cli.git
cd unitconv-cli
```

2. Build proyek menggunakan Cargo:

```bash
cargo build --release
```

3. Jalankan aplikasi:

```bash
cargo run -- convert --from celsius --to fahrenheit --value 10
```
