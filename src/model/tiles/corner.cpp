#include "src/model/tiles/corner.hpp"
#include "src/model/tiles/tile.hpp"

#include "src/model/board.hpp"
#include "src/utils/substrings.hpp"

#include <string>

Corner::Corner(const json &tile_data, const int id) : Event(tile_data, id) { update_detail(); }

void Corner::update_detail() {
  static constexpr auto spaces = repeat_str<1, Board::get_tile_length()>(" ");
  static const std::string detail = std::string(spaces.begin(), spaces.end());
  set_detail(detail);
}