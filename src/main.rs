use aes_only::{decrypt_with_aes, encrypt_with_aes};
use clap::{Parser, Subcommand};
use gen_key_pair::generate_key_pair;
use rsa_hybrid::{decrypt_file, encrypt_file};
mod aes_only;
mod gen_key_pair;
mod rsa_hybrid;

#[derive(Subcommand)]
enum Commands {
    Generate {
        private_key_file: String,
        public_key_file: String,
    },

    Encrypt {
        input_file: String,
        output_file: String,
        public_key_file: String,
    },

    Decrypt {
        input_file: String,
        output_file: String,
        private_key_file: String,
    },

    EncryptAes {
        input_file: String,
        output_file: String,
        key_file: String,
    },

    DecryptAes {
        input_file: String,
        output_file: String,
        key_file: String,
    },
}

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            private_key_file,
            public_key_file,
        } => {
            if let Err(e) = generate_key_pair(&private_key_file, &public_key_file) {
                eprintln!("Error generating key pair: {e}");
            }
        }

        Commands::Encrypt {
            input_file,
            output_file,
            public_key_file,
        } => {
            if let Err(e) = encrypt_file(&input_file, &output_file, &public_key_file) {
                eprintln!("Error encrypting the file: {e}");
            }
        }

        Commands::Decrypt {
            input_file,
            output_file,
            private_key_file,
        } => {
            if let Err(e) = decrypt_file(&input_file, &output_file, &private_key_file) {
                eprintln!("Error decrypting the file: {e}");
            }
        }

        Commands::EncryptAes {
            input_file,
            output_file,
            key_file,
        } => {
            if let Err(e) = encrypt_with_aes(&input_file, &output_file, &key_file) {
                eprintln!("Error encrypting the file with AES: {e}");
            }
        }

        Commands::DecryptAes {
            input_file,
            output_file,
            key_file,
        } => {
            if let Err(e) = decrypt_with_aes(&input_file, &output_file, &key_file) {
                eprintln!("Error decrypting with AES: {e}");
            }
        }
    }
}
