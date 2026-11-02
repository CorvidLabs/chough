//! Shared normalized slugs and random-suffix record identities.

use anyhow::{Result, ensure};

pub(crate) const OBSERVATION_SLUG_LIMIT: usize = 71;
pub(crate) const VALIDATION_SLUG_LIMIT: usize = 71;
pub(crate) const SNAPSHOT_SLUG_LIMIT: usize = 70;

#[cfg(test)]
use std::cell::{Cell, RefCell};

#[cfg(test)]
thread_local! {
    static RANDOM_RESULTS: RefCell<Vec<std::result::Result<[u8; 12], String>>> = const { RefCell::new(Vec::new()) };
    static RANDOM_DRAWS: Cell<usize> = const { Cell::new(0) };
}

pub(crate) fn slug(source: &str, fallback: &str, limit: usize) -> String {
    let mut normalized = String::new();
    let mut separated = false;
    for byte in source.bytes() {
        let byte = byte.to_ascii_lowercase();
        if byte.is_ascii_lowercase() || byte.is_ascii_digit() {
            if separated && !normalized.is_empty() {
                normalized.push('-');
            }
            separated = false;
            normalized.push(char::from(byte));
        } else {
            separated = true;
        }
    }
    if normalized.is_empty() {
        normalized.push_str(fallback);
    }
    normalized.truncate(limit);
    while normalized.ends_with('-') {
        normalized.pop();
    }
    normalized
}

fn valid_slug(value: &str, limit: usize) -> bool {
    !value.is_empty()
        && value.len() <= limit
        && !value.starts_with('-')
        && !value.ends_with('-')
        && !value.contains("--")
        && value
            .bytes()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-')
}

pub(crate) fn valid_case_id(module: &str, id: &str) -> bool {
    let Some(title) = id.strip_prefix(&format!("TC-{module}-")) else {
        return false;
    };
    if module.len() > 95 || id.len() > 100 {
        return false;
    }
    (title.len() == 4 && title.bytes().all(|byte| byte.is_ascii_digit()))
        || valid_slug(title, 96 - module.len())
}

fn valid_random_id(prefix: &str, id: &str, slug_limit: usize) -> bool {
    let Some(rest) = id.strip_prefix(prefix) else {
        return false;
    };
    if id.len() > 100 {
        return false;
    }
    let Some((slug, suffix)) = rest.rsplit_once('-') else {
        return false;
    };
    valid_slug(slug, slug_limit)
        && suffix.len() == 24
        && suffix
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(crate) fn valid_observation_id(id: &str) -> bool {
    valid_random_id("OBS-", id, OBSERVATION_SLUG_LIMIT)
}

pub(crate) fn valid_validation_id(id: &str) -> bool {
    valid_random_id("VAL-", id, VALIDATION_SLUG_LIMIT)
}

pub(crate) fn valid_snapshot_id(id: &str) -> bool {
    valid_random_id("SNAP-", id, SNAPSHOT_SLUG_LIMIT)
}

fn fill_random() -> Result<[u8; 12]> {
    #[cfg(test)]
    {
        let injected = RANDOM_RESULTS.with(|results| {
            if results.borrow().is_empty() {
                None
            } else {
                RANDOM_DRAWS.with(|draws| draws.set(draws.get() + 1));
                Some(results.borrow_mut().remove(0))
            }
        });
        if let Some(result) = injected {
            return result.map_err(anyhow::Error::msg);
        }
    }
    let mut bytes = [0_u8; 12];
    getrandom::fill(&mut bytes)?;
    Ok(bytes)
}

fn suffix(bytes: &[u8; 12]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut encoded = String::with_capacity(24);
    for byte in bytes {
        encoded.push(char::from(HEX[usize::from(byte >> 4)]));
        encoded.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    encoded
}

fn allocate(prefix: &str, source: &str, fallback: &str, slug_limit: usize) -> Result<String> {
    let bytes = fill_random()?;
    Ok(format!(
        "{prefix}{}-{suffix}",
        slug(source, fallback, slug_limit),
        suffix = suffix(&bytes)
    ))
}

pub(crate) fn observation(source: &str) -> Result<String> {
    allocate("OBS-", source, "observation", OBSERVATION_SLUG_LIMIT)
}

pub(crate) fn validation(source: &str) -> Result<String> {
    allocate("VAL-", source, "validation", VALIDATION_SLUG_LIMIT)
}

pub(crate) fn snapshot(source: &str) -> Result<String> {
    allocate("SNAP-", source, "snapshot", SNAPSHOT_SLUG_LIMIT)
}

pub(crate) fn require_explicit_observation_id(id: &str) -> Result<()> {
    ensure!(
        valid_observation_id(id),
        "old-style observation ID {id:?} cannot be used for a new write; omit the ID for automatic allocation"
    );
    Ok(())
}

#[cfg(test)]
pub(crate) fn with_random_results<T>(
    results: Vec<std::result::Result<[u8; 12], &'static str>>,
    run: impl FnOnce() -> T,
) -> T {
    RANDOM_RESULTS.with(|slot| {
        *slot.borrow_mut() = results
            .into_iter()
            .map(|result| result.map_err(str::to_string))
            .collect();
    });
    RANDOM_DRAWS.with(|draws| draws.set(0));
    let result = run();
    RANDOM_RESULTS.with(|slot| slot.borrow_mut().clear());
    result
}

#[cfg(test)]
pub(crate) fn random_draws() -> usize {
    RANDOM_DRAWS.with(Cell::get)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalization_is_ascii_bounded_and_nonempty() {
        for (source, expected) in [
            ("  Fix   LOGIN... now!  ", "fix-login-now"),
            ("already--hyphenated", "already-hyphenated"),
            ("ASCII_ separators!!!", "ascii-separators"),
            ("猫", "fallback"),
            ("!!!", "fallback"),
            ("", "fallback"),
        ] {
            assert_eq!(slug(source, "fallback", 71), expected, "{source:?}");
        }
        assert_eq!(
            slug(&format!("{}-b", "a".repeat(70)), "bug", 71),
            "a".repeat(70)
        );
    }

    #[test]
    fn twelve_random_bytes_encode_as_twenty_four_lowercase_hex_digits() {
        for (bytes, expected) in [
            ([0; 12], "000000000000000000000000"),
            ([255; 12], "ffffffffffffffffffffffff"),
            (
                [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
                "000102030405060708090a0b",
            ),
        ] {
            assert_eq!(suffix(&bytes), expected);
        }
        let id = with_random_results(vec![Ok([0xab; 12])], || observation("TC-cli-0001").unwrap());
        assert_eq!(id, "OBS-tc-cli-0001-abababababababababababab");
        assert_eq!(random_draws(), 1);
    }

    #[test]
    fn random_source_failure_is_returned_after_one_draw_without_a_fallback() {
        let error = with_random_results(vec![Err("injected random failure")], || {
            observation("TC-cli-0001").unwrap_err()
        });
        assert!(error.to_string().contains("injected random failure"));
        assert_eq!(random_draws(), 1);
    }

    #[test]
    fn exact_validators_keep_suffixes_and_legacy_case_ids() {
        for (prefix, limit, valid) in [
            (
                "OBS-",
                OBSERVATION_SLUG_LIMIT,
                valid_observation_id as fn(&str) -> bool,
            ),
            (
                "VAL-",
                VALIDATION_SLUG_LIMIT,
                valid_validation_id as fn(&str) -> bool,
            ),
            (
                "SNAP-",
                SNAPSHOT_SLUG_LIMIT,
                valid_snapshot_id as fn(&str) -> bool,
            ),
        ] {
            let id = format!("{prefix}{}-{}", "a".repeat(limit), "0".repeat(24));
            assert_eq!(id.len(), 100);
            assert!(valid(&id), "{id}");
            assert!(!valid(&format!(
                "{prefix}{}-{}",
                "a".repeat(limit + 1),
                "0".repeat(24)
            )));
            assert!(!valid(&id[..id.len() - 1]));
            assert!(!valid(&format!("{id}0")));
            assert!(!valid(&id.replace("a-", "A-")));
            assert!(!valid(&format!("{prefix}-{}", "0".repeat(24))));
            assert!(!valid(&format!("{prefix}slug-{}", "A".repeat(24))));
        }
        assert!(valid_case_id("cli", "TC-cli-0001"));
        assert!(valid_case_id("cli", &format!("TC-cli-{}", "a".repeat(93))));
        assert!(!valid_case_id("cli", &format!("TC-cli-{}", "a".repeat(94))));
        assert!(!valid_case_id("other", "TC-cli-0001"));
    }
}
