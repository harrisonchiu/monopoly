#define FMT_HEADER_ONLY

#include <string>
#include <string_view>

#include <fmt/args.h>
#include <fmt/format.h>

#include <board.hpp>
#include <tiles/corner.hpp>
#include <tiles/street.hpp>
#include <utils/color.hpp>
#include <utils/substrings.hpp>

Board::Board(json &board_data) {
  ascii_board = create_base_board(board_data);

  for (int id = 0; id < number_of_tiles; id++) {
    json &tile_data = board_data[id];

    // Create actual tiles to be manipulated
    if (tile_data["type"] == "Street") {
      board.emplace_back(std::make_unique<Street>(tile_data, id));
    } else if (tile_data["type"] == "Railroad") {
      board.emplace_back(std::make_unique<Street>(tile_data, id));
    } else if (tile_data["type"] == "Utility") {
      board.emplace_back(std::make_unique<Street>(tile_data, id));
    } else if (tile_data["type"] == "Corner") {
      board.emplace_back(std::make_unique<Corner>(tile_data, id));
    } else {
      board.emplace_back(std::make_unique<Corner>(tile_data, id));
    }

    const std::string_view color = tile_data["group"].get<std::string_view>();
    tile_colors[id] = fmt::format(Color::get(color), "▔▔▔▔▔▔▔");
    tile_details[id] = board.back()->get_detail();
    tile_players[id] = std::string(length_of_tile, ' ');

    // Notify View that there are visual changes to the board, so update it
    tile_color_update_queue->push(id);
    tile_detail_update_queue->push(id);
    tile_player_update_queue->push(id);
  }
}

std::string Board::create_base_board(const json &board_data) {
  fmt::dynamic_format_arg_store<fmt::format_context> board_format_args;
  for (int tile_id = 0; tile_id < number_of_tiles; tile_id++) {
    const std::string tile_name = board_data[tile_id]["display_name"];

    // Only the top and bottom row need to split the name into 2 parts
    if ((tile_id >= 0 && tile_id <= 10) || (tile_id >= 20 && tile_id <= 30)) {
      // Assume name is 2 words seperated by 1 space, each word max 7 characters
      // If the name is only 1 word, make 2nd row empty or it will duplicate.
      size_t space = tile_name.find(' ');
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

  const auto repeat = [](int n, std::string str) {
    std::string s{};
    for (int i = 0; i < n; i++)
      s += str;
    return s;
  };

  // Named arguments must be done after positional arguments
  board_format_args.push_back(fmt::arg("INDENT", std::string(15, ' ')));
  board_format_args.push_back(fmt::arg(
      "IIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIIII", std::string(33, ' ')));
  board_format_args.push_back(fmt::arg(
      "NNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNNN",
      repeat(71, "▔")));

  return fmt::vformat(ascii_board, board_format_args);
}