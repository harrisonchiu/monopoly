#ifndef UTILITY_HPP
#define UTILITY_HPP

#include <string>

#include <nlohmann/json.hpp>

#include <tiles/tile.hpp>

class Utility : public Property {
protected:
  constexpr std::string get_property_status_label() override;

public:
  Utility(const json &tile_data, const int id) : Property(tile_data, id) {}
  std::string get_detail() override;
};

#endif // UTILITY_HPP