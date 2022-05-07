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
The finalisation.

Before returning the results, the remaining lines will be indented appropriately
in order to directly append them to the target @CITATION.cff@.
-}

postprocess :: [String]                                                         -- ^ The remaining lines.
            -> [String]                                                         -- ^ The intended reference object.
postprocess x@(l:ls)    | null x
                        = []

                        | take 0x2 l == "  "
                        = "  - " ++ drop 0x2 l : map ("  " ++) ls

                        | otherwise
                        = "  - " ++ l : map ("    " ++) ls

{------------------------------------------------------------------------------}

{-|
The processing function.

It will check whether the given file contains a @preferred-citation@.  If so,
the respective lines will be returned as result.  If not so, the lines will be
returned unchanged.
-}

process :: [String]                                                             -- ^ The lines to extract the preferred citation from.
        -> [String]                                                             -- ^ The determined lines to cite.
process ls  | null ls
            = []

            | "preferred-citation:" `elem` ls
            = extract ls

            | otherwise
            = ls

            where extract x@(l:ls)  | (not . null) x && take 0x1 l == " "
                                    = l : extract ls

                                    | otherwise
                                    = []

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
preprocess x@(l:ls) | null x
                    = []

                    |  take 0xC l == "cff-version:" || take 0x8 l == "message:"
                    || take 0x1 l == "#" || null l
                    = preprocess ls

                    | take 0xB l == "references:"
                    = skipReferences ls

                    | otherwise
                    = l : preprocess ls

                    where skipReferences x@(l:ls)   | null ls
                                                    = []

                                                    | take 0x1 l == " "
                                                    = skipReferences ls

                                                    | otherwise
                                                    = ls

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
                                putStr . unlines
                                       . postprocess . process . preprocess
                                       . lines
                                       $ text
                    _   -> putStrLn "Usage: cffreference <file name>"


{------------------------------------------------------------------------------}
