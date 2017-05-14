#ifndef __GAME_COIN_PICKUP_COMPONENT_H__
#define __GAME_COIN_PICKUP_COMPONENT_H__

#include "../component.h"

namespace Game {
    class CoinPickup : public ObjectComponent {
        const int _value;
    public:
        CoinPickup(int);
        virtual bool collectable(Object&, const Object&) const override;
        virtual void collect(Object&, Object&) override;
    };
}

#endif
