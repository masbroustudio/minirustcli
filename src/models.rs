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
}

#[derive(Debug, PartialEq)]
pub enum Kategori {
    Suhu,
    Panjang,
    Berat,
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
            _ => None,
        }
    }

    pub fn kategori(&self) -> Kategori {
        match self {
            Satuan::Celsius | Satuan::Fahrenheit | Satuan::Kelvin => Kategori::Suhu,
            Satuan::Cm | Satuan::Inch | Satuan::Km | Satuan::Miles => Kategori::Panjang,
            Satuan::Kg | Satuan::Gram | Satuan::Lbs | Satuan::Ounce => Kategori::Berat,
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
        ]
    }
}

impl Kategori {
    pub fn nama(&self) -> &'static str {
        match self {
            Kategori::Suhu => "suhu",
            Kategori::Panjang => "panjang",
            Kategori::Berat => "berat",
        }
    }

    pub fn satuan_satuan(&self) -> Vec<Satuan> {
        match self {
            Kategori::Suhu => vec![Satuan::Celsius, Satuan::Fahrenheit, Satuan::Kelvin],
            Kategori::Panjang => vec![Satuan::Cm, Satuan::Inch, Satuan::Km, Satuan::Miles],
            Kategori::Berat => vec![Satuan::Kg, Satuan::Gram, Satuan::Lbs, Satuan::Ounce],
        }
    }
}
