#include "coin-pickup.h"

namespace Game {
    CoinPickup::CoinPickup(int value) : _value{value} {}
    bool CoinPickup::collectable(Object&, const Object&) const { return true; }
    void CoinPickup::collect(Object& o, Object&) {
        o.inventory.coins += _value;
    };
}
