use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Validate, Debug, Deserialize, Serialize)]
struct Order {
    #[validate(range(min = 1))]
    quantity: i32,

    #[validate(range(min = 0))]
    price: i32,

    total: i32,
}

impl Order {
    pub fn validate_all(&self) -> Result<(), ValidationErrors> {
        // 1️⃣ Validasi dari attribute
        self.validate()?;

        // 2️⃣ Cross-field validation
        let expected = self.quantity * self.price;
        if self.total != expected {
            let mut errors = ValidationErrors::new();
            let mut err = ValidationError::new("total_mismatch");
            err.add_param("expected".into(), &expected);
            err.add_param("actual".into(), &self.total);
            errors.add("total", err);
            return Err(errors);
        }

        Ok(())
    }
}

fn main() {
    // ========== SIMULASI DATA DARI EXTERNAL (JSON) ==========

    // Contoh 1: JSON dari user lewat API
    let json_data = r#"
    {
        "quantity": 5,
        "price": 1000,
        "total": 6000
    }
    "#;

    println!("📨 JSON dari Client:");
    println!("{}", json_data);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    // PARSE JSON ke Rust Struct
    match serde_json::from_str::<Order>(json_data) {
        Ok(order) => {
            println!("✅ JSON berhasil di-parse: {:?}", order);

            // VALIDASI struct nya!
            match order.validate_all() {
                Ok(()) => println!("✅ Order valid! siap diproses"),
                Err(e) => println!("❌ Validasi gagal: {}", e),
            }
        }
        Err(e) => {
            println!("❌ JSON invalid: {}", e);
        }
    }

    println!("\n═══════════════════════════════════════════════════════\n");

    // Contoh 2: JSON yang VALID
    let json_valid = r#"
    {
        "quantity": 3,
        "price": 2000,
        "total": 6000
    }
    "#;

    println!("📨 JSON Valid:");
    println!("{}", json_valid);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    match serde_json::from_str::<Order>(json_valid) {
        Ok(order) => {
            println!("✅ Parse: {:?}", order);
            match order.validate_all() {
                Ok(()) => println!("✅ VALID! Lanjut proses..."),
                Err(e) => println!("❌ Invalid: {}", e),
            }
        }
        Err(e) => println!("❌ JSON error: {}", e),
    }
}
