# 2026-11-25: the chough-workflow skill routes every lifecycle writer through chough

REQ-cli-072: SKILL.md's "Run HI, SpecSync and Fledge yourself" line sent agents to
specsync directly, which left chough session trails unable to reach done (PR tob#38
incident). The skill now states the chough-first order (DRIVE-20), reference files
show `chough change` forms with the four unwrapped steps labeled as
direct-native-write (DRIVE-21), and a CI check keeps the bundle honest.
