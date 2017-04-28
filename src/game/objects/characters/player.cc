#include "player.h"

namespace Game {
    Player::Player() {
        stats.atk = 25;
        stats.def = 25;
    }
    char Player::symbol() const { return '@'; }
    bool Player::attackable(std::shared_ptr<Object>) const { return true; }
}
