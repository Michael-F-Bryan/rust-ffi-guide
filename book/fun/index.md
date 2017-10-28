# Break All The Things!!1!

What would a guide on `unsafe` Rust be without an exploration into how (not) to
abuse it? This section takes a bit of a detour from the rest of the document in
that we'll explicitly be *trying* to break things in as many ways as possible.

> **Warning:** This section may not be for the faint hearted.

The exercise will be as follows, each problem will contain the source code
for a small program which deliberately does something horribly wrong, incorrect,
or dangerous (memory safety, data races, undefined behaviour, that sort of
thing). It's then your job to figure out what the issue is and why it could end
up hurting your application. 