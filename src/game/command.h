#ifndef __GAME_COMMAND_H__
#define __GAME_COMMAND_H__

namespace Game {
    enum class CommandType {
        CharacterSelect,
        Move,
        Interact,
        Item,
        Attack,
        Quit
    };

    enum class CharacterType;
    enum class Direction;

    struct Command {
        CommandType type;
        union {
            CharacterType character;
            Direction direction;
        };
        inline operator bool () const noexcept {
            return type != CommandType::Quit;
        }
    };
}


#endif
