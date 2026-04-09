# Deterministic Palette Generation Checklist

## Implementation Pitfalls Reference

### 1. **DefaultHasher Non-Determinism (YOUR RISK)**
**Status**: 🔴 HIGH PRIORITY in your code

**Problem**: `std::hash::DefaultHasher` is NOT deterministic across process runs.
- Uses randomized SipHash seed for DOS protection
- Seed changes each program invocation
- Breaking your cache mechanism

**References**:
- [Rust users.rust-lang.org — Hash stability (2025-07-26)](https://users.rust-lang.org/t/are-hash-and-hasher-outputs-guaranteed-to-be-stable-between-rust-versions/132304)
- [deterministic_default_hasher crate docs](https://docs.rs/deterministic_default_hasher/) — Proves stdlib doesn't guarantee determinism
- Your code: `src/palette/generator.rs:46-49` (palette_cache_key)

**Fix**: Replace with SHA256 or Blake3 for cache keys.

---

### 2. **Bytes-Only Hashing vs. Full Input**
**Status**: ✅ GOOD in your code (but incomplete)

**Issue**: Your `palette_cache_key` hashes bytes + settings, which is correct, but:
- If you add parameters (FUTURE: color count, algorithm version), cache keys won't invalidate
- Schema migrations will cause silent cache misses or wrong palettes

**References**:
- Content-addressed storage best practice: [nixpkgs derivation hashing](https://nixos.org/manual/nix/unstable/architecture/derivations.html)

**Fix**: Version the cache key format: `v1:sha256:{hash}` or store version in metadata.

---

### 3. **Hash Stability Across Rust Versions**
**Status**: ⚠️ MODERATE RISK

**Issue**: Even cryptographic hashes can change if:
- Digest crate updates its algorithm
- You switch from SHA256 to Blake3
- CPU architecture changes (rare, but possible with hex encoding)

**Fix**:
```rust
// Store version in cache file
#[derive(Serialize, Deserialize)]
pub struct CachedPalette {
    pub version: u32,  // Increment on algorithm changes
    pub algorithm: String,  // "sha256" or "blake3"
    pub palette: GeneratedPalette,
}
```

---

### 4. **Wallpaper File Path in Cache Key**
**Status**: ✅ CORRECT (you hash bytes, not path)

**Why good**: 
- Same image at different paths generates same cache key ✓
- Moving wallpapers doesn't break cache ✓

**Why NOT path-based**:
- Path changes → new cache key even if image identical
- Symbolic links → multiple cache entries

---

### 5. **Float Precision in Color Derivation**
**Status**: ✅ SAFE in your code

**Your approach**:
```rust
fn derive_palette(bytes: &/home/linuxbrew/.linuxbrew/bin/snip [u8]) -> Vec<String> {
    for index in 0..16 {
        let byte = bytes.get(index).copied().unwrap_or(0);
        /home/linuxbrew/.linuxbrew/bin/snip let r = byte;
        /home/linuxbrew/.linuxbrew/bin/snip let g = byte.rotate_left(2);
        /home/linuxbrew/.linuxbrew/bin/snip let b = byte.rotate_left(4);
        /home/linuxbrew/.linuxbrew/bin/snip colors.push(format!("#{r:02X}{g:02X}{b:02X}"));
    /home/linuxbrew/.linuxbrew/bin/snip }
}
```

**Good**: Integer-only operations → deterministic.
**Bad example**: Using floating-point color space conversions (RGB → HSL → RGB) — rounding errors compound.

---

### 6. **Serialization Stability**
**Status**: ⚠️ VERIFY

**Issue**: `serde_json::to_string_pretty()` in `write_palette_cache` is deterministic for the same data, BUT:
- JSON key ordering in maps is NOT guaranteed
- If `GeneratedPalette` gains additional fields, serialization might reorder

**Fix** (if needed):
```rust
// Use serde with sorted_keys feature or explicit ordering
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct GeneratedPalette {
    #[serde(serialize_with = "serialize_colors")]
    pub colors: Vec<String>,
    pub wallpaper_hash: String,  // Alphabetical order
}
```

---

### 7. **Cache Invalidation Strategy**
**Status**: ❌ MISSING

**Gap**: You can write cache, but how do you detect stale cache?

**Scenarios**:
1. User edits wallpaper in-place → cache key same, but content changed
2. Palette algorithm improves → old cached palettes are "wrong"
3. Settings format changes → cache key generation changes

**Mitigation**:
```rust
// Option A: Hash wallpaper content, not path
// (You already do this ✓)

// Option B: Store wallpaper mtime in cache
#[derive(Serialize)]
pub struct CachedPalette {
    pub wallpaper_hash: String,
    pub wallpaper_mtime: SystemTime,  // Detect edits
    pub palette: GeneratedPalette,
}

// Option C: Version in cache format
pub const CACHE_VERSION: u32 = 1;

/home/linuxbrew/.linuxbrew/bin/snip // Option D: Validate on read
if should_validate_cache() {
    let regenerated = generate_palette_from_wallpaper(path)?;
    /home/linuxbrew/.linuxbrew/bin/snip assert_eq!(cached.palette, regenerated);
/home/linuxbrew/.linuxbrew/bin/snip }
```

---

## 📋 Pre-Implementation Checklist

- [ ] **Replace `DefaultHasher` with SHA256 in `palette_cache_key()`**
  - Add `sha2 = "0.10"` to Cargo.toml
  - Update `palette_cache_key()` function body
  
- [ ] **Add version prefix to cache keys**
  - Format: `v1:{sha256_hash}`
  - Allows future migrations
  
- [ ] **Test determinism across separate processes**
  - Run program twice with same image
  - Assert both generate identical cache keys
  - Test: `tests/palette_determinism.rs` – add `test_cache_key_consistent_across_runs()`
  
- [ ] **Document hashing algorithm**
  - Add comment: "SHA256 ensures determinism across Rust versions and process runs"
  - Reference: [Rust hash stability forum](https://users.rust-lang.org/t/are-hash-and-hasher-outputs-guaranteed-to-be-stable-between-rust-versions/132304)
  
- [ ] **Add cache invalidation strategy** (optional for MVP, REQUIRED for production)
  - Consider wallpaper mtime or regeneration validation
  - Document in `palette_cache_key()` docstring
  
- [ ] **Verify serialization stability**
  - Ensure `serde_json::to_string_pretty()` always produces same JSON for same data
  - Test: `tests/palette_determinism.rs` – add `test_serialization_determinism()`
  
- [ ] **Add integration test across runs**
  ```rust
  #[test]
  fn cache_key_deterministic_across_invocations() {
      let path = Path::new("tests/fixtures/wallpapers/sample-a.png");
      /home/linuxbrew/.linuxbrew/bin/snip let key1 = palette_cache_key(path, "mode=dark").unwrap();
      /home/linuxbrew/.linuxbrew/bin/snip let key2 = palette_cache_key(path, "mode=dark").unwrap();
      /home/linuxbrew/.linuxbrew/bin/snip assert_eq!(key1, key2);  /home/linuxbrew/.linuxbrew/bin/snip // Will FAIL with DefaultHasher across processes
  }
  ```

---

## 🔍 Code Review Checklist for `generator.rs`

| /home/linuxbrew/.linuxbrew/bin/snip Line | /home/linuxbrew/.linuxbrew/bin/snip Issue | /home/linuxbrew/.linuxbrew/bin/snip Severity | /home/linuxbrew/.linuxbrew/bin/snip Status |
|/home/linuxbrew/.linuxbrew/bin/snip ------|/home/linuxbrew/.linuxbrew/bin/snip -------|/home/linuxbrew/.linuxbrew/bin/snip ----------|/home/linuxbrew/.linuxbrew/bin/snip --------|
| /home/linuxbrew/.linuxbrew/bin/snip 2 | /home/linuxbrew/.linuxbrew/bin/snip `DefaultHasher` used | /home/linuxbrew/.linuxbrew/bin/snip 🔴 HIGH | /home/linuxbrew/.linuxbrew/bin/snip Fix required |
| /home/linuxbrew/.linuxbrew/bin/snip 26, 100–103 | /home/linuxbrew/.linuxbrew/bin/snip `stable_hash_bytes()` for internal `wallpaper_hash` | /home/linuxbrew/.linuxbrew/bin/snip ✅ OK | /home/linuxbrew/.linuxbrew/bin/snip Keep |
| /home/linuxbrew/.linuxbrew/bin/snip 35–50 | /home/linuxbrew/.linuxbrew/bin/snip `palette_cache_key()` logic | /home/linuxbrew/.linuxbrew/bin/snip 🔴 HIGH | /home/linuxbrew/.linuxbrew/bin/snip Replace hasher |
| /home/linuxbrew/.linuxbrew/bin/snip 52–86 | /home/linuxbrew/.linuxbrew/bin/snip `write_palette_cache()` I/O | /home/linuxbrew/.linuxbrew/bin/snip ✅ OK | /home/linuxbrew/.linuxbrew/bin/snip Good error handling |
| /home/linuxbrew/.linuxbrew/bin/snip 88–98 | /home/linuxbrew/.linuxbrew/bin/snip `derive_palette()` integer math | /home/linuxbrew/.linuxbrew/bin/snip ✅ OK | /home/linuxbrew/.linuxbrew/bin/snip Deterministic |

/home/linuxbrew/.linuxbrew/bin/snip ---

## 🎯 Downstream Guidance

**For `generator.rs` update**:
1. Replace `use std::hash::*` with `use sha2::{Sha256, Digest}`
2. Rewrite `palette_cache_key()` function (8 lines → 12 lines)
3. Add version prefix: `format!("v1:{:x}", finalize())`
4. No other functions need changes

**For tests**:
1. Add `test_cache_key_same_image_produces_same_key()` (multirun test)
2. Add `test_cache_key_different_settings_produces_different_key()` (already exists as `test_palette_cache_key_changes_when_inputs_change`)
3. Verify SHA256 dependency in test environment

**For Cargo.toml**:
```toml
[dependencies]
sha2 = "0.10"
```

---

## References & /home/linuxbrew/.linuxbrew/bin/snip External Links

1. **Rust DefaultHasher NOT deterministic**: https://users.rust-lang.org/t/are-hash-and-hasher-outputs-guaranteed-to-be-stable-between-rust-versions/132304
2. **deterministic_default_hasher crate** (why it exists): https://docs.rs/deterministic_default_hasher/
3. **SHA2 crate docs**: https://docs.rs/sha2/
4. **Content-addressed storage principles**: https://nixos.org/manual/nix/unstable/architecture/derivations.html
5. **Your test**: `tests/palette_determinism.rs:10–18`
6. **Your generator**: `src/palette/generator.rs:35–50`

