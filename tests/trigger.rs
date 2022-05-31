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

//! Tests for the `Trigger` struct.
use cffreference::Trigger;

/// Test the method `help`.
#[test]
fn help() {
    assert_eq!("help", setup().help())
}

/// Test the method `input`.
#[test]
fn input() {
    assert_eq!("input", setup().input())
}

/// Test the method `license`.
#[test]
fn license() {
    assert_eq!("license", setup().license())
}

/// Test the method `output`.
#[test]
fn output() {
    assert_eq!("output", setup().output())
}

/// Create a new object to test.
fn setup() -> Trigger {
    Trigger::new("help", "input", "license", "output", "version")
}

/// Test the method `version`.
#[test]
fn version() {
    assert_eq!("version", setup().version())
}

/******************************************************************************/
