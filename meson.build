project(
    'monopoly',
    'cpp',
    default_options: [
        'cpp_std=c++20',
        'buildtype=debug',
        'warning_level=3' # -Wall -Wextra -Wpedantic
    ]
)

# Need to install Google's re2
# git clone https://code.googlesource.com/re2 && cd re2
# sudo make && sudo make test && sudo make testinstall
json_dep = dependency('nlohmann_json', version : '>=3.10.5')
fmt_dep = dependency('fmt', version : '>=9.0.0')
re2_dep = dependency('re2', version : '>=10.0.0')

project_dep = [json_dep, fmt_dep, re2_dep]

build_args = ['-Wall', '-Wextra', '-Werror', '-Wpedantic', '-Wcast-align']

tiles_src = [
    'src/model/tiles/street.cpp',
    'src/model/tiles/corner.cpp',
    'src/model/tiles/property.cpp',
    'src/model/tiles/event.cpp',
    'src/model/tiles/tile.cpp',
]

src = [
    'src/main.cpp',
    'src/model/board.cpp',
    'src/model/player.cpp',
    'src/view/view.cpp',
    'src/controller/controller.cpp',
    'src/controller/game.cpp',
    tiles_src
]

executable(
    meson.project_name(),
    src,
    dependencies: project_dep,
    cpp_args: build_args,
)
