use clap::Parser;
use minirustcli::cli::{Cli, Commands};
use minirustcli::converter;
use minirustcli::history;
use minirustcli::models::Satuan;
use std::process;
use std::cmp::min;

// Implementasi sederhana jarak Levenshtein
fn hitung_jarak_levenshtein(s1: &str, s2: &str) -> usize {
    let panjang1 = s1.chars().count();
    let panjang2 = s2.chars().count();
    let mut matriks = vec![vec![0; panjang2 + 1]; panjang1 + 1];

    for i in 0..=panjang1 { matriks[i][0] = i; }
    for j in 0..=panjang2 { matriks[0][j] = j; }

    for (i, c1) in s1.chars().enumerate() {
        for (j, c2) in s2.chars().enumerate() {
            let biaya = if c1 == c2 { 0 } else { 1 };
            matriks[i + 1][j + 1] = min(
                min(matriks[i][j + 1] + 1, matriks[i + 1][j] + 1),
                matriks[i][j] + biaya
            );
        }
    }
    matriks[panjang1][panjang2]
}

fn saran_satuan(input: &str) -> Option<String> {
    let daftar_satuan = Satuan::semua();
    let mut cocok_terbaik = None;
    let mut jarak_min = usize::MAX;

    for satuan in daftar_satuan {
        let nama = satuan.nama();
        let jarak = hitung_jarak_levenshtein(input.to_lowercase().as_str(), nama);
        if jarak < jarak_min {
            jarak_min = jarak;
            cocok_terbaik = Some(nama);
        }
        // Periksa simbol (misal "kg", "cm")
        let simbol = satuan.simbol().replace("°", ""); // Hapus simbol derajat
        let jarak_simbol = hitung_jarak_levenshtein(input.to_lowercase().as_str(), &simbol.to_lowercase());
        if jarak_simbol < jarak_min {
            jarak_min = jarak_simbol;
            cocok_terbaik = Some(satuan.nama());
        }
    }

    // Hanya sarankan jika jarak kecil relatif terhadap panjang string
    if jarak_min <= 3 {
        cocok_terbaik.map(|s| s.to_string())
    } else {
        None
    }
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Convert { from, to, value } => {
            // Mapping argumen
            let dari = from;
            let ke = to;
            let nilai = value;

            let opsi_satuan_asal = Satuan::dari_str(&dari);
            if opsi_satuan_asal.is_none() {
                let mut pesan = format!("Satuan asal '{}' tidak dikenali.", dari);
                if let Some(saran) = saran_satuan(&dari) {
                    pesan.push_str(&format!(" Apakah maksud Anda '{}'?", saran));
                }
                eprintln!("Error: [KESALAHAN] {}", pesan);
                history::simpan_riwayat(&dari, &ke, nilai, None, Some(pesan));
                process::exit(1);
            }
            let satuan_asal = opsi_satuan_asal.unwrap();

            let opsi_satuan_tujuan = Satuan::dari_str(&ke);
            if opsi_satuan_tujuan.is_none() {
                let mut pesan = format!("Satuan tujuan '{}' tidak dikenali.", ke);
                if let Some(saran) = saran_satuan(&ke) {
                    pesan.push_str(&format!(" Apakah maksud Anda '{}'?", saran));
                }
                eprintln!("Error: [KESALAHAN] {}", pesan);
                history::simpan_riwayat(&dari, &ke, nilai, None, Some(pesan));
                process::exit(1);
            }
            let satuan_tujuan = opsi_satuan_tujuan.unwrap();

            match converter::konversi(nilai, satuan_asal, satuan_tujuan) {
                Ok(hasil) => {
                    // Hitung konversi ke satuan lain dalam kategori yang sama
                    let mut lainnya = Vec::new();
                    for satuan in satuan_asal.kategori().satuan_satuan() {
                        if satuan != satuan_asal && satuan != satuan_tujuan {
                            if let Ok(val) = converter::konversi(nilai, satuan_asal, satuan) {
                                lainnya.push(format!("{} {}", val, satuan.simbol()));
                            }
                        }
                    }

                    // Format output
                    let string_lainnya = if lainnya.is_empty() {
                        String::new()
                    } else {
                        format!(" ({})", lainnya.join(", "))
                    };

                    println!("{} {} = {} {}{}", nilai, satuan_asal.simbol(), hasil, satuan_tujuan.simbol(), string_lainnya);
                    history::simpan_riwayat(&dari, &ke, nilai, Some(hasil), None);
                }
                Err(pesan) => {
                    eprintln!("Error: [KESALAHAN] {}", pesan);
                    history::simpan_riwayat(&dari, &ke, nilai, None, Some(pesan));
                    process::exit(1);
                }
            }
        }
        Commands::List => {
            println!("Satuan yang didukung:");
            println!("1. [suhu] celsius, fahrenheit, kelvin");
            println!("2. [panjang] cm, inch, km, miles");
            println!("3. [berat] kg, gram, lbs, ounce");
            println!("4. [volume] liter, gallon, ml");
            println!("5. [waktu] detik, menit, jam");
            println!("6. [kecepatan] km/h, mph, m/s");
            println!("7. [data] byte, kb, mb, gb");
        }
        Commands::History => {
            history::tampilkan_riwayat();
        }
    }
}
