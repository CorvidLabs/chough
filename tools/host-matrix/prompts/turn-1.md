The shop owner asks: "Orders of fifty dollars or more should ship free, and everyone
else pays the normal fee." The criteria are already in hi/ (CHECKOUT-1, CHECKOUT-2).

Start this work with Chough:
`chough init --objective "Free shipping from fifty dollars" --intent-scope CHECKOUT --module shipping {{INIT_ARGS}}`.
Then take it to the intent approval gate and stop.
