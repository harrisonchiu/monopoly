#define FMT_HEADER_ONLY

#include "board.hpp"

#include "tiles/corner.hpp"
#include "tiles/street.hpp"
#include "utils/color.hpp"
#include "utils/substrings.hpp"

#include <fmt/args.h>
#include <nlohmann/json.hpp>

#include <string>
#include <string_view>

Board::Board(json &board_data) {
  ascii_board = create_base_board(board_data);

  for (int id = 0; id < number_of_tiles; ++id) {
    const json &tile_data = board_data[id];

    // Create actual tiles to be manipulated
    if (tile_data["type"] == "Street") {
      board.emplace_back(std::make_shared<Street>(tile_data, id));
    } else {
      board.emplace_back(std::make_shared<Corner>(tile_data, id));
    }

    // Create what each row of a tile should look like
    tile_colors.at(id) = fmt::format(Color::get(board.back()->get_group()), "▔▔▔▔▔▔▔");
    tile_details.at(id) = board.back()->get_detail();
    tile_players.at(id).fill(" ");

    // Notify View that there are visual changes to the board, so visually update it
    tile_color_update_queue->push(id);
    tile_detail_update_queue->push(id);
    tile_player_update_queue->push(id);
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

void Board::set_players(std::shared_ptr<std::vector<Player>> p, int tile_start) {
  players = std::move(p);
  for (int player_id = 0; player_id < Player::get_max_players(); ++player_id) {
    tile_players.at(tile_start).at(player_id) = players->at(player_id).get_avatar();
  }
  tile_player_update_queue->push(tile_start);
}

void Board::update_player_pos(int player_id) {
  const int current_pos = players->at(player_id).get_pos();
  const int last_pos = players->at(player_id).get_last_pos();

  // Must be std::string& to swap the references in the arrays
  std::string &previous_token_pos = tile_players.at(last_pos).at(player_id);
  std::string &current_token_pos = tile_players.at(current_pos).at(player_id);
  std::swap(previous_token_pos, current_token_pos);

  tile_player_update_queue->push(last_pos);
  tile_player_update_queue->push(current_pos);

  players->at(player_id).set_pos_updated(true);
}

// Wrapper function to easily move every player tokens
void Board::update_all_player_pos() {
  for (const auto &player : *players) {
    if (!player.is_pos_updated()) {
      update_player_pos(player.get_id());
    }
  }
}

auto Board::get_current_tile(int player_id) const -> std::shared_ptr<Tile> {
  const int current_pos = get_player(player_id).get_pos();

  return get_tile(current_pos);
}