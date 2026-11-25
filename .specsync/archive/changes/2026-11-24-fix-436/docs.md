# Docs — fix-436

- `docs/cli.md`: wherever the gate-close decision format is
  documented, the rendered decision names the human's configured
  display name (REQ-records-008), not the raw `--decided-by` text;
  the stored relay keeps what was typed.
- No new commands or flags.