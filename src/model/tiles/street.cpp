#include "src/model/tiles/street.hpp"
#include "src/model/tiles/tile.hpp"

#include "src/model/board.hpp"

#include "src/utils/color.hpp"

#include <fmt/args.h>
#include <fmt/color.h>

#include <string>
#include <string_view>

Street::Street(const json &tile_data, const int id)
    : Property(tile_data, id),
      card{ create_card(tile_data) } {
  update_detail();
}

auto Street::create_card(const json &tile_data) -> std::string {
  constexpr int card_width = 33;
  const std::string empty_card_width = std::string(card_width, ' ');
  const auto card_pos = fmt::arg("POSITION", "\x1b[{1}G");
  const auto card_space = fmt::arg("EMPTY_ROW_CARD_WIDTH", empty_card_width);

  static const std::string color_banner =
      fmt::format(Color::get(get_group()), base_color_banner, card_pos, card_space);

  // Cannot be static for some reason? I think too many format() inside a format()
  const auto white_card_black_text = fmt::fg(fmt::color::black) | fmt::bg(fmt::color::white);
  const std::string card_details = fmt::format(
      white_card_black_text,
      base_card,
      card_space,
      card_pos,
      fmt::arg("INDENT", "  "),
      fmt::arg("NAME", tile_data["name"].get<std::string>()),
      fmt::arg("PROPERTY_COST", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("MORTGAGE_VALUE", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("RENT_BASIC", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("RENT_FULLSET", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("RENT_1HOUSE", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("RENT_2HOUSE", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("RENT_3HOUSE", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("RENT_4HOUSE", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("RENT_HOTEL", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("HOUSE_COST", fmt::format("${}", tile_data["cost"].get<int>())),
      fmt::arg("HOTEL_COST", fmt::format("${}", tile_data["cost"].get<int>()))
  );

  return fmt::format("{}{}{}", "\x1b[{0};{1}H", color_banner, card_details);
}

void Street::update_detail() {
  const std::string_view label = ownership_labels.at(get_ownership_status());
  const std::string detail = fmt::format(get_owner()->get_color(), label);
  const std::string cost = fmt::format("${}", get_cost());

  // Cost is max 4 digits (i.e. cost <= 9999) in order to fit in the board's visuals
  const int max_cost_length = Board::get_tile_length() - static_cast<int>(label.length());

  const std::string new_detail =
      fmt::format("{}{:>{LENGTH}}", detail, cost, fmt::arg("LENGTH", max_cost_length));

  set_detail(new_detail);
}