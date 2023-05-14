#include "src/model/tiles/street.hpp"
#include "src/model/tiles/tile.hpp"

#include "src/model/board.hpp"

#include <fmt/core.h>

#include <string>

Street::Street(const json &tile_data, const int id) : Property(tile_data, id) { update_detail(); }

// TODO: this could be done as a compile time map
auto Street::get_property_status_label() const -> std::string {
  switch (Property::get_property_status()) {
  case PropertyStatus::Mortgaged:
    return fmt::format(Property::get_owner()->color, "M");
  case PropertyStatus::Unowned:
    return fmt::format(Property::get_owner()->color, "_");
  case PropertyStatus::Owned:
    return fmt::format(Property::get_owner()->color, "X");
  case PropertyStatus::Tier1:
    return fmt::format(Property::get_owner()->color, "1H");
  case PropertyStatus::Tier2:
    return fmt::format(Property::get_owner()->color, "2H");
  case PropertyStatus::Tier3:
    return fmt::format(Property::get_owner()->color, "3H");
  case PropertyStatus::Tier4:
    return fmt::format(Property::get_owner()->color, "4H");
  case PropertyStatus::Tier5:
    return fmt::format(Property::get_owner()->color, "HT");
  default:
    return fmt::format(Property::get_owner()->color, "?");
  }
}

void Street::update_detail() {
  const std::string property_status_label = get_property_status_label();
  const std::string cost = fmt::format("${}", Property::get_property_cost());

  constexpr int chars_fmt_color_added = 23;
  const int max_string_length =
      Board::get_tile_length() -
      (static_cast<int>(property_status_label.length()) - chars_fmt_color_added);

  const std::string new_detail = fmt::format(
      "{}{:>{LENGTH}}", property_status_label, cost, fmt::arg("LENGTH", max_string_length)
  );

  Tile::set_detail(new_detail);
}