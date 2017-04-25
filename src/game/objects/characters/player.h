#ifndef __GAME_PLAYER_H__
#define __GAME_PLAYER_H__

#include "../object.h"

namespace Game {
    class Player : public Object {
    public:
        virtual char symbol() const override { return '@'; }
        virtual bool attackable() const override { return true; }
    };
}

#endif
