#ifndef __GAME_EYES_COMPONENT_H__
#define __GAME_EYES_COMPONENT_H__

#include "../component.h"

namespace Game {
    class Eyes : public ObjectComponent {
        const bool _room;
    public:
        Eyes(bool = true);
        virtual Command update(Command, Object&) override;
    };
}

#endif
