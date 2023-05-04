#ifndef BOARD_HPP
#define BOARD_HPP

#include <array>
#include <queue>
#include <string>
#include <string_view>
#include <vector>

#include <fmt/color.h>
#include <fmt/compile.h>
#include <nlohmann/json.hpp>

#include <tiles/tile.hpp>
#include <utils/color.hpp>
#include <utils/component.hpp>
#include <utils/sorting.hpp>
#include <utils/substrings.hpp>

class Board {
  using json = nlohmann::json;

private:
  static constexpr int length_of_tile = 7;
  static constexpr int number_of_tiles = 40;

  // Given the actual tile ids (index), get the visual tile ids (value)
  static constexpr std::array<int, number_of_tiles> actual_to_visual_order{
      39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 27, 25, 23,
      21, 19, 17, 15, 13, 11, 0,  1,  2,  3,  4,  5,  6,  7,
      8,  9,  10, 12, 14, 16, 18, 20, 22, 24, 26, 28};

  // Given the visual tile ids (index), get the actual tile ids (value)
  static constexpr std::array<int, number_of_tiles> visual_to_actual_order{
      20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 19, 31, 18,
      32, 17, 33, 16, 34, 15, 35, 14, 36, 13, 37, 12, 38, 11,
      39, 10, 9,  8,  7,  6,  5,  4,  3,  2,  1,  0};
  // Actual ids label each tile starting from GO (0) to the last tile (39)
  // Visual ids label each tile iterated as a multi-lined string
  // starting from top to bottom, left to right

  std::vector<std::unique_ptr<Tile>> board;

  // @ascii_board is the main structure of the board with display_names
  // tile_* holds the strings that will be printed ontop of @ascii_board
  // Originally, @ascii_board had substrings replaced and entire @ascii_board
  // reprited, but it would cause the terminal screen to flash which is
  // annoying. Colors could be part of @ascii_board instead of the arrays, but
  // it is more consistent with the other tile rows
  std::string ascii_board;
  std::array<std::string, number_of_tiles> tile_colors;
  std::array<std::string, number_of_tiles> tile_details;
  std::array<std::string, number_of_tiles> tile_players;

  using update_queue = std::shared_ptr<std::queue<int>>;
  update_queue(tile_color_update_queue) = std::make_shared<std::queue<int>>();
  update_queue(tile_detail_update_queue) = std::make_shared<std::queue<int>>();
  update_queue(tile_player_update_queue) = std::make_shared<std::queue<int>>();

  // Named arguments with brackets have the same length as the actual tile
  // length. Therefore, it has the dimensions and visual base look of the board
  // even before the placeholders are put there.
  // This uses unicode char ▔. It is easier to iterate with wchar_t or wstring
  // variations but it does not support constexpr as well and cannot easily
  // convert to and from regular string.
  // {NNN...} replaces unicode box char ▔, to easily to find substrings indices
  // Unicode chars are considered as multiple chars. They are bigger than ASCII
  static constexpr std::string_view base_board = R"""(
   {31:^7} {33:^7} {35:^7} {37:^7} {39:^7} {41:^7} {43:^7} {45:^7} {47:^7} {49:^7} {51:^7} 
   {32:^7} {34:^7} {36:^7} {38:^7} {40:^7} {42:^7} {44:^7} {46:^7} {48:^7} {50:^7} {52:^7} 
  |CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|
  |DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|
  |PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|
  |CCCCCCC|{NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN}|CCCCCCC|
  |DDDDDDD| {30:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {53:>15} |DDDDDDD|
  |PPPPPPP| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |PPPPPPP|
  |CCCCCCC| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |CCCCCCC|
  |DDDDDDD| {29:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {54:>15} |DDDDDDD|
  |PPPPPPP| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |PPPPPPP|
  |CCCCCCC| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |CCCCCCC|
  |DDDDDDD| {28:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {55:>15} |DDDDDDD|
  |PPPPPPP| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |PPPPPPP|
  |CCCCCCC| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |CCCCCCC|
  |DDDDDDD| {27:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {56:>15} |DDDDDDD|
  |PPPPPPP| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |PPPPPPP|
  |CCCCCCC| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |CCCCCCC|
  |DDDDDDD| {26:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {57:>15} |DDDDDDD|
  |PPPPPPP| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |PPPPPPP|
  |CCCCCCC| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |CCCCCCC|
  |DDDDDDD| {25:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {58:>15} |DDDDDDD|
  |PPPPPPP| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |PPPPPPP|
  |CCCCCCC| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |CCCCCCC|
  |DDDDDDD| {24:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {59:>15} |DDDDDDD|
  |PPPPPPP| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |PPPPPPP|
  |CCCCCCC| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |CCCCCCC|
  |DDDDDDD| {23:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {60:>15} |DDDDDDD|
  |PPPPPPP| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |PPPPPPP|
  |CCCCCCC| {INDENT} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {INDENT} |CCCCCCC|
  |DDDDDDD| {22:<15} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {61:>15} |DDDDDDD|
  |PPPPPPP|                                                                       |PPPPPPP|
  |CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|
  |DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|
  |PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|
   ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
   {20:^7} {18:^7} {16:^7} {14:^7} {12:^7} {10:^7} {8:^7} {6:^7} {4:^7} {2:^7} {0:^7}      
   {21:^7} {19:^7} {17:^7} {15:^7} {13:^7} {11:^7} {9:^7} {7:^7} {5:^7} {3:^7} {1:^7}      
)""";

  // Find the (X, Y) coords of the substrings where origin (0, 0) is top left
  // Finding and sorting to actual tile order should be done during compilation
  static constexpr std::array<Position, number_of_tiles> color_coords =
      sort_by_order<Position, number_of_tiles>(
          find_coords<number_of_tiles>(base_board, "CCCCCCC"),
          visual_to_actual_order);
  static constexpr std::array<Position, number_of_tiles> detail_coords =
      sort_by_order<Position, number_of_tiles>(
          find_coords<number_of_tiles>(base_board, "DDDDDDD"),
          visual_to_actual_order);
  static constexpr std::array<Position, number_of_tiles> player_coords =
      sort_by_order<Position, number_of_tiles>(
          find_coords<number_of_tiles>(base_board, "PPPPPPP"),
          visual_to_actual_order);

  std::string create_base_board(const json &board_data);

public:
  Board(json &board_data);
  static constexpr int get_length_of_tile() { return length_of_tile; }
  static constexpr int get_number_of_tiles() { return number_of_tiles; }

  static constexpr auto get_color_coord(int id) { return &color_coords[id]; }
  static constexpr auto get_detail_coord(int id) { return &detail_coords[id]; }
  static constexpr auto get_player_coord(int id) { return &player_coords[id]; }

  update_queue &get_color_queue() { return tile_color_update_queue; }
  update_queue &get_detail_queue() { return tile_detail_update_queue; }
  update_queue &get_player_queue() { return tile_player_update_queue; }

  std::string_view get_board() const { return ascii_board; }
  std::string_view get_tile_color(int id) const { return tile_colors[id]; }
  std::string_view get_tile_detail(int id) const { return tile_details[id]; }
  std::string_view get_tile_player(int id) const { return tile_players[id]; }

  static constexpr Size get_size() {
    // This seems to have to be declared in the header??
    // No way the base_board is every smaller than 10 lines right?
    constexpr auto newlines = find_substrs<10>(base_board, "\n");
    constexpr int w = static_cast<int>(newlines[1] - newlines[0]);
    constexpr int h = std::count(base_board.begin(), base_board.end(), '\n');

    return Size{w, h};
  }
};

#endif // BOARD_HPP