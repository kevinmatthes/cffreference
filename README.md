<!------------------------------------------------------------------------------
--
-- Copyright (C) 2022 Kevin Matthes
--
-- This program is free software; you can redistribute it and/or modify
-- it under the terms of the GNU General Public License as published by
-- the Free Software Foundation; either version 2 of the License, or
-- (at your option) any later version.
--
-- This program is distributed in the hope that it will be useful,
-- but WITHOUT ANY WARRANTY; without even the implied warranty of
-- MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
-- GNU General Public License for more details.
--
-- You should have received a copy of the GNU General Public License along
-- with this program; if not, write to the Free Software Foundation, Inc.,
-- 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
--
----
--
--  FILE
--      README.md
--
--  BRIEF
--      Important information regarding this project.
--
--  AUTHOR
--      Kevin Matthes
--
--  COPYRIGHT
--      (C) 2022 Kevin Matthes.
--      This file is licensed GPL 2 as of June 1991.
--
--  DATE
--      2022
--
--  NOTE
--      See `LICENSE' for full license.
--
------------------------------------------------------------------------------->

# cffreference

## Summary

A simple CLI to extract citation information from a CITATION.cff.

## License

This project's license is **GPL 2** (as of June 1991).  The whole license text
can be found in `LICENSE` in the main directory of this repository.  The brief
version is as follows:

> Copyright (C) 2022 Kevin Matthes
>
> This program is free software; you can redistribute it and/or modify
> it under the terms of the GNU General Public License as published by
> the Free Software Foundation; either version 2 of the License, or
> (at your option) any later version.
>
> This program is distributed in the hope that it will be useful,
> but WITHOUT ANY WARRANTY; without even the implied warranty of
> MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
> GNU General Public License for more details.
>
> You should have received a copy of the GNU General Public License along
> with this program; if not, write to the Free Software Foundation, Inc.,
> 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.

When compiling a printable version of this documentation using Pandoc, the full
license will be attached automatically to the resulting document.  This can be
invoked by calling `repository-manual.m`.

## Software Requirements

| Requirement       | Type          | Role                                  |
|:------------------|:-------------:|:--------------------------------------|
| `docs-snippets`   | repository    | documentation constants               |
| GHC               | application   | Haskell compiler                      |
| GNU Octave        | application   | execution of the provided scripts     |
| Haddock           | application   | creation of source code documentation |
| Pandoc            | application   | compilation of repository manual      |
| `texlive-full`    | package       | compilation of repository manual      |

The compilation of such an **optional** repository manual can be invoked by just
calling one of the following lines in a terminal.

```
octave repository-manual.m
octave-cli repository-manual.m
```

Both will redirect to the same application, GNU Octave, which will then create
the manual for this repository and attach the entire license to it.  The
resulting file, `repository.pdf`, will be saved in the main directory of this
repository.

## Description

A common task when working with CFF is adding another project's citation meta
data to the own `CITATION.cff`'s `references` section.  Since these files are
often very long -- due to having `references` to cite on their own or many
people who contributed, for instance -- and not sorted uniformly, it might be
a challenge to filter out all required lines by hand.  Furthermore, there can
also be a `preferred-citation` provided which could be overseen in a quite big
citation configuration file.

In order to solve these issues, `cffreference` provides the capability to
appropriately cite another project automatically.  Given an input file which is
valid according to the Schema Version 1.2.0, `cffreference` will extract the
relevant lines to cite.

By default, the results are written to `stdout`.  This behaviour is intended to
be a preview mode in order to verify that the correct repository is going to be
cited.  The extracted results can be also piped around in terminal sessions such
that other CFF tools can be instructed with `cffreference`'s output.

```
cffreference ./path/to/input/CITATION.cff
```

In order to append the extracted citation information to a certain, writable
`CITATION.cff`, its path must be given as the second argument to the application
which will then add the new reference at the end of the output file.

```
cffreference ./path/to/input/CITATION.cff ./path/to/output/CITATION.cff
```

In case no arguments or more than two are given, the application will show the
license information as well as a brief information on how to use it correctly.

```
cffreference
```

Any input file will be cited by its `preferred-citation`, by default.  If this
entry should not be specified, the whole file will be processed in order to
generate a proper `references` item.  In the latter case, the input file is
assumed to specify a `software`.

<!----------------------------------------------------------------------------->
