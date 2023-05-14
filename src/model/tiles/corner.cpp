#include "src/model/tiles/corner.hpp"
#include "src/model/tiles/tile.hpp"

#include "src/model/board.hpp"
#include "src/utils/color.hpp"

#include <string>

Corner::Corner(const json &tile_data, int id) : Event(tile_data, id) { update_detail(); }

void Corner::update_detail() { Tile::set_detail(Color::empty(Board::get_tile_length())); }