use serde::Deserialize;
use validator::Validate;

#[derive(Debug, Validate, Deserialize)]
struct User {
    #[validate(length(min = 3, message = "must be at least 3 characters"))]
    username: String,

    #[validate(
        length(min = 8, message = "must be at least 8 characters"),
        contains(pattern = "123", message = "must contain number 123")
    )]
    password: String,

    #[validate(email(message = "invalid email format"))]
    email: String,
}

fn main() {
    let user = User {
        username: "jo".to_string(),
        password: "abc".to_string(),
        email: "bukanemail".to_string(),
    };

    println!("🔍 Validating User: {:?}", user);
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    if let Err(e) = user.validate() {
        println!("❌ Validation Errors:");
        for (field, errors) in e.field_errors() {
            let messages: Vec<&str> = errors
                .iter()
                .filter_map(|err| err.message.as_deref())
                .collect();
            println!("   • {}: {}", field, messages.join(", "));
        }
    } else {
        println!("✅ User is valid!");
    }
}
