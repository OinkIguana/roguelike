#include "eyes.h"

namespace Game {
    Eyes::Eyes(bool room) : _room{room} {}
    Command Eyes::update(Command cmd, Object& me) {
        std::shared_ptr<Cell> cell;
        if(cmd.type == CommandType::Move) {
            switch(cmd.data.direction) {
            case Direction::Up:
                cell = me.cell()->north();
                break;
            case Direction::Right:
                cell = me.cell()->east();
                break;
            case Direction::Down:
                cell = me.cell()->south();
                break;
            case Direction::Left:
                cell = me.cell()->west();
                break;
            }
        }
        if(cell->available(me, true, true, true, true)) { cell->set_visible(true, _room); }
        return { CommandType::Idle };
    }
}
