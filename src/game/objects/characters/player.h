#ifndef __GAME_PLAYER_H__
#define __GAME_PLAYER_H__

#include "../object.h"

namespace Game {
    class Player : public Object {
    public:
        Player();
        virtual char symbol() const override;
        virtual bool attackable(std::shared_ptr<Object>) const override;
    };
}

#endif
