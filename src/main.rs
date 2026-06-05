use validator::Validate;

// ==========================
// CUSTOM VALIDATORS
// ==========================
pub mod custom_validator {
    use validator::ValidationError;

    pub fn validasi_kode_product(kode: &str) -> Result<(), ValidationError> {
        if !kode.starts_with("PRD-") {
            let mut err = ValidationError::new("kode_product_invalid");
            err.add_param("format".into(), &"PRD-XXXX");
            return Err(err);
        }

        let number_part = &kode[4..];

        if number_part.len() != 4 || !number_part.chars().all(|c| c.is_ascii_digit()) {
            let mut err = ValidationError::new("kode_product_invalid_format");
            err.add_param("example".into(), &"PRD-1234");
            return Err(err);
        }

        Ok(())
    }

    pub fn validasi_diskon(diskon: u8) -> Result<(), ValidationError> {
        match diskon {
            0 => Ok(()),

            5..=30 => Ok(()),

            31..=50 => {
                let mut err = ValidationError::new("diskon_tinggi");
                err.add_param("max_normal".into(), &30);
                Err(err)
            }

            _ => {
                let mut err = ValidationError::new("diskon_invalid");
                err.add_param("max".into(), &50);
                Err(err)
            }
        }
    }
}

// ==========================
// STRUCT
// ==========================
#[derive(Debug, Validate)]
struct Product {
    #[validate(range(min = 1))]
    id: u64,

    #[validate(length(min = 1))]
    name: String,

    #[validate(custom(function = "custom_validator::validasi_kode_product"))]
    kode_product: String,

    #[validate(custom(function = "custom_validator::validasi_diskon"))]
    diskon: u8,
}

fn main() {
    println!("═══════════════════════════════════");
    println!("   CUSTOM VALIDATION");
    println!("═══════════════════════════════════");

    // -------------------------
    // VALID
    // -------------------------
    let product_valid = Product {
        id: 1,
        name: "Laptop".to_string(),
        kode_product: "PRD-1234".to_string(),
        diskon: 20,
    };

    match product_valid.validate() {
        Ok(_) => {
            println!("✅ Product valid");
            println!("{:#?}", product_valid);
        }
        Err(e) => {
            println!("❌ Error:");
            println!("{:#?}", e);
        }
    }

    println!();

    // -------------------------
    // KODE INVALID
    // -------------------------
    let product_invalid1 = Product {
        id: 2,
        name: "Mouse".to_string(),
        kode_product: "ABC-123".to_string(),
        diskon: 10,
    };

    match product_invalid1.validate() {
        Ok(_) => println!("✅ Product valid"),
        Err(e) => {
            println!("❌ Error kode product:");
            println!("{:#?}", e);
        }
    }

    println!();

    // -------------------------
    // DISKON INVALID
    // -------------------------
    let product_invalid2 = Product {
        id: 3,
        name: "Keyboard".to_string(),
        kode_product: "PRD-5678".to_string(),
        diskon: 70,
    };

    match product_invalid2.validate() {
        Ok(_) => println!("✅ Product valid"),
        Err(e) => {
            println!("❌ Error diskon:");
            println!("{:#?}", e);
        }
    }
}
