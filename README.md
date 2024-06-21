# bgtools

bgtools is a set of utilities for performing commonly-needed operations on
bedgraph files. It is not intended as a replacement for bedtools, and it does not
re-implement functionality that bedtools already offers for bedgraph data.

Currently, bgtools offers the following sub-commands:

```bash
# For running a rolling mean or median
bgtools roll
# For getting genome-wide robust z-scores
bgtools robust_z
# For getting counts per million
bgtools cpm
# For writing contiguous regions of a bedgraph file to stdout in bed format
bgtools contiguous_regions
```

To get help documentation for bgtools, simply run `bgtools --help`.
Likewise, to get help documentation for any subcommand, run `bgtools <subcommand> --help`,
substituting the subcommand of interest for `<subcommand>`

Note that the above instructions assume the `bgtools` binary found in `target/release`
is in your `PATH` environment variable.

