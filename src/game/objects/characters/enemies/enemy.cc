#include "enemy.h"
#include "../../../cell.h"
#include "../../../command.h"
#include "../../../random.h"

namespace Game {
    std::uniform_int_distribution<int> rd(0, 3);

    bool Enemy::attackable(std::shared_ptr<Object>) const { return true; }
    void Enemy::attack(std::shared_ptr<Object> attacker) {
        int damage = attacker->stats.atk;
        int reduction = stats.def;
        stats.hp -= std::max(0, damage - reduction);
        if(stats.hp <= 0) {
            cell()->clear();
        }
    }
    Command Enemy::update(Command) {
        Direction dir = static_cast<Direction>(rd(rng));
        return Command{ CommandType::Move, dir };
    }
}
