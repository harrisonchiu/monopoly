#include <string>

#include <fmt/color.h>

#include <board.hpp>
#include <tiles/tile.hpp>
#include <tiles/utility.hpp>

constexpr std::string Utility::get_property_status_label() {
  switch (property_status) {
  case PropertyStatus::Mortgaged:
    return fmt::format(owner, "M");
  case PropertyStatus::Unowned:
    return fmt::format(owner, "_");
  case PropertyStatus::Owned:
    return fmt::format(owner, "X");
  case PropertyStatus::Tier1:
    return fmt::format(owner, "2");
  case PropertyStatus::Tier2:
    return fmt::format(owner, "3");
  case PropertyStatus::Tier3:
    return fmt::format(owner, "4");
  case PropertyStatus::Tier4:
    return fmt::format(owner, "5");
  case PropertyStatus::Tier5:
    return fmt::format(owner, "6");
  default:
    return fmt::format(owner, "?");
  }
}

std::string Utility::get_detail() {
  const std::string label = get_property_status_label();
  const std::string cost = fmt::format("${}", property_cost);

  const int chars_fmt_color_added = 23;
  const int max_string_length =
      Board::get_length_of_tile() - (label.length() - chars_fmt_color_added);

  return fmt::format("{}{:>{LENGTH}}", get_property_status_label(), cost,
                     fmt::arg("LENGTH", max_string_length));
}