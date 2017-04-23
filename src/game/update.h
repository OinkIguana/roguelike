#ifndef __GAME_UPDATE_H__
#define __GAME_UPDATE_H__

#include <memory>
#include "map.h"

namespace Game {
    enum class UpdateType {
        MapChange
    };
    struct Update {
        UpdateType type;
        union {
            Map* map;
        };
    };
}

#endif
