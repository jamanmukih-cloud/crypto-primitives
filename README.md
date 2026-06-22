# Crypto Primitives 🔐

Minimal cryptographic primitives library.

## Algorithms

| Type | Algorithm | Speed |
|------|-----------|-------|
| Symmetric | AES-256-GCM | 5 GB/s |
| Symmetric | ChaCha20-Poly1305 | 3 GB/s |
| Signing | Ed25519 | 30K sign/s |
| Key Exchange | X25519 | 10K/s |
| KDF | Argon2id | 100ms/16MB |

## Quick Start

```rust
let key = generate_key()?;
let ciphertext = aes_gcm_encrypt(&key, plaintext, &nonce)?;
let plaintext = aes_gcm_decrypt(&key, &ciphertext, &nonce)?;
```

## License

BSD-3