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
%%      ghc-haddock.m
%%
%%  BRIEF
%%      Compile a Haskell project together with its documentation.
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

% Software.
compiler.args   = '--make -static -fasm -fPIE -O2';
compiler.hidir  = ['-hidir' ' ' 'build'];
compiler.in     = '*.hs';
compiler.odir   = ['-odir' ' ' 'build'];
compiler.out    = ['./' 'cffreference'];
compiler.self   = 'ghc';
compiler.call   = [ compiler.self ' ' compiler.args ' ' compiler.hidir ' '   ...
                    compiler.odir ' ' compiler.in ' -o ' compiler.out        ...
                  ];

haddock.args    = '--html';
haddock.in      = '*.hs';
haddock.out     = ['./' 'html/'];
haddock.self    = 'haddock';
haddock.call    = [ haddock.self ' ' haddock.args ' -o ' haddock.out ' '     ...
                    haddock.in                                               ...
                  ];



% Miscellaneous.
misc.self   = 'ghc-haddock.m';
misc.banner = ['[ ' misc.self ' ] '];



%%%%
%%
%% Build steps.
%%
%%%%

% Begin build instruction.
disp ([misc.banner 'Begin build instruction.']);

% Clean outdated compilation.
if length (glob (compiler.out));
    fprintf ([misc.banner 'Remove outdated compilation ... ']);
    delete (compiler.out);
    disp (compiler.out);
end;

% Compile Haskell code with GHC.
disp ([misc.banner 'Compile ' compiler.out ' with GHC ...']);
disp (compiler.call);
system (compiler.call);
disp ([misc.banner 'Done.']);

% Compile Haddock documentation, if possible.
if length (glob (compiler.out));
    disp ([misc.banner 'Compile Haddock documentation ...']);
    disp (haddock.call);
    system (haddock.call);
    disp ([misc.banner 'Done.']);
end;

% End build instruction.
disp ([misc.banner 'End build instruction.']);

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
