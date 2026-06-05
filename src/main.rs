use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug)]
struct Order {
    jumlah_barang: u8,
    harga_satuan: u64,
    diskon: u8,
    kode_promo: Option<String>,
}

impl Validate for Order {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Schema validation 1: Diskon tidak boleh > 50%
        if self.diskon > 50 {
            let mut err = ValidationError::new("diskon_terlalu_besar");
            err.add_param("max".into(), &50);
            errors.add("diskon", err);
        }

        // Schema validation 2: Total < 100.000 → diskon max 10%
        let total = self.jumlah_barang as u64 * self.harga_satuan;
        if total < 100_000 && self.diskon > 10 {
            let mut err = ValidationError::new("diskon_melebihi_batas");
            err.add_param("max_diskon".into(), &10);
            err.add_param("total".into(), &total);
            errors.add("diskon", err);
        }

        // Schema validation 3: Kode promo "GRATIS" → minimal beli 5 barang
        if self.kode_promo == Some("GRATIS".to_string()) && self.jumlah_barang < 5 {
            let mut err = ValidationError::new("minimal_pembelian_untuk_promo");
            err.add_param("min_jumlah".into(), &5);
            errors.add("kode_promo", err);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn main() {
    let order1 = Order {
        jumlah_barang: 10,
        harga_satuan: 20_000,
        diskon: 15, // total 200.000, diskon 15% OK
        kode_promo: None,
    };

    let order2 = Order {
        jumlah_barang: 2,
        harga_satuan: 30_000,
        diskon: 20, // total 60.000, diskon 20% ❌ (max 10%)
        kode_promo: None,
    };

    println!("Order 1 valid: {:?}", order1.validate().is_ok());
    println!("Order 2 valid: {:?}", order2.validate().is_ok());
}
