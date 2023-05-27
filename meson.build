project(
    'monopoly',
    'cpp',
    default_options: [
        'cpp_std=c++20',
        'buildtype=debug',
        'warning_level=3' # -Wall -Wextra -Wpedantic
    ]
)

compiler = meson.get_compiler('cpp')
if compiler.get_id() == 'clang' and compiler.version() < '16.0.0'
    warning('Clang version < 16.0.0 does not support ranges, so it may not compile')
elif compiler.get_id() == 'gcc'
    warning('Not really tested on GCC. Use the most recent version and supports C++20')
endif

# Need to install Google's re2
# git clone https://code.googlesource.com/re2 && cd re2
# sudo make && sudo make test && sudo make testinstall
json_dep = dependency('nlohmann_json', version : '>=3.10.5')
fmt_dep = dependency('fmt', version : '>=9.0.0')
re2_dep = dependency('re2', version : '>=10.0.0')

project_dep = [json_dep, fmt_dep, re2_dep]

build_args = ['-Wall', '-Wextra', '-Werror', '-Wpedantic', '-Wcast-align']

misc_src = [
    'data/json_validator.cpp'
]

controller_src = [
    'src/controller/controller.cpp',
    'src/controller/game.cpp',
]

tiles_src = [
    'src/model/tiles/event.cpp',
    'src/model/tiles/property.cpp',
    'src/model/tiles/street.cpp',
    'src/model/tiles/tile.cpp',
]

model_src = [
    tiles_src,
    'src/model/players/player.cpp',
    'src/model/board_config.cpp',
    'src/model/board.cpp',
]

view_src = [
    'src/view/view.cpp',
    'src/view/board_display.cpp',
]

src = [
    'src/main.cpp',
    misc_src,
    controller_src,
    model_src,
    view_src,
]

executable(
    meson.project_name(),
    src,
    dependencies: project_dep,
    cpp_args: build_args,
)
