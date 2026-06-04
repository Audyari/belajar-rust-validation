use serde::Serialize;
use validator::Validate;

#[derive(Debug, Validate, Serialize)]
struct Alamat {
    #[validate(length(min = 1, message = "Jalan tidak boleh kosong"))]
    jalan: String,

    #[validate(length(min = 1, message = "Kota tidak boleh kosong"))]
    kota: String,

    #[validate(range(
        min = 10000,
        max = 99999,
        message = "Kode pos harus antara 10000-99999"
    ))]
    kode_pos: u32,
}

#[derive(Debug, Validate, Serialize)]
struct Mahasiswa {
    #[validate(length(min = 3, message = "Nama minimal 3 karakter"))]
    nama: String,

    #[validate(range(min = 17, max = 50, message = "Umur harus antara 17-50 tahun"))]
    umur: u8,

    #[validate(nested)]
    alamat: Alamat,
}

fn proses_mahasiswa(mahasiswa: &Mahasiswa, judul: &str) {
    println!("═══════════════════════════════════");
    println!("  {judul}");
    println!("═══════════════════════════════════");

    if let Err(e) = mahasiswa.validate() {
        println!("❌ Error: {e}\n");
        return; // ← RETURN hanya keluar dari fungsi, bukan dari main!
    }

    let json_pretty = serde_json::to_string_pretty(&mahasiswa).unwrap();
    println!("\nJSON Pretty:\n{json_pretty}\n");
}

fn main() {
    // Contoh 1
    let alamat_valid = Alamat {
        jalan: String::from("Jl. Merdeka No. 123"),
        kota: String::from("Jakarta"),
        kode_pos: 12345,
    };

    let mahasiswa_valid = Mahasiswa {
        nama: String::from("Budi Santoso"),
        umur: 20,
        alamat: alamat_valid,
    };

    proses_mahasiswa(&mahasiswa_valid, "CONTOH 1: DATA VALID");

    // Contoh 2 (TETAP EKSEKUSI!)
    let alamat_invalid = Alamat {
        jalan: String::from("Jl. Sudirman"),
        kota: String::new(),
        kode_pos: 9999,
    };

    let mahasiswa_invalid = Mahasiswa {
        nama: String::from("Ani"),
        umur: 16,
        alamat: alamat_invalid,
    };

    proses_mahasiswa(&mahasiswa_invalid, "CONTOH 2: DATA INVALID");
}
