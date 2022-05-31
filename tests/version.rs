/*********************** GNU General Public License 2.0 ***********************\
|                                                                              |
|  Copyright (C) 2022 Kevin Matthes                                            |
|                                                                              |
|  This program is free software; you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation; either version 2 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License along     |
|  with this program; if not, write to the Free Software Foundation, Inc.,     |
|  51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.                 |
|                                                                              |
\******************************************************************************/

//! Tests for the `Version` struct.
use cffreference::Version;

/// Test the method `fix_level`.
#[test]
fn fix_level() {
    assert_eq!(3, *setup().fix_level())
}

/// Test the method `major`.
#[test]
fn major() {
    assert_eq!(1, *setup().major())
}

/// Test the method `minor`.
#[test]
fn minor() {
    assert_eq!(2, *setup().minor())
}

/// Create a new object to test.
fn setup() -> Version {
    Version::new(1, 2, 3)
}

/******************************************************************************/
