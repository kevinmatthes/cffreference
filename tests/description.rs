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

//! Tests for the `Description` struct.
use cffreference::Description;

/// Test the method `name`.
#[test]
fn name() {
    assert_eq!("name", setup().name())
}

/// Create a new object to test.
fn setup() -> Description {
    Description::new("name", "summary", "synopsis", "usage")
}

/// Test the method `summary`.
#[test]
fn summary() {
    assert_eq!("summary", setup().summary())
}

/// Test the method `synopsis`.
#[test]
fn synopsis() {
    assert_eq!("synopsis", setup().synopsis())
}

/// Test the method `usage`.
#[test]
fn usage() {
    assert_eq!("usage", setup().usage())
}

/******************************************************************************/
