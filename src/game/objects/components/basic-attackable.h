#ifndef __GAME_BASIC_ATTACKABLE_COMPONENT_H__
#define __GAME_BASIC_ATTACKABLE_COMPONENT_H__

#include "../component.h"

namespace Game {
    class BasicAttackable : public ObjectComponent {
    public:
        virtual bool attackable(Object&, const Object&) const override;
        virtual void attack(Object&, Object&) override;
    };
}

#endif
