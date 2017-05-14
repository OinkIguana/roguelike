#include "basic-attackable.h"

namespace Game {
    bool BasicAttackable::attackable(Object&, const Object&) const { return true; }
    void BasicAttackable::attack(Object& attacker, Object& me) {
        int damage = attacker.stats.atk;
        int reduction = me.stats.def;
        me.stats.hp -= std::max(0, damage - reduction);
        if(me.stats.hp <= 0) {
            me.destroy();
        }
    }
}
