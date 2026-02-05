use crate::models::{Kategori, Satuan};

pub fn konversi(nilai: f64, dari: Satuan, ke: Satuan) -> Result<f64, String> {
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

    match dari.kategori() {
        Kategori::Suhu => Ok(konversi_suhu(nilai, dari, ke)),
        Kategori::Panjang => Ok(konversi_panjang(nilai, dari, ke)),
        Kategori::Berat => Ok(konversi_berat(nilai, dari, ke)),
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
    // cm -> m: / 100
    // inch -> m: * 0.0254
    // km -> m: * 1000
    // miles -> m: * 1609.344
    
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
    // gram -> kg: / 1000
    // lbs -> kg: * 0.45359237
    // ounce -> kg: * 0.0283495
    
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
