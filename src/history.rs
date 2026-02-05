use crate::models::{CatatanKonversi, Satuan};
use std::fs::{self, File, OpenOptions};
use std::io::BufReader;
use std::path::Path;

pub fn simpan_riwayat(dari: &str, ke: &str, nilai_input: f64, nilai_output: Option<f64>, pesan_error: Option<String>) {
    let catatan = CatatanKonversi {
        satuan_asal: dari.to_string(),
        satuan_tujuan: ke.to_string(),
        nilai_input,
        nilai_output,
        pesan_error,
    };

    let path = "conversion.json";
    let mut daftar_catatan: Vec<CatatanKonversi> = Vec::new();

    if Path::new(path).exists() {
        match File::open(path) {
            Ok(file) => {
                let reader = BufReader::new(file);
                match serde_json::from_reader(reader) {
                    Ok(existing) => {
                        daftar_catatan = existing;
                    }
                    Err(_) => {
                        // JSON korup ditemukan. Backup dan mulai baru.
                        let backup_path = "conversion.json.bak";
                        eprintln!("Peringatan: File riwayat korup. Membuat cadangan ke '{}' dan memulai riwayat baru.", backup_path);
                        if let Err(e) = fs::rename(path, backup_path) {
                            eprintln!("Error: Gagal membuat cadangan riwayat: {}", e);
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Peringatan: Gagal membaca file riwayat: {}", e);
            }
        }
    }

    daftar_catatan.push(catatan);

    match OpenOptions::new().write(true).create(true).truncate(true).open(path) {
        Ok(file) => {
            if let Err(e) = serde_json::to_writer_pretty(file, &daftar_catatan) {
                eprintln!("Error: Gagal menyimpan riwayat: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Error: Gagal membuka file riwayat untuk ditulis: {}", e);
        }
    }
}

pub fn tampilkan_riwayat() {
    let path = "conversion.json";
    if !Path::new(path).exists() {
        println!("Belum ada riwayat konversi.");
        return;
    }

    match File::open(path) {
        Ok(file) => {
            let reader = BufReader::new(file);
            match serde_json::from_reader::<_, Vec<CatatanKonversi>>(reader) {
                Ok(daftar_catatan) => {
                    if daftar_catatan.is_empty() {
                        println!("Riwayat konversi kosong.");
                        return;
                    }
                    println!("Riwayat Konversi:");
                    for (i, catatan) in daftar_catatan.iter().enumerate() {
                        if let Some(error) = &catatan.pesan_error {
                            println!(
                                "{}. [GAGAL] {} {} -> {} (Error: {})",
                                i + 1,
                                catatan.nilai_input,
                                catatan.satuan_asal,
                                catatan.satuan_tujuan,
                                error
                            );
                        } else if let Some(output) = catatan.nilai_output {
                            // Coba dapatkan simbol jika memungkinkan, jika tidak gunakan string satuan mentah
                            let simbol_asal = Satuan::dari_str(&catatan.satuan_asal).map(|u| u.simbol()).unwrap_or(&catatan.satuan_asal);
                            let simbol_tujuan = Satuan::dari_str(&catatan.satuan_tujuan).map(|u| u.simbol()).unwrap_or(&catatan.satuan_tujuan);
        
                            println!(
                                "{}. {} {} = {} {}",
                                i + 1,
                                catatan.nilai_input,
                                simbol_asal,
                                output,
                                simbol_tujuan
                            );
                        }
                    }
                }
                Err(e) => {
                    println!("Error: Gagal membaca format data riwayat (file mungkin korup): {}", e);
                }
            }
        }
        Err(e) => {
            println!("Error: Gagal membuka file riwayat: {}", e);
        }
    }
}
