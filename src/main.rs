use serde::Serialize;
use validator::Validate;

#[derive(Debug, Validate, Serialize)]
struct Product {
    #[validate(range(min = 1, message = "Product ID harus minimal 1"))]
    id: u64,

    #[validate(length(min = 1, message = "Product Name harus minimal 1"))]
    name: String,

    #[validate(length(min = 1, message = "Product harus punya minimal 1 varian"))]
    #[validate(nested)]
    variants: Vec<ProductVariant>,
}

#[derive(Debug, Validate, Serialize)]
struct ProductVariant {
    #[validate(length(min = 1, message = "Nama varian harus minimal 1 karakter"))]
    name: String,

    #[validate(range(min = 1000, message = "Harga minimal Rp 1000"))]
    price: u64,
}

fn main() {
    println!("═══════════════════════════════════");
    println!("   CONTOH 1: DATA VALID");
    println!("═══════════════════════════════════");

    let product = Product {
        id: 1,
        name: "Baju".to_string(),
        variants: vec![ProductVariant {
            name: "Merah".to_string(),
            price: 10000,
        }],
    };

    match product.validate() {
        Ok(_) => {
            println!("✅ Validasi BERHASIL!\n");
            println!("Data Product: {:?}\n", product);
        }
        Err(e) => println!("❌ Error: {}\n", e),
    }

    println!("═══════════════════════════════════");
    println!("   CONTOH 2: DATA INVALID");
    println!("═══════════════════════════════════");

    let product_invalid = Product {
        id: 1,
        name: "Baju".to_string(),
        variants: vec![ProductVariant {
            name: "".to_string(), // ❌ Nama varian kosong
            price: 10000,         // ✅ Harga valid
        }],
    };

    match product_invalid.validate() {
        Ok(_) => println!("✅ Validasi BERHASIL!"),
        Err(e) => println!("❌ Error:\n{}\n", e),
    }
}
