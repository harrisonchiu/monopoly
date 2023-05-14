project('monopoly', 'cpp', default_options : ['cpp_std=c++20'])

tiles_src = [
    'tiles/street.cpp',
    'tiles/corner.cpp'
]
src = ['main.cpp', 'board.cpp', 'player.cpp', 'view.cpp', 'controller.cpp', 'game.cpp', tiles_src]
# Need to install Google's re2
# git clone https://code.googlesource.com/re2 && cd re2
# sudo make && sudo make test && sudo make testinstall
executable('monopoly', src, link_args : '-lre2')

#test = ['main.cpp']
#executable('monopoly', test, link_args : '-lre2')



