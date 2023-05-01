#include <string>

#include <fmt/color.h>

#include <board.hpp>
#include <tiles/street.hpp>
#include <tiles/tile.hpp>

std::string Street::get_property_status_label() const {
  switch (property_status) {
  case PropertyStatus::Mortgaged:
    return fmt::format(owner->color, "M");
  case PropertyStatus::Unowned:
    return fmt::format(owner->color, "_");
  case PropertyStatus::Owned:
    return fmt::format(owner->color, "X");
  case PropertyStatus::Tier1:
    return fmt::format(owner->color, "1H");
  case PropertyStatus::Tier2:
    return fmt::format(owner->color, "2H");
  case PropertyStatus::Tier3:
    return fmt::format(owner->color, "3H");
  case PropertyStatus::Tier4:
    return fmt::format(owner->color, "4H");
  case PropertyStatus::Tier5:
    return fmt::format(owner->color, "HT");
  default:
    return fmt::format(owner->color, "?");
  }
}

std::string Street::get_detail() const {
  const std::string label = get_property_status_label();
  const std::string cost = fmt::format("${}", property_cost);

  const int chars_fmt_color_added = 23;
  const int max_string_length =
      Board::get_length_of_tile() - (label.length() - chars_fmt_color_added);

  return fmt::format("{}{:>{LENGTH}}", get_property_status_label(), cost,
                     fmt::arg("LENGTH", max_string_length));
}