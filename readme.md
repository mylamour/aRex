Rules was extracted from [Yara-Rules](https://github.com/Yara-Rules/rules)

Then use a fast implementation of Aho-Corasick in Rust was named [aho-corasick](https://github.com/BurntSushi/aho-corasick)

So you can just run it with `./target/debug/aRex -r rule.list -t ./target.detect`

* you should use whitelist to check your rules
* you should use blacklist to check your rules

![image](https://user-images.githubusercontent.com/12653147/60076961-6f802d00-975b-11e9-942f-7ffbcf09e182.png)
