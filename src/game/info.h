#ifndef __GAME_INFO_H__
#define __GAME_INFO_H__

namespace Game {
    enum class CharacterType {
        Human
    };

    struct Info {
        CharacterType race;
        int level;
    };
}

#endif
