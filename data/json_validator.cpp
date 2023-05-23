#include "data/json_validator.hpp"

#include "src/model/board.hpp"

#include <fmt/format.h>

#include <functional>
#include <string>
#include <string_view>
#include <vector>

namespace checker {

auto check_size(const json &board) -> std::string {
  if (board.size() == Board::get_number_of_tiles()) {
    return "";
  }
  return fmt::format("Must define only {} tiles.", Board::get_number_of_tiles());
}

auto check_name(const json &tile) -> std::string {
  if (tile.contains("name") && tile.at("name").is_string()) {
    return "";
  }
  return fmt::format("Tile must have property `name` with string type.");
}

auto check_display_name(const json &tile) -> std::string {
  if (tile.contains("display_name") && tile.at("display_name").is_string()) {
    const std::string &display_name = tile.at("display_name");

    // If space does not exist, then @word_2 will be same as @word_1
    // Does not change anything in terms of checks
    std::size_t space = display_name.find(' ');
    std::string word_1 = display_name.substr(0, space);
    std::string word_2 = display_name.substr(space + 1);

    if (word_1.length() <= Board::get_tile_length() &&
        word_2.length() <= Board::get_tile_length() &&
        display_name.length() <= (Board::get_tile_length() * 2 + 1)) {
      return "";
    }
  }
  return fmt::format(
      "Tile must have property `display_name` with string type. "
      "String must have at most 1 space (i.e. 1 or 2 words) "
      "where each word is a maximum of {} characters.",
      Board::get_tile_length()
  );
}

auto check_type(const json &tile) -> std::string {
  if (tile.contains("type") && tile.at("type").is_string()) {
    return "";
  }
  return fmt::format("Tile must have property `type` with string type. "
                     "String must be one of the tile's concrete child classes: "
                     "{{}}");
}

auto check_group(const json &tile) -> std::string {
  if (tile.contains("group") && tile.at("group").is_string()) {
    return "";
  }
  return fmt::format("Tile must have property `group` with string type. "
                     "String must be one of the defined colors in `utils/color.hpp.");
}

auto check_cost(const json &tile) -> std::string {
  bool correct_cost = tile.contains("cost") && tile.at("cost").is_number() &&
                      tile.at("cost") < Tile::get_maximum_cost();
  if (correct_cost || !tile.contains("cost")) {
    return "";
  }

  return fmt::format(
      "If tile has property `cost`, it must be int type and in range [0..{}].",
      Tile::get_maximum_cost()
  );
}

} // namespace checker

namespace validation {

// Checks the user defined board, specifically for our Monopoly use case
auto validate_board_json(const json &json_data) -> std::vector<std::string> {
  std::vector<std::string> errors;

  std::string size_error = checker::check_size(json_data);
  if (!size_error.empty()) {
    errors.emplace_back(checker::check_size(json_data));
  }

  using check_function = std::function<std::string(const json &tile)>;
  std::vector<check_function> checks{
    // Add all your wanted validators from the defined functions here
    checker::check_name,  checker::check_display_name, checker::check_type,
    checker::check_group, checker::check_cost,
  };

  for (auto &[key, tile_data] : json_data.items()) {
    for (const auto &check : checks) {
      std::string error = check(tile_data);
      if (!error.empty()) {
        errors.emplace_back(error);
      }
    }
  }

  return errors;
}

} // namespace validation
