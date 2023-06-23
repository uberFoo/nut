# Nut: a hard one to crack, also the meat of the matter

The idea is to generate code from models.
To do this we need a starting point.
`ooa_0` is that starting point.
We iterate on creating new models from old ones until the model is expressive enough to generate code.

Eventually I would like to target embedded platforms.

## The Present

This is where it all started.
I initially created the definitions found in `ooa_0`.
You know, `Object` and friends.
Next I created instances, by hand (in code), of them in `gen_schema.rs`.
I made a few and got bored.
So I started working on [Cuckoo](https://git.uberfoo.com/uberfoo/cuckoo), using the instances above as input.
Cuckoo was eventually able to create instances and output JSON.
Finally, I created `code_gen.rs` to read this JSON and output some code.

## License

nut is distributed under the terms of both the MIT license and the Apache License (Version 2.0).

See [LICENSE-APACHE](LICENSE-APACHE), [LICENSE-MIT](LICENSE-MIT) for details.