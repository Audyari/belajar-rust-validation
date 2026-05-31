use std::borrow::Cow;
use validator::Validate;

#[derive(Debug, Validate)]
struct Negara {
    #[validate(length(min = 2, message = "Kode negara minimal 2 karakter"))]
    kode: String,
}

#[derive(Debug, Validate)]
struct Alamat {
    #[validate(length(min = 5, message = "Jalan minimal 5 karakter"))]
    jalan: String,

    #[validate(nested)]
    negara: Negara,
}

#[derive(Debug, Validate)]
struct Pelanggan {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    nama: String,

    #[validate(nested)]
    alamat: Alamat,
}

impl Pelanggan {
    fn validate_all(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validasi level 1: Pelanggan sendiri
        if let Err(e) = self.validate() {
            for (field, errs) in e.field_errors() {
                for err in errs {
                    let msg = err
                        .message
                        .clone()
                        .unwrap_or_else(|| Cow::Owned(format!("{:?}", err.code)));
                    errors.push(format!("{}: {}", field, msg));
                }
            }
        }

        // Validasi level 2: Alamat (pake self.alamat, BUKAN alamat doang!)
        if let Err(e) = self.alamat.validate() {
            for (field, errs) in e.field_errors() {
                for err in errs {
                    let msg = err
                        .message
                        .clone()
                        .unwrap_or_else(|| Cow::Owned(format!("{:?}", err.code)));
                    errors.push(format!("alamat.{}: {}", field, msg));
                }
            }
        }

        // Validasi level 3: Negara (pake self.alamat.negara!)
        if let Err(e) = self.alamat.negara.validate() {
            for (field, errs) in e.field_errors() {
                for err in errs {
                    let msg = err
                        .message
                        .clone()
                        .unwrap_or_else(|| Cow::Owned(format!("{:?}", err.code)));
                    errors.push(format!("alamat.negara.{}: {}", field, msg));
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}
fn main() {
    // ========== KASUS 1: VALID ==========
    let pelanggan_valid = Pelanggan {
        nama: "Budi Santoso".to_string(),
        alamat: Alamat {
            jalan: "Jl. Merdeka No. 123".to_string(),
            negara: Negara {
                kode: "ID".to_string(),
            },
        },
    };

    println!("🔍 Validating Valid Customer...");
    if let Err(e) = pelanggan_valid.validate() {
        println!("❌ Error: {}", e);
    } else {
        println!("✅ Valid!");
    }

    println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // ========== KASUS 2: INVALID (Nested Errors) ==========
    let data = Pelanggan {
        nama: "An".to_string(), // ❌ error
        alamat: Alamat {
            jalan: "Jl".to_string(), // ❌ error
            negara: Negara {
                kode: "I".to_string(), // ❌ error
            },
        },
    };

    println!("🔍 Test Nested Validation dengan validator v0.20.0");
    match data.validate_all() {
        Ok(_) => println!("✅ Valid!"),
        Err(errors) => {
            println!("❌ Errors:");
            for err in errors {
                println!("   • {}", err);
            }
        }
    }
}
