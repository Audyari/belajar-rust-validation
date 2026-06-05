use validator::{Validate, ValidationError, ValidationErrors};

#[derive(Debug)]
struct Register {
    password: String,
    confirm_password: String,
}

impl Validate for Register {
    fn validate(&self) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // ✅ CUSTOM VALIDATION untuk membandingkan 2 field
        if self.password != self.confirm_password {
            let mut err = ValidationError::new("password_mismatch");
            err.add_param(
                "message".into(),
                &"Password dan confirm password harus sama",
            );
            errors.add("confirm_password", err);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn main() {
    let valid = Register {
        password: "secret123".to_string(),
        confirm_password: "secret123".to_string(),
    };

    let invalid = Register {
        password: "secret123".to_string(),
        confirm_password: "wrong456".to_string(),
    };

    println!("Valid register: {:?}", valid.validate().is_ok());
    println!("Invalid register: {:?}", invalid.validate().is_ok());
}
