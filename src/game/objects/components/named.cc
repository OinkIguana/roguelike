#include "named.h"

namespace Game {
    Named::Named(std::string name) : _name{name} {}
    std::string Named::name() const { return _name; }
}
