#include <ncurses.h>
#include <locale.h>
#include "default.h"
#include "../game/info.h"

namespace View {
    Default::Default() {
        setlocale(LC_ALL, "");
        initscr();
        noecho();
        keypad(stdscr, TRUE);
        start_color();
        init_pair(1, COLOR_BLACK, COLOR_WHITE);
        curs_set(0);
        ESCDELAY = 0;
    }

    Default::~Default() {
        endwin();
    }

    void Default::update(Game::Update update) {
        switch(update.type) {
        case Game::UpdateType::MapChange:
            map = update.map->to_string();
            break;
        }
    }

    Game::Command Default::character_select() {
        getch();
        state = [this] () { return world_view(); };
        return { Game::CommandType::CharacterSelect, Game::CharacterType::Human };
    }

    Game::Command Default::world_view() {
        getch();
        return { Game::CommandType::Quit };
    }

    Game::Command Default::command() {
        return state();
    }
    void Default::redraw() {
        refresh();
    }
}
