//! Shared normalized slugs and random-suffix record identities, and the date and slug
//! names of 0.6.0 records with their clash suffix (REQ-records-003).

use anyhow::{Result, bail, ensure};
use std::path::Path;

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

// Plan step 3 calls these; until then only their tests do.
#[cfg_attr(not(test), allow(dead_code))]
/// The longest a record name may be, a clash suffix included (REQ-records-003).
pub(crate) const NAME_LIMIT: usize = 80;
/// `-` and six hexadecimal digits.
#[cfg_attr(not(test), allow(dead_code))]
const CLASH_SUFFIX: usize = 7;

// Plan step 3 calls these; until then only their tests do.
#[cfg_attr(not(test), allow(dead_code))]
/// `YYYY-MM-DD` for a Unix time, in UTC.
pub(crate) fn utc_date(seconds: u64) -> String {
    // Days to civil date (H. Hinnant), for the proleptic Gregorian calendar.
    let days = i64::try_from(seconds / 86_400).unwrap_or(i64::MAX / 2) + 719_468;
    let era = days.div_euclid(146_097);
    let day_of_era = days.rem_euclid(146_097);
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = if shifted_month < 10 {
        shifted_month + 3
    } else {
        shifted_month - 9
    };
    let year = year_of_era + era * 400 + i64::from(month <= 2);
    format!("{year:04}-{month:02}-{day:02}")
}

// Plan step 3 calls these; until then only their tests do.
#[cfg_attr(not(test), allow(dead_code))]
/// `<date>-<slug>`: the slug of `source`, or `kind` when nothing is left, cut back at a
/// hyphen so that the name with a clash suffix stays within [`NAME_LIMIT`]. It holds no
/// number taken from a count of other records.
pub(crate) fn dated_name(date: &str, source: &str, kind: &str) -> String {
    let limit = NAME_LIMIT - CLASH_SUFFIX - date.len() - 1;
    let full = slug(source, kind, usize::MAX);
    let cut = if full.len() <= limit {
        full.as_str()
    } else if full.as_bytes()[limit] == b'-' {
        &full[..limit]
    } else {
        let prefix = &full[..limit];
        prefix.rfind('-').map_or(prefix, |at| &prefix[..at])
    };
    format!("{date}-{cut}")
}

// Plan step 3 calls these; until then only their tests do.
#[cfg_attr(not(test), allow(dead_code))]
/// Six lowercase hexadecimal digits from 24 fresh random bits.
fn clash_suffix() -> Result<String> {
    let bytes = fill_random()?;
    Ok(suffix(&bytes)[..6].to_string())
}

// Plan step 3 calls these; until then only their tests do.
#[cfg_attr(not(test), allow(dead_code))]
/// `<name>-<6 hex>`, from one fresh draw of 24 random bits.
pub(crate) fn suffixed(name: &str) -> Result<String> {
    Ok(format!("{name}-{}", clash_suffix()?))
}

/// Create `<dir>/<name><extension>` by create-new with the content `render(name)`. When
/// that file exists, add one clash suffix and try once more; when that name exists too,
/// refuse with nothing written and draw no other suffix. Returns the name written.
pub(crate) fn create_named(
    root: &Path,
    dir: &str,
    name: &str,
    extension: &str,
    bound: u64,
    render: impl Fn(&str) -> Result<String>,
) -> Result<String> {
    let first = format!("{dir}/{name}{extension}");
    if crate::records::write_new_bounded(root, &first, &render(name)?, bound)? {
        return Ok(name.to_string());
    }
    let suffixed = format!("{name}-{}", clash_suffix()?);
    let second = format!("{dir}/{suffixed}{extension}");
    if crate::records::write_new_bounded(root, &second, &render(&suffixed)?, bound)? {
        return Ok(suffixed);
    }
    bail!("refused: {second} also exists; nothing was written and no other name was drawn")
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
    fn utc_dates_follow_the_calendar() {
        for (seconds, date) in [
            (0, "1970-01-01"),
            (951_782_400, "2000-02-29"),
            (1_709_164_800, "2024-02-29"),
            (1_735_689_599, "2024-12-31"),
            (1_735_689_600, "2025-01-01"),
        ] {
            assert_eq!(utc_date(seconds), date, "{seconds}");
        }
    }

    #[test]
    fn dated_names_are_bounded_slugs_without_counters() {
        assert_eq!(
            dated_name("2026-11-10", "  Fix   LOGIN... now!  ", "bug"),
            "2026-11-10-fix-login-now"
        );
        assert_eq!(dated_name("2026-11-10", "!!! ...", "bug"), "2026-11-10-bug");
        let long = dated_name("2026-11-10", &"word ".repeat(40), "bug");
        assert!(long.len() + CLASH_SUFFIX <= NAME_LIMIT, "{long}");
        assert!(!long.ends_with('-'), "{long}");
        assert!(long.starts_with("2026-11-10-word-word"), "{long}");
        let unbroken = dated_name("2026-11-10", &"a".repeat(200), "bug");
        assert_eq!(unbroken.len() + CLASH_SUFFIX, NAME_LIMIT, "{unbroken}");
    }

    #[test]
    fn a_clash_gets_one_suffix_and_a_second_clash_is_refused_without_another_draw() {
        let root = tempfile::tempdir().unwrap();
        let render = |name: &str| Ok(format!("---\nid: {name}\n---\n"));
        let first = create_named(root.path(), "b", "2026-11-10-x", ".md", 1_000, render).unwrap();
        assert_eq!(first, "2026-11-10-x");
        let second = with_random_results(vec![Ok([0xab; 12])], || {
            create_named(root.path(), "b", "2026-11-10-x", ".md", 1_000, render).unwrap()
        });
        assert_eq!(second, "2026-11-10-x-ababab");
        assert_eq!(
            std::fs::read_to_string(root.path().join("b/2026-11-10-x-ababab.md")).unwrap(),
            "---\nid: 2026-11-10-x-ababab\n---\n"
        );
        let (error, draws) = with_random_results(vec![Ok([0xab; 12]), Ok([0xcd; 12])], || {
            let error = create_named(root.path(), "b", "2026-11-10-x", ".md", 1_000, render)
                .unwrap_err()
                .to_string();
            (error, random_draws())
        });
        assert!(error.contains("also exists"), "{error}");
        assert_eq!(draws, 1);
        assert_eq!(std::fs::read_dir(root.path().join("b")).unwrap().count(), 2);
    }

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
