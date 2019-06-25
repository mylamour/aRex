Rules was extracted from [Yara-Rules](https://github.com/Yara-Rules/rules)

Then use a fast implementation of Aho-Corasick in Rust was named [aho-corasick](https://github.com/BurntSushi/aho-corasick)

So you can just run it with `./target/debug/aRex -r rule.list -t ./target.detect`

* you should use whitelist to check your rules
* you should use blacklist to check your rules