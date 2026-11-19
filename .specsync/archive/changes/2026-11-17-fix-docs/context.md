# Context

Fable reviewed HEAD 0fd9fd22 and found twelve documentation sentences that no longer match the helper, plus a fast path that told a small model to answer with raw SpecSync and to commit after `chough change check`. FLOW-18 already requires the docs, skill and diagrams to say what current main does. This change corrects those sentences and puts the same short rules in AGENTS.md and the five host templates. It does not change requirement text.
