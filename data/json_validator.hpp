#ifndef JSON_VALIDATOR_HPP
#define JSON_VALIDATOR_HPP

#include <nlohmann/json.hpp>

namespace checker {

using json = nlohmann::json;

// Each check returns log error messages. Empty string for no errors.
auto check_size(const json &board) -> std::string;

auto check_name(const json &tile) -> std::string;

auto check_display_name(const json &tile) -> std::string;

auto check_type(const json &tile) -> std::string;

auto check_group(const json &tile) -> std::string;

auto check_cost(const json &tile) -> std::string;

} // namespace checker

namespace validation {

using json = nlohmann::json;

auto validate_board_json(const nlohmann::json &json_data) -> std::vector<std::string>;

} // namespace validation

#endif // JSON_VALIDATOR_HPP