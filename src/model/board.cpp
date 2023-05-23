#define FMT_HEADER_ONLY

#include "src/model/board.hpp"

#include "src/model/tiles/corner.hpp"
#include "src/model/tiles/street.hpp"
#include "src/utils/substrings.hpp"

#include <fmt/args.h>
#include <nlohmann/json.hpp>

#include <string>
#include <string_view>

Board::Board(const json &board_data) {
  ascii_board = create_base_board(board_data);

  // @Tile ids MUST be int [0, 39] so we can easily refer to one in std::vector<Tile>
  for (int tile_id = 0; auto &[key, tile_data] : board_data.items()) {
    // Create actual tiles to be manipulated
    if (tile_data["type"] == "Street") {
      board.emplace_back(std::make_shared<Street>(tile_data, tile_id));
    } else {
      board.emplace_back(std::make_shared<Corner>(tile_data, tile_id));
    }

    // Color and detail are part of the tile itself, so store it in @Tile
    // Pieces (players) are placed on the board, so store it here
    tile_players.at(tile_id).fill(" ");
    tile_color_update_queue.push(tile_id);
    tile_detail_update_queue.push(tile_id);
    tile_player_update_queue.push(tile_id);
    ++tile_id;
  }
}

auto Board::create_base_board(const json &board_data) -> std::string {
  constexpr int bot_row_start = 0;
  constexpr int bot_row_end = 10;
  constexpr int top_row_start = 20;
  constexpr int top_row_end = 30;

  fmt::dynamic_format_arg_store<fmt::format_context> board_format_args;
  for (int tile_id = 0; tile_id < number_of_tiles; ++tile_id) {
    const std::string tile_name = board_data[tile_id]["display_name"];

    // Only the top and bottom row need to split the name into 2 parts
    if ((tile_id >= bot_row_start && tile_id <= bot_row_end) ||
        (tile_id >= top_row_start && tile_id <= top_row_end)) {
      // Assume name is 2 words seperated by 1 space, each word max 7 characters
      // If the name is only 1 word, make 2nd row empty or it will duplicate.
      std::size_t space = tile_name.find(' ');
      if (space != std::string::npos) {
        board_format_args.push_back(tile_name.substr(0, space));
        board_format_args.push_back(tile_name.substr(space + 1));
      } else {
        board_format_args.push_back(tile_name.substr(0, space));
        board_format_args.push_back("");
      }
    } else {
      board_format_args.push_back(tile_name);
    }
  }

  // Named arguments must be done after positional arguments
  constexpr auto side_pad = repeat_str<1, tile_length * 2 + 1>(" ");
  board_format_args.push_back(
      fmt::arg("INDENT", std::string_view(side_pad.begin(), side_pad.end()))
  );

  constexpr auto center_pad = repeat_str<1, 33>(" ");
  board_format_args.push_back(fmt::arg(
      "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII",
      std::string_view(center_pad.begin(), center_pad.end())
  ));

  // 3 in repeat_str<> is the size of the string ▔. Unicode chars must be string
  constexpr auto border_box = repeat_str<3, tile_length * 10 + 1>("▔");
  board_format_args.push_back(fmt::arg(
      "NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN",
      std::string_view(border_box.begin(), border_box.end())
  ));

  return fmt::vformat(base_board, board_format_args);
}

// Add player pieces to the board. Use after creating the board
void Board::place_player_pieces(const Player &player, const int tile_start) {
  tile_players.at(tile_start).at(player.get_id()) = player.get_avatar();
  tile_player_update_queue.push(tile_start);
}

// @Player may have moved, but @Board may not have moved the piece itself.
// @View relies on this to visually update player movement.
// Assume pieces exist on the board
void Board::move_player_piece(Player &player) {
  const int player_id = player.get_id();
  const int current_pos = player.get_pos();
  const int last_pos = player.get_last_pos();

  // Exit early if the piece does not need to be moved
  if (player.is_pos_updated() || current_pos == last_pos) {
    return;
  }

  std::string &previous_piece_pos = tile_players.at(last_pos).at(player_id);
  std::string &current_piece_pos = tile_players.at(current_pos).at(player_id);
  std::swap(previous_piece_pos, current_piece_pos);

  tile_player_update_queue.push(last_pos);
  tile_player_update_queue.push(current_pos);

  player.set_pos_updated(true);
}
