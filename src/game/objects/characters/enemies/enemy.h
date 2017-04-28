#ifndef __GAME_ENEMY_H__
#define __GAME_ENEMY_H__

#include "../../object.h"

namespace Game {
    class Enemy : public Object {
    public:
        virtual bool attackable(std::shared_ptr<Object>) const override;
        virtual void attack(std::shared_ptr<Object>) override;
        virtual Command update(Command) override;
    };
}

#endif
