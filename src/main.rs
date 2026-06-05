use validator::Validate;

pub mod custom_validator {
    use validator::ValidationError;

    pub fn validasi_username(username: &str) -> Result<(), ValidationError> {
        // 1. Cek panjang: 3 sampai 20 karakter
        if username.len() < 3 || username.len() > 20 {
            let mut err = ValidationError::new("username_length_invalid");
            err.add_param("min".into(), &3);
            err.add_param("max".into(), &20);
            return Err(err);
        }

        // 2. Cek karakter pertama: harus huruf kecil (bukan underscore!)
        let first_char = username.chars().next().unwrap();
        if !first_char.is_ascii_lowercase() {
            // ← underscore TIDAK boleh di awal
            let mut err = ValidationError::new("username_first_char_invalid");
            err.add_param("valid_first_chars".into(), &"a-z");
            return Err(err);
        }

        // 3. Cek karakter terakhir: tidak boleh underscore
        if username.ends_with('_') {
            let mut err = ValidationError::new("username_last_char_invalid");
            err.add_param("reason".into(), &"cannot end with underscore");
            return Err(err);
        }

        // 4. Cek karakter: hanya huruf kecil, angka, dan underscore
        if !username
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        {
            let mut err = ValidationError::new("username_invalid_chars");
            err.add_param("allowed".into(), &"a-z, 0-9, _");
            return Err(err);
        }

        Ok(())
    }
}

#[derive(Debug, Validate)]
struct User {
    #[allow(dead_code)]
    id: u32,

    // ✅ PERBAIKAN: hapus tanda petik!
    #[validate(custom(function = "custom_validator::validasi_username"))]
    username: String,
}

fn main() {
    println!("═══════════════════════════════════");
    println!("   USERNAME VALIDATION TEST");
    println!("═══════════════════════════════════\n");

    let test_users = vec![
        ("john_doe", true, "valid username dengan underscore"),
        ("john_doe_123", true, "valid username dengan angka"),
        ("jo", false, "terlalu pendek (min 3)"),
        (
            "john_doe_12345678901234567890",
            false,
            "terlalu panjang (max 20)",
        ),
        ("2john", false, "diawali angka"),
        ("john_", false, "diakhiri underscore"),
        ("JohnDoe", false, "mengandung huruf besar"),
        ("john@doe", false, "karakter terlarang (@)"),
        ("_john", false, "diawali underscore"),
        ("j", false, "hanya 1 karakter"),
    ];

    for (username, should_be_valid, description) in test_users {
        let user = User {
            id: 1,
            username: username.to_string(),
        };

        match user.validate() {
            Ok(_) => {
                if should_be_valid {
                    println!("✅ '{}' → VALID (sesuai: {})", username, description);
                } else {
                    println!(
                        "❌ '{}' → LULUS (seharusnya TIDAK valid!): {}",
                        username, description
                    );
                }
            }
            Err(e) => {
                if should_be_valid {
                    println!(
                        "❌ '{}' → ERROR (seharusnya valid!): {}",
                        username, description
                    );
                    println!("   Error details: {:?}\n", e);
                } else {
                    println!("✅ '{}' → TIDAK VALID (sesuai: {})", username, description);
                }
            }
        }
    }
}
