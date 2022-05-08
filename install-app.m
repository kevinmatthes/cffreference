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
%%      install-app.m
%%
%%  BRIEF
%%      Install a compilation of the provided application.
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
octave.in   = ['' '.m'];
octave.out  = ['./' ''];
octave.self = 'octave';
octave.call = [octave.self ' ' octave.in];

% Directories.
directories.dep1    = '~/.local/';
directories.target  = '~/.local/bin/';

% Miscellaneous.
misc.self   = 'install-app.m';
misc.banner = ['[ ' misc.self ' ] '];



%%%%
%%
%% Build steps.
%%
%%%%

% Begin build instruction.
disp ([misc.banner 'Begin build instruction.']);



% Compile application.
disp ([misc.banner 'Compile the application to install ...']);
system (octave.call);
disp ([misc.banner 'Done.']);



% Create directories, if required.
fprintf ([misc.banner 'Check for ' directories.dep1 ' ... ']);
if length (glob (directories.dep1));
    disp ('Done.');
else;
    mkdir (directories.dep1);
    disp ('Created.');
end;

fprintf ([misc.banner 'Check for ' directories.target ' ... ']);
if length (glob (directories.target));
    disp ('Done.');
else;
    mkdir (directories.target);
    disp ('Created.');
end;



% Install.
fprintf ([misc.banner 'Install ' octave.out ' in ' directories.target ' ... ']);
movefile (octave.out, directories.target, 'f');
disp ('Done.');



% End build instruction.
disp ([misc.banner 'End build instruction.']);

%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%%
