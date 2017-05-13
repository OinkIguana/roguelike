#ifndef __GAME_KEYBOARD_INPUT_COMPONENT_H__
#define __GAME_KEYBOARD_INPUT_COMPONENT_H__

#include "../component.h"

namespace Game {
    class KeyboardInput : public ObjectComponent {
    public:
        virtual Command update(Command, Object&) override;
    };
}

#endif
