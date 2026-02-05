use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Satuan {
    Celsius,
    Fahrenheit,
    Kelvin,
    Cm,
    Inch,
    Km,
    Miles,
    Kg,
    Gram,
    Lbs,
    Ounce,
    // Volume
    Liter,
    Gallon,
    Ml,
    // Waktu
    Detik,
    Menit,
    Jam,
    // Kecepatan
    Kmh, // km/h
    Mph, // mph
    Ms,  // m/s
    // Data
    Byte,
    KB,
    MB,
    GB,
}

#[derive(Debug, PartialEq)]
pub enum Kategori {
    Suhu,
    Panjang,
    Berat,
    Volume,
    Waktu,
    Kecepatan,
    Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CatatanKonversi {
    pub satuan_asal: String,
    pub satuan_tujuan: String,
    pub nilai_input: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nilai_output: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pesan_error: Option<String>,
}

impl Satuan {
    pub fn dari_str(s: &str) -> Option<Satuan> {
        match s.to_lowercase().as_str() {
            "celsius" => Some(Satuan::Celsius),
            "fahrenheit" => Some(Satuan::Fahrenheit),
            "kelvin" => Some(Satuan::Kelvin),
            "cm" => Some(Satuan::Cm),
            "inch" => Some(Satuan::Inch),
            "km" => Some(Satuan::Km),
            "miles" => Some(Satuan::Miles),
            "kg" => Some(Satuan::Kg),
            "gram" => Some(Satuan::Gram),
            "lbs" => Some(Satuan::Lbs),
            "ounce" => Some(Satuan::Ounce),
            "liter" | "l" => Some(Satuan::Liter),
            "gallon" | "gal" => Some(Satuan::Gallon),
            "ml" => Some(Satuan::Ml),
            "detik" | "second" | "sec" | "s" => Some(Satuan::Detik),
            "menit" | "minute" | "min" | "m" => Some(Satuan::Menit),
            "jam" | "hour" | "h" => Some(Satuan::Jam),
            "km/h" | "kmh" => Some(Satuan::Kmh),
            "mph" => Some(Satuan::Mph),
            "m/s" | "ms" => Some(Satuan::Ms),
            "byte" | "b" => Some(Satuan::Byte),
            "kb" => Some(Satuan::KB),
            "mb" => Some(Satuan::MB),
            "gb" => Some(Satuan::GB),
            _ => None,
        }
    }

    pub fn kategori(&self) -> Kategori {
        match self {
            Satuan::Celsius | Satuan::Fahrenheit | Satuan::Kelvin => Kategori::Suhu,
            Satuan::Cm | Satuan::Inch | Satuan::Km | Satuan::Miles => Kategori::Panjang,
            Satuan::Kg | Satuan::Gram | Satuan::Lbs | Satuan::Ounce => Kategori::Berat,
            Satuan::Liter | Satuan::Gallon | Satuan::Ml => Kategori::Volume,
            Satuan::Detik | Satuan::Menit | Satuan::Jam => Kategori::Waktu,
            Satuan::Kmh | Satuan::Mph | Satuan::Ms => Kategori::Kecepatan,
            Satuan::Byte | Satuan::KB | Satuan::MB | Satuan::GB => Kategori::Data,
        }
    }

    pub fn simbol(&self) -> &'static str {
        match self {
            Satuan::Celsius => "°C",
            Satuan::Fahrenheit => "°F",
            Satuan::Kelvin => "K",
            Satuan::Cm => "cm",
            Satuan::Inch => "inch",
            Satuan::Km => "km",
            Satuan::Miles => "miles",
            Satuan::Kg => "kg",
            Satuan::Gram => "g",
            Satuan::Lbs => "lbs",
            Satuan::Ounce => "oz",
            Satuan::Liter => "L",
            Satuan::Gallon => "gal",
            Satuan::Ml => "ml",
            Satuan::Detik => "s",
            Satuan::Menit => "min",
            Satuan::Jam => "h",
            Satuan::Kmh => "km/h",
            Satuan::Mph => "mph",
            Satuan::Ms => "m/s",
            Satuan::Byte => "B",
            Satuan::KB => "KB",
            Satuan::MB => "MB",
            Satuan::GB => "GB",
        }
    }
    
    pub fn nama(&self) -> &'static str {
        match self {
            Satuan::Celsius => "celsius",
            Satuan::Fahrenheit => "fahrenheit",
            Satuan::Kelvin => "kelvin",
            Satuan::Cm => "cm",
            Satuan::Inch => "inch",
            Satuan::Km => "km",
            Satuan::Miles => "miles",
            Satuan::Kg => "kg",
            Satuan::Gram => "gram",
            Satuan::Lbs => "lbs",
            Satuan::Ounce => "ounce",
            Satuan::Liter => "liter",
            Satuan::Gallon => "gallon",
            Satuan::Ml => "ml",
            Satuan::Detik => "detik",
            Satuan::Menit => "menit",
            Satuan::Jam => "jam",
            Satuan::Kmh => "km/h",
            Satuan::Mph => "mph",
            Satuan::Ms => "m/s",
            Satuan::Byte => "byte",
            Satuan::KB => "kb",
            Satuan::MB => "mb",
            Satuan::GB => "gb",
        }
    }

    pub fn semua() -> Vec<Satuan> {
        vec![
            Satuan::Celsius,
            Satuan::Fahrenheit,
            Satuan::Kelvin,
            Satuan::Cm,
            Satuan::Inch,
            Satuan::Km,
            Satuan::Miles,
            Satuan::Kg,
            Satuan::Gram,
            Satuan::Lbs,
            Satuan::Ounce,
            Satuan::Liter,
            Satuan::Gallon,
            Satuan::Ml,
            Satuan::Detik,
            Satuan::Menit,
            Satuan::Jam,
            Satuan::Kmh,
            Satuan::Mph,
            Satuan::Ms,
            Satuan::Byte,
            Satuan::KB,
            Satuan::MB,
            Satuan::GB,
        ]
    }
}

impl Kategori {
    pub fn nama(&self) -> &'static str {
        match self {
            Kategori::Suhu => "suhu",
            Kategori::Panjang => "panjang",
            Kategori::Berat => "berat",
            Kategori::Volume => "volume",
            Kategori::Waktu => "waktu",
            Kategori::Kecepatan => "kecepatan",
            Kategori::Data => "data",
        }
    }

    pub fn satuan_satuan(&self) -> Vec<Satuan> {
        match self {
            Kategori::Suhu => vec![Satuan::Celsius, Satuan::Fahrenheit, Satuan::Kelvin],
            Kategori::Panjang => vec![Satuan::Cm, Satuan::Inch, Satuan::Km, Satuan::Miles],
            Kategori::Berat => vec![Satuan::Kg, Satuan::Gram, Satuan::Lbs, Satuan::Ounce],
            Kategori::Volume => vec![Satuan::Liter, Satuan::Gallon, Satuan::Ml],
            Kategori::Waktu => vec![Satuan::Detik, Satuan::Menit, Satuan::Jam],
            Kategori::Kecepatan => vec![Satuan::Kmh, Satuan::Mph, Satuan::Ms],
            Kategori::Data => vec![Satuan::Byte, Satuan::KB, Satuan::MB, Satuan::GB],
        }
    }
}
