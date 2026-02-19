<img src="https://github.com/jsonh-org/Jsonh/blob/main/IconUpscaled.png?raw=true" width=180>

[![crates.io](https://img.shields.io/crates/v/jsonh_rs.svg)](https://crates.io/crates/jsonh_rs)

**JSON for Humans.**

JSON is great. Until you miss that trailing comma... or want to use comments. What about multiline strings?
JSONH provides a much more elegant way to write JSON that's designed for humans rather than machines.

Since JSONH is compatible with JSON, any JSONH syntax can be represented with equivalent JSON.

## JsonhRs

JsonhRs is a parser implementation of [JSONH V1 & V2](https://github.com/jsonh-org/Jsonh) for C# .NET.

## Example

```jsonh
{
    // use #, // or /**/ comments
    
    // quotes are optional
    keys: without quotes,

    // commas are optional
    isn\'t: {
        that: cool? # yes
    }

    // use multiline strings
    haiku: '''
        Let me die in spring
          beneath the cherry blossoms
            while the moon is full.
        '''
    
    // compatible with JSON5
    key: 0xDEADCAFE

    // or use JSON
    "old school": 1337
}
```

## Usage

Everything you need is contained within `JsonhReader`:

```rs
use jsonh_rs::*;

let jsonh: &str = r#"
{
    this is: awesome
}
"#;
let element: Value = JsonhReader::parse_element_from_str(jsonh, JsonhReaderOptions::new()).unwrap();
```

## Limitations

In comparison to [JsonhCs](https://github.com/jsonh-org/JsonhCs), this Rust implementation has some limitations.

### No 'infinity' fallbacks

While JSON doesn't have infinity or not-a-number, `1e99999` is sometimes used to represent infinity.

This is correctly parsed, but it cannot be converted to a `serde_json::Value` due to a limitation with serde_json.