%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
%%
%% Copyright (C) 2022 Kevin Matthes
%%
%% This program is free software; you can redistribute it and/or modify
%% it under the terms of the GNU General Public License as published by
%% the Free Software Foundation; either version 2 of the License, or
%% (at your option) any later version.
%%
%% This program is distributed in the hope that it will be useful,
%% but WITHOUT ANY WARRANTY; without even the implied warranty of
%% MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
%% GNU General Public License for more details.
%%
%% You should have received a copy of the GNU General Public License along
%% with this program; if not, write to the Free Software Foundation, Inc.,
%% 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
%%
%%%%
%%
%%  FILE
%%      repository-manual.m
%%
%%  BRIEF
%%      Create a LaTeX documentation for this repository using Pandoc.
%%
%%  AUTHOR
%%      Kevin Matthes
%%
%%  COPYRIGHT
%%      (C) 2022 Kevin Matthes.
%%      This file is licensed GPL 2 as of June 1991.
%%
%%  DATE
%%      2022
%%
%%  NOTE
%%      See `LICENSE' for full license.
%%      See `README.md' for project details.
%%
%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%

%%%%
%%
%% Variables.
%%
%%%%

% Settings.
manual.language = 'British';
manual.paper    = 'a4-3x3-duplex';
manual.style    = 'scrartcl';



% Software.
software.compiler.self  = ' pandoc ';
software.compiler.flags = ' -N ';
software.compiler.call  = [software.compiler.self software.compiler.flags];



% Directories.
directories.md      = './docs-snippets/markdown/';
directories.yaml    = './docs-snippets/yaml/';



% Files.
files.code.begin    = [directories.md 'begin-code.md '];
files.code.end      = [directories.md 'end-code.md '];

files.license       = [files.code.begin ' ./LICENSE ' files.code.end];
files.license       = [directories.md 'heading-license.md ' files.license];

files.newpage       = [directories.md 'newpage.md '];

files.self          = ' repository-manual.m ';

files.source        = ' ./project.yaml ';
files.source        = [files.source directories.yaml manual.language '.yaml '];
files.source        = [files.source directories.yaml manual.paper '.yaml '];
files.source        = [files.source directories.yaml manual.style '.yaml '];
files.source        = [files.source files.newpage];
files.source        = [files.source ' ./README.md '];
files.source        = [files.source files.newpage];
files.source        = [files.source files.license];
files.source        = [files.source files.newpage];
files.source        = [files.source ' ./CHANGELOG.md '];
files.source        = [files.source files.newpage];

files.target        = ' repository.pdf ';



% Control flow.
banner  = ['[' files.self '] '];



% Call adjustment.
software.compiler.call  = [software.compiler.call files.source];
software.compiler.call  = [software.compiler.call ' -o ' files.target];



%%%%
%%
%% Build steps.
%%
%%%%

% Begin build instruction.
disp ([banner 'Begin build instruction.']);



% Call Pandoc.
disp ([banner 'Compile Pandoc documentation ...']);

disp (software.compiler.call);
system (software.compiler.call);

disp ([banner 'Done.']);



% End build instruction.
disp ([banner 'End build instruction.']);

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
