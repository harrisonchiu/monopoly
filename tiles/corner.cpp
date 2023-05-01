#include <string>

#include <board.hpp>
#include <tiles/corner.hpp>
#include <utils/color.hpp>

std::string Corner::get_detail() const {
  return Color::empty(Board::get_length_of_tile());
}