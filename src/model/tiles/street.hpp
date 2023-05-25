#ifndef STREET_HPP
#define STREET_HPP

#include "src/model/tiles/tile.hpp"

#include "src/model/tiles/attributes.hpp"
#include "src/utils/map.hpp"

#include <nlohmann/json.hpp>

#include <array>
#include <string_view>

class Street : public Property {
  using json = nlohmann::json;
  using status_labels = std::pair<OwnershipStatus, std::string_view>;

private:
  static constexpr std::size_t status_count = static_cast<std::size_t>(OwnershipStatus::COUNT);
  static constexpr std::array<status_labels, status_count> labels = { {
      { OwnershipStatus::Mortgaged, "M" },
      { OwnershipStatus::Unowned, "_" },
      { OwnershipStatus::Owned, "X" },
      { OwnershipStatus::Tier1, "1H" },
      { OwnershipStatus::Tier2, "2H" },
      { OwnershipStatus::Tier3, "3H" },
      { OwnershipStatus::Tier4, "4H" },
      { OwnershipStatus::Tier5, "HT" },
  } };
  static constexpr auto ownership_labels =
      CompileTimeMap<OwnershipStatus, std::string_view, labels.size()>{ { labels } };

  // Card must be separated into 2: color_banner and the details section
  // They have different colors. If they are put as one, fmt escapes (\x1b[0m) the
  // textstyle at the end of the arg, but that cancels any and all colors.
  // Impossible to have 2 different colors (1 all encompassing, 1 as an arg) if styling
  // using fmt::format(text_style, str, args)
  static constexpr std::string_view base_color_banner{ R"""(
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{EMPTY_ROW_CARD_WIDTH})""" }; // No newline to keep this together with @base_card

  // Hard-coded values because its too hard to dynamically calculate width of different phrases
  // Values assume that `INDENT` is 2 spaces and card width is 33 chars wide
  static constexpr std::string_view base_card{ R"""(
{POSITION}{NAME:^33}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{INDENT}Property cost: {PROPERTY_COST: >14}{INDENT}
{POSITION}{INDENT}Mortgage value: {MORTGAGE_VALUE: >13}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{INDENT}Basic rent: {RENT_BASIC: >17}{INDENT}
{POSITION}{INDENT}Rent with full set: {RENT_FULLSET: >9}{INDENT}
{POSITION}{INDENT}Rent with 1 house: {RENT_1HOUSE: >10}{INDENT}
{POSITION}{INDENT}Rent with 2 houses: {RENT_2HOUSE: >9}{INDENT}
{POSITION}{INDENT}Rent with 3 houses: {RENT_3HOUSE: >9}{INDENT}
{POSITION}{INDENT}Rent with 4 houses: {RENT_4HOUSE: >9}{INDENT}
{POSITION}{INDENT}Rent with hotel: {RENT_HOTEL: >12}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{INDENT}House cost: {HOUSE_COST: >17}{INDENT}
{POSITION}{INDENT}Hotel cost: {HOTEL_COST: >17}{INDENT}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
{POSITION}{EMPTY_ROW_CARD_WIDTH}
)""" };

  std::string card;

  auto create_card(const json &tile_data) -> std::string;

public:
  Street(const json &tile_data, int id);
  void update_detail() override;

  auto get_card() const -> std::string_view override { return card; };
};

#endif // STREET_HPP