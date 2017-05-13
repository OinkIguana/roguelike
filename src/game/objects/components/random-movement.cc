#include "random-movement.h"
#include "../../random.h"

namespace Game {
    std::uniform_int_distribution<int> rd(0, 3);

    Command RandomMovement::update(Command, Object&) {
        Direction dir = static_cast<Direction>(rd(rng));
        return Command{ CommandType::Move, dir };
    }
}
