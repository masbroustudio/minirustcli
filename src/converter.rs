use crate::models::{Kategori, Satuan};

pub fn konversi(nilai: f64, dari: Satuan, ke: Satuan) -> Result<f64, String> {
    // Validasi nilai numerik
    if nilai.is_nan() || nilai.is_infinite() {
        return Err("Nilai input tidak valid (NaN atau Infinity)".to_string());
    }

    if dari == ke {
        return Ok(nilai);
    }

    if dari.kategori() != ke.kategori() {
        return Err(format!(
            "Tidak dapat mengonversi satuan yang berbeda kategori: [{}] {} -> [{}] {}",
            dari.kategori().nama(),
            dari.nama(),
            ke.kategori().nama(),
            ke.nama()
        ));
    }

    // Validasi spesifik
    if dari == Satuan::Kelvin && nilai < 0.0 {
        return Err("Nilai Kelvin tidak boleh negatif (< 0)".to_string());
    }

    match dari.kategori() {
        Kategori::Suhu => Ok(konversi_suhu(nilai, dari, ke)),
        Kategori::Panjang => Ok(konversi_panjang(nilai, dari, ke)),
        Kategori::Berat => Ok(konversi_berat(nilai, dari, ke)),
        Kategori::Volume => Ok(konversi_volume(nilai, dari, ke)),
        Kategori::Waktu => Ok(konversi_waktu(nilai, dari, ke)),
        Kategori::Kecepatan => Ok(konversi_kecepatan(nilai, dari, ke)),
        Kategori::Data => Ok(konversi_data(nilai, dari, ke)),
    }
}

fn konversi_suhu(nilai: f64, dari: Satuan, ke: Satuan) -> f64 {
    // Konversi ke Celsius terlebih dahulu
    let celsius = match dari {
        Satuan::Celsius => nilai,
        Satuan::Fahrenheit => (nilai - 32.0) * 5.0 / 9.0,
        Satuan::Kelvin => nilai - 273.15,
        _ => unreachable!("Seharusnya satuan suhu"),
    };

    // Konversi dari Celsius ke target
    match ke {
        Satuan::Celsius => celsius,
        Satuan::Fahrenheit => (celsius * 9.0 / 5.0) + 32.0,
        Satuan::Kelvin => celsius + 273.15,
        _ => unreachable!("Seharusnya satuan suhu"),
    }
}

fn konversi_panjang(nilai: f64, dari: Satuan, ke: Satuan) -> f64 {
    // Konversi ke Meter terlebih dahulu (Standar SI)
    let meter = match dari {
        Satuan::Cm => nilai / 100.0,
        Satuan::Inch => nilai * 0.0254,
        Satuan::Km => nilai * 1000.0,
        Satuan::Miles => nilai * 1609.344,
        _ => unreachable!("Seharusnya satuan panjang"),
    };

    match ke {
        Satuan::Cm => meter * 100.0,
        Satuan::Inch => meter / 0.0254,
        Satuan::Km => meter / 1000.0,
        Satuan::Miles => meter / 1609.344,
        _ => unreachable!("Seharusnya satuan panjang"),
    }
}

fn konversi_berat(nilai: f64, dari: Satuan, ke: Satuan) -> f64 {
    // Konversi ke Kilogram terlebih dahulu (Standar SI)
    let kg = match dari {
        Satuan::Kg => nilai,
        Satuan::Gram => nilai / 1000.0,
        Satuan::Lbs => nilai * 0.45359237,
        Satuan::Ounce => nilai * 0.0283495,
        _ => unreachable!("Seharusnya satuan berat"),
    };

    match ke {
        Satuan::Kg => kg,
        Satuan::Gram => kg * 1000.0,
        Satuan::Lbs => kg / 0.45359237,
        Satuan::Ounce => kg / 0.0283495,
        _ => unreachable!("Seharusnya satuan berat"),
    }
}

fn konversi_volume(nilai: f64, dari: Satuan, ke: Satuan) -> f64 {
    // Konversi ke Liter terlebih dahulu
    // 1 Gallon (US) = 3.785411784 L
    let liter = match dari {
        Satuan::Liter => nilai,
        Satuan::Gallon => nilai * 3.785411784,
        Satuan::Ml => nilai / 1000.0,
        _ => unreachable!("Seharusnya satuan volume"),
    };

    match ke {
        Satuan::Liter => liter,
        Satuan::Gallon => liter / 3.785411784,
        Satuan::Ml => liter * 1000.0,
        _ => unreachable!("Seharusnya satuan volume"),
    }
}

fn konversi_waktu(nilai: f64, dari: Satuan, ke: Satuan) -> f64 {
    // Konversi ke Detik terlebih dahulu
    let detik = match dari {
        Satuan::Detik => nilai,
        Satuan::Menit => nilai * 60.0,
        Satuan::Jam => nilai * 3600.0,
        _ => unreachable!("Seharusnya satuan waktu"),
    };

    match ke {
        Satuan::Detik => detik,
        Satuan::Menit => detik / 60.0,
        Satuan::Jam => detik / 3600.0,
        _ => unreachable!("Seharusnya satuan waktu"),
    }
}

fn konversi_kecepatan(nilai: f64, dari: Satuan, ke: Satuan) -> f64 {
    // Konversi ke Meter per detik (m/s) terlebih dahulu
    // 1 km/h = 1000m / 3600s = 1/3.6 m/s
    // 1 mph = 1609.344m / 3600s = 0.44704 m/s
    let ms = match dari {
        Satuan::Ms => nilai,
        Satuan::Kmh => nilai / 3.6,
        Satuan::Mph => nilai * 0.44704,
        _ => unreachable!("Seharusnya satuan kecepatan"),
    };

    match ke {
        Satuan::Ms => ms,
        Satuan::Kmh => ms * 3.6,
        Satuan::Mph => ms / 0.44704,
        _ => unreachable!("Seharusnya satuan kecepatan"),
    }
}

fn konversi_data(nilai: f64, dari: Satuan, ke: Satuan) -> f64 {
    // Konversi ke Byte terlebih dahulu (menggunakan 1024 / Binary prefixes)
    let byte = match dari {
        Satuan::Byte => nilai,
        Satuan::KB => nilai * 1024.0,
        Satuan::MB => nilai * 1024.0 * 1024.0,
        Satuan::GB => nilai * 1024.0 * 1024.0 * 1024.0,
        _ => unreachable!("Seharusnya satuan data"),
    };

    match ke {
        Satuan::Byte => byte,
        Satuan::KB => byte / 1024.0,
        Satuan::MB => byte / (1024.0 * 1024.0),
        Satuan::GB => byte / (1024.0 * 1024.0 * 1024.0),
        _ => unreachable!("Seharusnya satuan data"),
    }
}
