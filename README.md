# Rust News

A rust application which compiles rust-community news


## About

This is a little project to see if git-based collaboration is practical.
The goal is to provide a simple way to suggest items to be added to a news stream;
specifically regarding the rust programming language.

This repository has two parts:
+ A 'news' branch containing standard yaml files representing news posts
+ A 'master' and 'dev' branch which houses the rust application code that renders the yaml files

To submit news, fork this repository and pull only the news branch. Add/edit yaml files to your content
and submit a pull request.


### Submodules

The master & dev branches uses git's submodule feature to contain a reference
to the news branch as the data/ directory. This currently uses the remote
repository http://github.com/akubera/rust-news, as it appears you cannot use
the 'parent' repository as the submodule. (If anybody knows how, let me know!)

To clone, use `git clone --recursive <path-to-your-fork>` which should
automatically clone all submodules as well.
If that doesn't work (or you forgot) use `git submodule init`, then
`git submodule update` to pull the data/ directory. To update, simply pull from
within the submodule dir: `cd data/ && git pull && cd ..`. Again, this pulls
from http://github.com/akubera/rust-news and not your fork, unfortunately.

If you see a bunch of yaml files in data/ you're good to go.
