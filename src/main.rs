use validator::{Validate, ValidationError, ValidationErrors};

#[allow(dead_code)]
#[derive(Debug, Validate)]
struct Transaction {
    amount: u64,
    balance: u64,

    #[validate(custom(
        function = "validate_withdrawal",
        message = "Saldo tidak mencukupi",
        code = "INSUFFICIENT_BALANCE"
    ))]
    withdrawal: u64,
}

fn validate_withdrawal(withdrawal: u64) -> Result<(), ValidationError> {
    if withdrawal == 0 {
        let mut err = ValidationError::new("WITHDRAWAL_ZERO");
        err.message = Some("❌ Tidak bisa tarik tunai Rp 0".into());
        return Err(err);
    }

    if withdrawal % 10000 != 0 {
        let mut err = ValidationError::new("WITHDRAWAL_INVALID_AMOUNT");
        err.message = Some("❌ Penarikan harus kelipatan Rp 10.000".into());
        err.add_param("kelipatan".into(), &10000);
        return Err(err);
    }

    Ok(())
}

#[derive(Debug)]
struct Transaction2 {
    withdrawal: u64,
    balance: u64,
}

impl Validate for Transaction2 {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        if self.withdrawal > self.balance {
            let mut err = ValidationError::new("INSUFFICIENT_BALANCE");
            err.message = Some("❌ Saldo tidak mencukupi".into());
            err.add_param("balance".into(), &self.balance);
            err.add_param("withdrawal".into(), &self.withdrawal);
            errors.add("withdrawal", err);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

// ========== FUNGSI FORMAT ERROR MANUSIA ==========
fn format_validation_errors(errors: ValidationErrors) -> String {
    let mut result = String::new();

    for (field, field_errors) in errors.field_errors() {
        for error in field_errors {
            // Ambil pesan error (custom message)
            if let Some(msg) = &error.message {
                result.push_str(&format!("  • {}: {}\n", field, msg));
            } else {
                // ✅ PERBAIKAN: Gunakan &*error.code atau error.code.clone() sebagai ganti .as_str()
                let friendly_msg = match &*error.code {
                    // <-- Perubahan di baris ini
                    "INSUFFICIENT_BALANCE" => format!(
                        "Saldo tidak cukup (Saldo: Rp {}, Penarikan: Rp {})",
                        error.params.get("balance").unwrap_or(&"?".into()),
                        error.params.get("withdrawal").unwrap_or(&"?".into())
                    ),
                    _ => format!("Validasi gagal: {}", error.code),
                };
                result.push_str(&format!("  • {}: {}\n", field, friendly_msg));
            }
        }
    }

    if result.is_empty() {
        "  ✅ Tidak ada error".to_string()
    } else {
        result
    }
}

// ========== FUNGSI PRINT YANG BAGUS ==========
fn print_validation_result(result: Result<(), ValidationErrors>, title: &str) {
    println!("\n┌─────────────────────────────────────────┐");
    println!("│ {:<39} │", title);
    println!("├─────────────────────────────────────────┤");

    match result {
        Ok(_) => {
            println!("│ ✅ VALID                                 │");
            println!("│   Data berhasil divalidasi              │");
        }
        Err(e) => {
            println!("│ ❌ INVALID                               │");
            println!("│   Error detail:                          │");
            print!("{}", format_validation_errors(e));
        }
    }
    println!("└─────────────────────────────────────────┘");
}

fn main() {
    println!("\n╔═════════════════════════════════════════════╗");
    println!("║        TRANSACTION VALIDATION SYSTEM        ║");
    println!("╚═════════════════════════════════════════════╝");

    // ========== TEST 1: Transaction 1 ==========
    let tx1 = Transaction {
        amount: 50_000,
        balance: 30_000,
        withdrawal: 50_000,
    };

    print_validation_result(tx1.validate(), "FIELD VALIDATION (Format)");

    // ========== TEST 2: Transaction 2 ==========
    let tx2 = Transaction2 {
        withdrawal: 50_000,
        balance: 30_000,
    };

    print_validation_result(tx2.validate(), "STRUCT VALIDATION (Cross-field)");

    // ========== TEST 3: Withdrawal kelipatan 10rb ==========
    let tx3 = Transaction {
        amount: 100_000,
        balance: 200_000,
        withdrawal: 53_000, // ❌ Bukan kelipatan 10.000
    };

    print_validation_result(tx3.validate(), "FIELD VALIDATION (Kelipatan)");

    // ========== TEST 4: Withdrawal 0 ==========
    let tx4 = Transaction {
        amount: 100_000,
        balance: 200_000,
        withdrawal: 0, // ❌ Tidak boleh 0
    };

    print_validation_result(tx4.validate(), "FIELD VALIDATION (Nol)");

    // ========== TEST 5: Valid semua ==========
    let tx5 = Transaction2 {
        withdrawal: 50_000,
        balance: 100_000,
    };

    print_validation_result(tx5.validate(), "VALID TRANSACTION");

    println!("\n═══════════════════════════════════════════════\n");
}
