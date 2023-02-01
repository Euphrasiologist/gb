# `gb`

Naive group by implementation. I'm looking to expand and understand more algorithms but the one here is a simple concurrent hash map. Should be very fast for small to large data sets. Larger data sets not tested.

Very rough draft!

## cli

```
gb 0.1.0
Max Brown <euphrasiamax@gmail.com>
https://github.com/euphrasiologist/gb

Usage:
    gb [OPTIONS] <FILE/STDIN>
    gb -f field1 -k key1,key2 -s N input.tsv
Options:
      --input <INPUT>          Path to the input file. Defaults to STDIN. [default: -]
      --keys <KEYS>...         The grouping keys as column header strings
      --field <FIELD>          The field on which to calculate grouping stats
      --delimiter <DELIMITER>  The delimiter, default is tab [default: "\t"]
      --summary <SUMMARY>      Summary stat to comupte on groups [default: N] [possible values: mean, N, sd, var]
  -h, --help                   Print help (see more with '--help')
  -V, --version                Print version
```