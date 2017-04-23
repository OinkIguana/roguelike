#include "engine.h"
#include "update.h"
#include "command.h"
#include "view.h"
#include "info.h"
#include "map.h"

namespace Game {
    Engine::Engine(std::shared_ptr<View> view) : view{ view } {}

    int Engine::start() {
        // intialize new game
        info = std::make_shared<Info>();
        info->level = 0;

        // ignore everything until character class is chosen
        Command cmd;
        do {
            cmd = view->command();
            if(cmd.type == CommandType::CharacterSelect) {
                info->race = cmd.character;
            } else if(cmd.type == CommandType::Quit) { return 0; }
        } while(cmd.type != CommandType::CharacterSelect);

        // play the game
        do {
            if(!map) {
                // create new level
                map = std::make_shared<Map>(++info->level);
                view->update(Game::Update{ UpdateType::MapChange, map.get() });
            }
            // process player input
            cmd = view->command();

            // update the map

            // update the display
            view->redraw();
            map = nullptr;
        } while(true || cmd);

        // end the game

        return 0;
    }
}
