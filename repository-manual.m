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

% Directories.
directories.md      = './docs-snippets/markdown/';
directories.yaml    = './docs-snippets/yaml/';

% Settings.
settings.language   = [directories.yaml 'British'       '.yaml'];
settings.paper      = [directories.yaml 'a4-3x3-duplex' '.yaml'];
settings.style      = [directories.yaml 'scrartcl'      '.yaml'];

% Files.
files.code.begin    = [directories.md 'begin-code.md'];
files.code.end      = [directories.md 'end-code.md'];
files.license       = [ directories.md 'heading-license.md'              ' ' ...
                        files.code.begin ' ' './LICENSE' ' ' files.code.end  ...
                      ];
files.newpage       = [directories.md 'newpage.md'];

% Software.
pandoc.args = '-N';
pandoc.in   = [ './project.yaml'                                         ' ' ...
                settings.language ' ' settings.paper ' ' settings.style  ' ' ...
                files.newpage ' ' './'                  'README.md'      ' ' ...
                files.newpage ' ' files.license                          ' ' ...
                files.newpage ' ' './CHANGELOG.md'                       ' ' ...
              ];
pandoc.out  = 'repository.pdf';
pandoc.self = 'pandoc';
pandoc.call = [pandoc.self ' ' pandoc.args ' -o ' pandoc.out ' ' pandoc.in];

% Miscellaneous.
misc.self   = 'repository-manual.m';
misc.banner = ['[ ' misc.self ' ] '];



%%%%
%%
%% Build steps.
%%
%%%%

% Begin build instruction.
disp ([misc.banner 'Begin build instruction.']);

% Call Pandoc.
disp ([misc.banner 'Compile Pandoc documentation ...']);
disp (pandoc.call);
system (pandoc.call);
disp ([misc.banner 'Done.']);

% End build instruction.
disp ([misc.banner 'End build instruction.']);

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
