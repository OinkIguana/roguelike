#include "basic-attackable.h"

namespace Game {
    bool BasicAttackable::attackable(Object&, const Object&) const { return true; }
    void BasicAttackable::attack(Object& attacker, Object& my) {
        int damage = attacker.stats.atk;
        int reduction = my.stats.def;
        my.stats.hp -= std::max(0, damage - reduction);
        if(my.stats.hp <= 0) {
            my.cell()->clear();
        }
    }
}
