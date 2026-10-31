---
module: shipping
version: 1
status: active
files:
  - src/lib.rs

db_tables: []
depends_on: []
---

# Shipping

## Purpose

Compute the shipping fee for an order total. Sample module for the Chough exercise.

## Public API

| Name | Description |
|------|-------------|
| `STANDARD_FEE_CENTS` | Standard shipping fee in cents |
| `shipping_fee` | Shipping fee in cents for an order total in cents |

## Invariants

1. Fees are whole cents and never negative.

## Behavioral Examples

```gherkin
Given an order total in cents
When the shipping fee is computed
Then the standard fee applies unless a rule exempts the order
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| None | Every total is valid | A fee is always returned |

## Dependencies

None.

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-10-31 | Sample baseline |
