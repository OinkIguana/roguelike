#ifndef __GAME_RANDOM_MOVEMENT_COMPONENT_H__
#define __GAME_RANDOM_MOVEMENT_COMPONENT_H__

#include "../component.h"

namespace Game {
    class RandomMovement : public ObjectComponent {
    public:
        virtual Command update(Command, Object&) override;
    };
}

#endif
