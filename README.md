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
    gb -f field1 -k key1,key2 -s N -i input.tsv
    cat input.csv | gb -f field1 -k key1 -s mean -d,
Options:
  -i, --input <INPUT>
          Path to the input file. Defaults to STDIN.
          
          [default: -]

  -k, --keys <KEYS>...
          The grouping keys as column header strings

  -f, --field <FIELD>
          The field on which to calculate grouping stats

  -d, --delimiter <DELIMITER>
          The delimiter, default is tab
          
          [default: "\t"]

  -s, --summary <SUMMARY>
          Summary stat to comupte on groups
          
          [default: N]

          Possible values:
          - mean: Calculate mean on groups
          - N:    Calculate number in each group
          - sd:   Calculate standard deviation on each group
          - var:  Calculate variance on each group

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```