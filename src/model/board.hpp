#ifndef BOARD_HPP
#define BOARD_HPP

#include "src/model/players/player.hpp"
#include "src/model/tiles/tile.hpp"
#include "src/utils/sorting.hpp"
#include "src/utils/substrings.hpp"
#include "src/view/components.hpp"

#include <nlohmann/json.hpp>

#include <array>
#include <queue>
#include <ranges>
#include <string>
#include <string_view>
#include <vector>

class Board {
  using json = nlohmann::json;

  using update_queue = std::queue<int>;
  using presence = std::array<std::string, Player::get_max_players()>;

private:
  static constexpr int tile_length = 7; // Length in chars
  static constexpr int number_of_tiles = 40;
  static constexpr auto tile_ids = std::views::iota(0, number_of_tiles);

  // Support only for clang 16.0.0+ :(, I am on clang 15.0.0 and too lazy
  // static constexpr auto tile_ids = std::views::iota(0, number_of_tiles);

  // @ascii_board is the main structure of the board with display_names
  // Originally, @ascii_board had substrings replaced and entire @ascii_board reprited, but it
  // would cause the terminal screen to flash which is annoying.
  std::string ascii_board;
  std::vector<std::shared_ptr<Tile>> board;

  // @Players are the only row @Board keeps track of because color and detail should be part of
  // @Tile. @Players are "on" the board, so they are not really part of @Tile.
  std::array<presence, number_of_tiles> tile_players{};

  // The @Tiles that must be visually updated because some change happened to it
  // One way notifier of tiles needing update: @Board -> @View
  // @View should never add to these update queues
  update_queue tile_color_update_queue;
  update_queue tile_detail_update_queue;
  update_queue tile_player_update_queue;

  // Named arguments are chosen with specific length to ensure this board string has the general
  // dimensions and visuals of the board.
  // This does not contain unicode chars (e.g. ▔). Easier to iterate unicode string with wchar, but
  //  has bad constexpr, fmt, and normal string conversion support.
  // {NNN...} is placeholder for unicode char ▔, so we can iterate this easily to find substrings
  //  indices because unicodes are considered multiple chars.
  static constexpr std::string_view base_board{ R"""(
   {31:^7} {33:^7} {35:^7} {37:^7} {39:^7} {41:^7} {43:^7} {45:^7} {47:^7} {49:^7} {51:^7} 
   {32:^7} {34:^7} {36:^7} {38:^7} {40:^7} {42:^7} {44:^7} {46:^7} {48:^7} {50:^7} {52:^7} 
  |CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|
  |DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|
  |PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|
  |CCCCCCC|{NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN}|CCCCCCC|
  |DDDDDDD| {30:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {53:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |PPPPPPP|
  |CCCCCCC| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |CCCCCCC|
  |DDDDDDD| {29:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {54:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |PPPPPPP|
  |CCCCCCC| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |CCCCCCC|
  |DDDDDDD| {28:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {55:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |PPPPPPP|
  |CCCCCCC| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |CCCCCCC|
  |DDDDDDD| {27:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {56:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |PPPPPPP|
  |CCCCCCC| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |CCCCCCC|
  |DDDDDDD| {26:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {57:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |PPPPPPP|
  |CCCCCCC| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |CCCCCCC|
  |DDDDDDD| {25:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {58:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |PPPPPPP|
  |CCCCCCC| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |CCCCCCC|
  |DDDDDDD| {24:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {59:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |PPPPPPP|
  |CCCCCCC| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |CCCCCCC|
  |DDDDDDD| {23:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {60:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |PPPPPPP|
  |CCCCCCC| {COLUMNPADDING} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {COLUMNPADDING} |CCCCCCC|
  |DDDDDDD| {22:<{NAMEPAD}} | {IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII} | {61:>{NAMEPAD}} |DDDDDDD|
  |PPPPPPP|                                                                       |PPPPPPP|
  |CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|CCCCCCC|
  |DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|DDDDDDD|
  |PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|PPPPPPP|
   ▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔▔
   {20:^7} {18:^7} {16:^7} {14:^7} {12:^7} {10:^7} {8:^7} {6:^7} {4:^7} {2:^7} {0:^7} 
   {21:^7} {19:^7} {17:^7} {15:^7} {13:^7} {11:^7} {9:^7} {7:^7} {5:^7} {3:^7} {1:^7} 
)""" };

  // Actual ids label each tile starting from GO (0) to the last tile (39)
  // Visual ids label each tile iterated as a multi-lined string from top to bottom, left to right
  // Given the actual tile ids (index), get the visual tile ids (value)
  static constexpr std::array<int, number_of_tiles> actual_to_visual_order{
    39, 38, 37, 36, 35, 34, 33, 32, 31, 30, 29, 27, 25, 23, 21, 19, 17, 15, 13, 11,
    0,  1,  2,  3,  4,  5,  6,  7,  8,  9,  10, 12, 14, 16, 18, 20, 22, 24, 26, 28
  };

  // Given the visual tile ids (index), get the actual tile ids (value)
  static constexpr std::array<int, number_of_tiles> visual_to_actual_order{
    20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 19, 31, 18, 32, 17, 33, 16, 34, 15,
    35, 14, 36, 13, 37, 12, 38, 11, 39, 10, 9,  8,  7,  6,  5,  4,  3,  2,  1,  0
  };

  // Find the (X, Y) coords of the substrings where origin (0, 0) is top left
  // Finding and sorting to actual tile order should be done during compilation
  static constexpr std::array<Position, number_of_tiles> color_pos =
      sort_by_order<Position, number_of_tiles>(
          find_pos<number_of_tiles>(base_board, "CCCCCCC"), visual_to_actual_order
      );
  static constexpr std::array<Position, number_of_tiles> detail_pos =
      sort_by_order<Position, number_of_tiles>(
          find_pos<number_of_tiles>(base_board, "DDDDDDD"), visual_to_actual_order
      );
  static constexpr std::array<Position, number_of_tiles> player_pos =
      sort_by_order<Position, number_of_tiles>(
          find_pos<number_of_tiles>(base_board, "PPPPPPP"), visual_to_actual_order
      );

  static constexpr Position center =
      find_pos<1>(base_board, "{IIIIIIIIIIIIIIIIIIIIIIIIIIIIIII}")[0];

  static auto create_base_board(const json &board_data) -> std::string;

public:
  explicit Board(const json &board_data);
  static constexpr auto get_tile_length() -> int { return tile_length; }
  static constexpr auto get_number_of_tiles() -> int { return number_of_tiles; }
  static constexpr auto get_tile_ids() { return tile_ids; }

  auto get_board_str() const -> std::string_view { return ascii_board; }

  static constexpr auto get_color_pos(int id) -> const Position & { return color_pos.at(id); }
  static constexpr auto get_detail_pos(int id) -> const Position & { return detail_pos.at(id); }
  static constexpr auto get_player_pos(int id) -> const Position & { return player_pos.at(id); }
  static constexpr auto get_center_pos() -> const Position & { return center; }

  // @View should not use this to request an update; it should update itself directly
  auto get_color_update_queue() -> update_queue & { return tile_color_update_queue; }
  auto get_detail_update_queue() -> update_queue & { return tile_detail_update_queue; }
  auto get_player_update_queue() -> update_queue & { return tile_player_update_queue; }

  auto get_tile(int tile_id) const -> const std::shared_ptr<Tile> & { return board.at(tile_id); }
  auto get_tile_players(int tile_id) const -> const presence & { return tile_players.at(tile_id); }

  void place_player_pieces(const Player &player, int tile_start);
  void move_player_piece(Player &player);

  static constexpr auto get_size() -> Size {
    // This seems to have to be declared in the header??
    // No way the base_board has only 2 lines right?
    // Assume first 2 lines represent the width of the board and are non-unicode chars (1 byte)
    constexpr auto newlines = find_substrs<2>(base_board, "\n");
    constexpr int width = static_cast<int>(newlines[1] - newlines[0]);
    constexpr int height = std::count(base_board.begin(), base_board.end(), '\n');

    return Size{ width, height };
  }
};

#endif // BOARD_HPP