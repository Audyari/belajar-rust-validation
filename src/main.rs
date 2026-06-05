use validator::{ValidationError, ValidationErrors};

// 1. DEFINE CONTEXT
#[derive(Debug)]
pub struct MyContext {
    pub min_length: usize,
    pub max_length: usize,
}

// 2. CUSTOM VALIDATOR (tanpa macro arg!)
fn validate_with_context(value: &str, ctx: &MyContext) -> Result<(), ValidationError> {
    if value.len() < ctx.min_length {
        let mut err = ValidationError::new("too_short");
        err.add_param("min".into(), &ctx.min_length);
        err.add_param("actual".into(), &value.len());
        return Err(err);
    }

    if value.len() > ctx.max_length {
        let mut err = ValidationError::new("too_long");
        err.add_param("max".into(), &ctx.max_length);
        return Err(err);
    }

    Ok(())
}

// 3. STRUCT BIASA (tanpa #[derive(Validate)])
#[derive(Debug)]
struct User {
    username: String,
}

// 4. MANUAL IMPLEMENT ValidateArgs (ini yang stabil!)
impl<'a> User {
    fn validate_with_args(&self, ctx: &'a MyContext) -> Result<(), ValidationErrors> {
        let mut errors = ValidationErrors::new();

        // Panggil custom validator dengan context!
        if let Err(e) = validate_with_context(&self.username, ctx) {
            errors.add("username", e);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }
}

fn main() {
    println!("═══════════════════════════════════");
    println!("   CONTEXT VALIDATION (MANUAL)");
    println!("═══════════════════════════════════");

    // Buat context
    let ctx = MyContext {
        min_length: 3,
        max_length: 10,
    };

    // Test 1: Username terlalu pendek
    let user1 = User {
        username: "jo".to_string(),
    };

    println!("\n🔵 Test 1: username 'jo'");
    match user1.validate_with_args(&ctx) {
        Ok(_) => println!("✅ Valid"),
        Err(e) => {
            println!("❌ Error:");
            for (field, errors) in e.field_errors() {
                for err in errors {
                    println!("   - {}: {}", field, err.code);
                    println!("     Params: {:?}", err.params);
                }
            }
        }
    }

    // Test 2: Username valid
    let user2 = User {
        username: "john".to_string(),
    };

    println!("\n🔵 Test 2: username 'john'");
    match user2.validate_with_args(&ctx) {
        Ok(_) => println!("✅ Valid!"),
        Err(e) => println!("❌ Error: {:?}", e),
    }
}
