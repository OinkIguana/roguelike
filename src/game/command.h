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
    enum class Direction {
        Up, Down, Left, Right
    };

    struct Command {
        CommandType type;
        union _ {
            _(CharacterType character) : character{ character } {}
            _(Direction direction) : direction{ direction } {}
            _() {}
            CharacterType character;
            Direction direction;
        } data;
        inline operator bool () const noexcept {
            return type != CommandType::Quit;
        }
    };
}


#endif
