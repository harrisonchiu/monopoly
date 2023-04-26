#define FMT_HEADER_ONLY

#include <algorithm>
#include <iostream>

#include <fmt/args.h>
#include <fmt/color.h>
#include <fmt/format.h>

#include <board.hpp>
#include <tiles/corner.hpp>
#include <tiles/street.hpp>
#include <utils/color.hpp>

Board::Board(json &board_data) {
  ascii_board = create_base_board(board_data);
  color_indices = find_substrings(ascii_board, color_placeholder);
  detail_indices = find_substrings(ascii_board, detail_placeholder);
  player_indices = find_substrings(ascii_board, player_placeholder);

  for (int tile_id = 0; tile_id < number_of_tiles; tile_id++) {
    json &tile_data = board_data[tile_id];

    // Create actual tiles to be manipulated
    if (tile_data["type"] == "Street") {
      board.emplace_back(std::make_unique<Street>(tile_data, tile_id));
    } else if (tile_data["type"] == "Railroad") {
      board.emplace_back(std::make_unique<Street>(tile_data, tile_id));
    } else if (tile_data["type"] == "Utility") {
      board.emplace_back(std::make_unique<Street>(tile_data, tile_id));
    } else if (tile_data["type"] == "Corner") {
      board.emplace_back(std::make_unique<Corner>(tile_data, tile_id));
    } else {
      board.emplace_back(std::make_unique<Corner>(tile_data, tile_id));
    }

    // Update the base ascii board to have the appropriate colors and details
    update_tile_color(board.back()->get_group(), tile_id);
    update_tile_detail(board.back()->get_detail(), tile_id);
  }
}

std::vector<int> Board::find_substrings(const std::string &str,
                                        const std::string &substr) const {
  std::vector<int> indices;
  indices.reserve(number_of_tiles);

  size_t position = 0;
  while ((position = str.find(substr, position)) != std::string::npos) {
    indices.emplace_back(position);
    position += substr.length();
  }

  return indices;
}

std::string Board::create_base_board(const json &board_data) {
  constexpr auto padding = [](const int spaces) -> std::string {
    return std::string(spaces, ' ');
  };

  std::string base_board =
      fmt::format(fmt::runtime(ascii_board), fmt::arg("INDENT", padding(2)),
                  fmt::arg("COLOR", color_placeholder),
                  fmt::arg("INFOS", detail_placeholder),
                  fmt::arg("USERS", player_placeholder),
                  fmt::arg("SIDE_PAD", padding(length_of_tile * 2 + 1)),
                  fmt::arg("CORE_PAD", padding(0)));

  fmt::dynamic_format_arg_store<fmt::format_context> tile_names;
  for (int tile_id = 0; tile_id < number_of_tiles; tile_id++) {
    const std::string name = board_data[tile_id]["display_name"];

    // Only the top and bottom row need to split the name into 2 parts
    if ((tile_id >= 0 && tile_id <= 10) || (tile_id >= 20 && tile_id <= 30)) {
      // Assume name is 2 words seperated by 1 space, each word max 7 characters
      // If the name is only 1 word, make 2nd row empty or it will duplicate.
      std::size_t space = name.find(' ');
      if (space != std::string::npos) {
        tile_names.push_back(name.substr(0, space));
        tile_names.push_back(name.substr(space + 1));
      } else {
        tile_names.push_back(name.substr(0, space));
        tile_names.push_back("");
      }
    } else {
      tile_names.push_back(name);
    }
  }

  return fmt::vformat(base_board, tile_names);
}

const Size Board::get_size() const {
  constexpr char nl = '\n';
  const size_t first_nl = ascii_board.find(nl);
  const size_t second_nl = ascii_board.find(nl, first_nl + 1);
  const int width = static_cast<int>(second_nl - first_nl);
  const int height = std::count(ascii_board.begin(), ascii_board.end(), nl);

  return Size{width, height};
}

void Board::update_tile_color(const std::string &group, const int tile_id) {
  ascii_board.replace(color_indices[actual_to_visual_order[tile_id]],
                      color_placeholder.length(),
                      fmt::format(Color::get(group), "▔▔▔▔▔▔▔"));
}

void Board::update_tile_detail(const std::string &detail, const int tile_id) {
  ascii_board.replace(detail_indices[actual_to_visual_order[tile_id]],
                      detail_placeholder.length(), detail);
}

void Board::display_board() const { fmt::print(fmt::runtime(ascii_board)); }