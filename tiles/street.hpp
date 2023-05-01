#ifndef STREET_HPP
#define STREET_HPP

#include <string>

#include <nlohmann/json.hpp>

#include <tiles/tile.hpp>

class Street : public Property {
  using json = nlohmann::json;

protected:
  std::string get_property_status_label() const override;

public:
  Street(const json &tile_data, const int id) : Property(tile_data, id) {}
  std::string get_detail() const override;
};

#endif // STREET_HPP