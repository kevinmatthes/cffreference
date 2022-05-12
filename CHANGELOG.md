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
--      CHANGELOG.md
--
--  BRIEF
--      The development history of this project.
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
--      See `README.md' for project details.
--
------------------------------------------------------------------------------->

# Changelog

## [Unreleased]

### Changed

* optimise comment banners of `CITATION.cff`
* outsource `extract` to a dedicated function
* outsource `skipReferences` to a dedicated function

### Fixed

* add missing patterns in `Main.hs`

## [0.3] -- 2022-05-11

### Added

* cite `blank`
* cite `docs-snippets`

### Changed

* update `blank`
* update `docs-snippets`

### Fixed

* insufficient patterns for skipping references

## [0.2] -- 2022-05-10

### Added

* submodule `blank`

### Changed

* optimise `.gitignore`
* optimise repository manual build instruction
* update `docs-snippets`

### Fixed

* add missing `catch`
* docstrings which are not cited
* obsolete line in `CITATION.cff`

## [0.1] -- 2022-05-09

### Added

* add GPL 2.0 license
* build instruction:  GHC and Haddock compilation
* build instruction:  installation
* build instruction:  repository manual
* create `.gitignore`
* create `Main.hs`
* create project meta data for repository manual
* create repository README
* create this changelog
* create this repository
* submodule `docs-snippets`
* submodule `routines`

<!----------------------------------------------------------------------------->
