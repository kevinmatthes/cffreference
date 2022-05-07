{------------------------------------------------------------------------------]
|                                                                              |
| Copyright (C) 2022 Kevin Matthes                                             |
|                                                                              |
| This program is free software; you can redistribute it and/or modify         |
| it under the terms of the GNU General Public License as published by         |
| the Free Software Foundation; either version 2 of the License, or            |
| (at your option) any later version.                                          |
|                                                                              |
| This program is distributed in the hope that it will be useful,              |
| but WITHOUT ANY WARRANTY; without even the implied warranty of               |
| MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the                |
| GNU General Public License for more details.                                 |
|                                                                              |
| You should have received a copy of the GNU General Public License along      |
| with this program; if not, write to the Free Software Foundation, Inc.,      |
| 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.                  |
|                                                                              |
[------------------------------------------------------------------------------}

{-|
Module:      Main
Copyright:   (C) 2022 Kevin Matthes
License:     GPL 2.0
Maintainer:  Kevin Matthes
Stability:   develop
Portability: any
Description: The main source file of this project.

A simple application in order to extract citation information from a given CFF
file.
-}

module Main where

{------------------------------------------------------------------------------}

import System.Environment

{------------------------------------------------------------------------------}

{-|
The preprocessing function.

It will discard any pure comment and blank lines as well as:

- the @cff-version@ (since the target file will bring its own),
- the @message@ (since the input file is going to be cited),
- and the whole @references@ section (since the references of a reference do not
  need to be referenced).
-}

preprocess  :: [String]                                                         -- ^ The lines to preprocess.
            -> [String]                                                         -- ^ The remaining lines.
preprocess []       = []
preprocess (l:ls)   |  take 0xC l == "cff-version:" || take 0x8 l == "message:"
                    || take 0x1 l == "#"
                    || l == "\n" || l == "\r\n"
                    = preprocess ls

                    | otherwise
                    = l : preprocess ls

{------------------------------------------------------------------------------}

{-|
The main function.

It controls the behaviour of the application and invokes the other functions as
required.

The compiled application will always return with @0x0@.
-}

main    :: IO ()                                                                -- ^ This function returns nothing.
main    = do    args <- getArgs
                let argc = length args
                case argc of
                    0x1 -> do   text <- readFile $ head args
                                putStrLn . unlines . preprocess . lines $ text
                    _   -> putStrLn "Usage: cffreference <file name>"


{------------------------------------------------------------------------------}
