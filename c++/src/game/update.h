#ifndef __GAME_UPDATE_H__
#define __GAME_UPDATE_H__

#include <memory>
#include "map.h"

namespace Game {
    enum class UpdateType {
        MapChange, PlayerChange, TargetChange, MessageChange
    };
    struct Update {
        UpdateType type;
        union {
            void* map;
            void* object;
            char* message;
        };
    };
}

#endif
