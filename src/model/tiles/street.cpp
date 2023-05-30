#include "src/model/tiles/street.hpp"
#include "src/model/tiles/tile.hpp"

#include "src/model/board.hpp"

#include <fmt/args.h>
#include <fmt/color.h>

#include <ranges>
#include <string>
#include <string_view>
#include <vector>

Street::Street(const json &tile_data, const int id)
    : Property(tile_data, id),
      card{ create_card(tile_data) },
      rent{ tile_data["rent"].get<std::vector<int>>() } {
  update_detail();
  update_effect();
}

auto Street::create_card(const json &tile_data) -> std::string {
  constexpr int card_width = 33;
  constexpr const Position &pos = Board::get_center_pos(); // cards will always be shown here
  const auto card_start = fmt::format("\x1b[{0};{1}H", pos.row, pos.col);
  const auto card_pos = fmt::arg("POSITION", fmt::format("\x1b[{}G", pos.col));

  const std::string color_row = fmt::format(
      get_color().has_background() ? get_color() : fmt::bg(fmt::color::white),
      base_color_row,
      card_pos,
      fmt::arg("EMPTY_ROW_CARD_WIDTH", std::string(card_width, ' '))
  );

  const std::string card_details = fmt::format(
      fmt::fg(fmt::color::black) | fmt::bg(fmt::color::white),
      base_card,
      card_pos,
      fmt::arg("INDENT", "  "),
      fmt::arg("EMPTY_ROW_CARD_WIDTH", std::string(card_width, ' ')),
      fmt::arg("NAME", tile_data["name"].get<std::string>()),
      fmt::arg("TYPE", tile_data["type"].get<std::string>()),
      fmt::arg("TILE_COST", fmt::format("${}", tile_data["cost"].get<int>())),
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

  return fmt::format("{0}{1}{1}{1}{2}", card_start, color_row, card_details);
}

void Street::update_detail() {
  const std::string_view label = ownership_labels.at(get_ownership_status());
  const std::string detail = fmt::format(get_owner()->get_color(), label);
  const std::string cost = fmt::format("${}", get_cost());

  // Cost is max 4 digits (i.e. cost <= 9999) in order to fit in the board's visuals
  const int max_cost_length = Board::get_tile_length() - static_cast<int>(label.length());

  const std::string new_detail = fmt::format("{0}{1:>{2}}", detail, cost, max_cost_length);

  set_detail(new_detail);
}

void Street::update_effect() {
  if (rent.empty()) {
    return;
  }

  switch (get_ownership_status()) {
  case OwnershipStatus::Mortgaged:
  case OwnershipStatus::Unowned:
    current_rent = 0;
    break;
  case OwnershipStatus::Owned:
    current_rent = rent.at(0);
    break;
  case OwnershipStatus::Tier1:
    current_rent = (1 < rent.size()) ? rent.at(1) : rent.back();
    break;
  case OwnershipStatus::Tier2:
    current_rent = (2 < rent.size()) ? rent.at(2) : rent.back();
    break;
  case OwnershipStatus::Tier3:
    current_rent = (3 < rent.size()) ? rent.at(3) : rent.back();
    break;
  case OwnershipStatus::Tier4:
    current_rent = (4 < rent.size()) ? rent.at(4) : rent.back();
    break;
  case OwnershipStatus::Tier5: // NOLINTNEXTLINE
    current_rent = (5 < rent.size()) ? rent.at(5) : rent.back();
    break;
  default:
    current_rent = 0;
    break;
  }

  effect = Effect{ Action::Rent, current_rent };
}