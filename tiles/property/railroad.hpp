#ifndef RAILROAD_HPP
#define RAILROAD_HPP

#include <string>

#include <nlohmann/json.hpp>

#include <tiles/tile.hpp>

class Railroad : public Property {
protected:
  constexpr std::string get_property_status_label() override;

public:
  Railroad(const json &tile_data, const int id) : Property(tile_data, id) {}
  std::string get_detail() override;
};

#endif // RAILROAD_HPP