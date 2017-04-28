#ifndef __GAME_GOBLIN_H__
#define __GAME_GOBLIN_H__

#include "enemy.h"

namespace Game {
    class Goblin : public Enemy {
    public:
        virtual char symbol() const override { return 'G'; }
    };
}

#endif
