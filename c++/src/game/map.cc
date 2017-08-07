#include "map.h"
#include "cell.h"
#include "room.h"
#include "random.h"

#include <functional>
#include <cmath>
#include <algorithm>
#include <numeric>
#include <iostream>
#include <array>
#include <map>

// flood fills a continuous section of a grid[y][x]-like data structure with v
//  given a predicate
template<typename T>
void flood_fill(std::vector<std::vector<T>>& grid, unsigned int x, unsigned int y, T v, std::function<bool(int, int)> pred) {
    if(pred(x, y)) {
        grid[y][x] = v;
        if(y > 0)
            flood_fill(grid, x, y - 1, v, pred);
        if(y + 1 < grid.size())
            flood_fill(grid, x, y + 1, v, pred);
        if(x > 0)
            flood_fill(grid, x - 1, y, v, pred);
        if(x + 1 < grid[y].size())
            flood_fill(grid, x + 1, y, v, pred);
    }
}


const int MIN_ROOMS = 5;
const int MAX_ROOMS = 30;
const float GROWTH_FACTOR = 1.5f;

struct Rectangle { int x, y, w, h; };
struct Point { int x, y; };

namespace Game {
    bool is_dead_end(std::vector<std::vector<std::shared_ptr<Cell>>>& cells, unsigned int x, unsigned int y) {
        if(cells[y][x]->type != Cell::Type::Hall && cells[y][x]->type != Cell::Type::Door) return false;
        int count = 0;
        if(y == 0 || (cells[y-1][x]->type != Cell::Type::Hall && cells[y-1][x]->type != Cell::Type::Door && cells[y-1][x]->type != Cell::Type::Room)) {
            ++count;
        }
        if(x == 0 || (cells[y][x-1]->type != Cell::Type::Hall && cells[y][x-1]->type != Cell::Type::Door && cells[y][x-1]->type != Cell::Type::Room)) {
            ++count;
        }
        if(y == cells.size() - 1 || (cells[y+1][x]->type != Cell::Type::Hall && cells[y+1][x]->type != Cell::Type::Door && cells[y+1][x]->type != Cell::Type::Room)) {
            ++count;
        }
        if(x == cells[0].size() - 1 || (cells[y][x+1]->type != Cell::Type::Hall && cells[y][x+1]->type != Cell::Type::Door && cells[y][x+1]->type != Cell::Type::Room)) {
            ++count;
        }
        return count == 3;
    }

    bool is_fat_hallway(std::vector<std::vector<std::shared_ptr<Cell>>>& cells, unsigned int x, unsigned int y) {
        if(cells[y][x]->type != Cell::Type::Hall && cells[y][x]->type != Cell::Type::Door) return false;
        #define IS_A_ROOM(a) (a->type == Cell::Type::Hall || a->type == Cell::Type::Door || a->type == Cell::Type::Room)
        // this needs work
        Point sides[] = {
            { -1, -1 },
            { -1,  0 },
            { -1,  1 },
            {  0,  1 },
            {  1,  1 },
            {  1,  0 },
            {  1, -1 },
            {  0, -1 }
        };
        int start = 0;
        while(
            y + sides[start].y >= 0 &&
            y + sides[start].y < cells.size() &&
            x + sides[start].x >= 0 &&
            x + sides[start].x < cells[0].size() &&
            IS_A_ROOM(cells[y+sides[start].y][x+sides[start].x])
        ) {
            if(++start == 6) { return true; }
        }
        int cycle = 0;
        bool four = false;
        for(int i = 0; i < 8; ++i) {
            Point side = sides[(i + start) % 8];
            if(0 <= y + side.y && y + side.y < cells.size()) {
                if(0 <= x + side.x && x + side.x < cells[0].size()) {
                    if(IS_A_ROOM(cells[y+side.y][x+side.x])) {
                        ++cycle;
                        if(cycle == 6) {
                            return true;
                        } else if(cycle == 4) {
                            four = true;
                        }
                        continue;
                    }
                }
            }
            if(cycle && cycle < 4 && four) {
                return false;
            } else {
                cycle = 0;
            }
        }
        return four && (cycle == 0 || cycle >= 4);
        #undef IS_A_ROOM
    }

    // TODO these need to remove from rm_cells

    void remove_dead_end(std::vector<std::vector<std::shared_ptr<Cell>>>& cells, std::vector<std::vector<Point>>& rm_cells, unsigned int x, unsigned int y) {
        if(is_dead_end(cells, x, y)) {
            if(cells[y][x]->type == Cell::Type::Hall) {
                cells[y][x]->type = Cell::Type::Empty;
                for(auto& row : rm_cells) {
                    for(auto i = row.begin(); i != row.end(); ) {
                        if(i->x == static_cast<int>(x) && i->y == static_cast<int>(y)) {
                            i = row.erase(i);
                        } else {
                            ++i;
                        }
                    }
                }
                if(y != 0) {
                    remove_dead_end(cells, rm_cells, x, y - 1);
                }
                if(x != 0) {
                    remove_dead_end(cells, rm_cells, x - 1, y);
                }
                if(y != cells.size() - 1) {
                    remove_dead_end(cells, rm_cells, x, y + 1);
                }
                if(x != cells[0].size() - 1) {
                    remove_dead_end(cells, rm_cells, x + 1, y);
                }
            } else if(cells[y][x]->type == Cell::Type::Door) {
                if(y == 0 || y == cells.size() -1 || cells[y + 1][x]->type == Cell::Type::Room || cells[y - 1][x]->type == Cell::Type::Room) {
                    cells[y][x]->type = Cell::Type::WallH;
                } else if(x == 0 || x == cells[0].size() - 1 || cells[y][x + 1]->type == Cell::Type::Room || cells[y][x - 1]->type == Cell::Type::Room) {
                    cells[y][x]->type = Cell::Type::WallV;
                }
            }
        }
    }

    void remove_fat_hallways(std::vector<std::vector<std::shared_ptr<Cell>>>& cells, std::vector<std::vector<Point>>& rm_cells, unsigned int x, unsigned int y) {
        if(is_fat_hallway(cells, x, y)) {
            if(cells[y][x]->type == Cell::Type::Hall) {
                cells[y][x]->type = Cell::Type::Empty;
                for(auto& row : rm_cells) {
                    for(auto i = row.begin(); i != row.end(); ) {
                        if(i->x == static_cast<int>(x) && i->y == static_cast<int>(y)) {
                            i = row.erase(i);
                        } else {
                            ++i;
                        }
                    }
                }
            } else if(cells[y][x]->type == Cell::Type::Door) {
                if(y == 0 || y == cells.size() - 1 || cells[y + 1][x]->type == Cell::Type::Room || cells[y - 1][x]->type == Cell::Type::Room) {
                    cells[y][x]->type = Cell::Type::WallH;
                } else if(x == 0 || x == cells[0].size() - 1 || cells[y][x + 1]->type == Cell::Type::Room || cells[y][x - 1]->type == Cell::Type::Room) {
                    cells[y][x]->type = Cell::Type::WallV;
                }
            }
            if(y > 0) {
                remove_fat_hallways(cells, rm_cells, x, y - 1);
            }
            if(x > 0) {
                remove_fat_hallways(cells, rm_cells, x - 1, y);
            }
            if(y < cells.size() - 1) {
                remove_fat_hallways(cells, rm_cells, x, y + 1);
            }
            if(x < cells[0].size() - 1) {
                remove_fat_hallways(cells, rm_cells, x + 1, y);
            }
        }
    }

    Map::Map(int level) : _floor{level} {
        rng.seed(level);

        const int rm_count = std::min(MIN_ROOMS + level / 3, MAX_ROOMS);
        const int max_merges = rm_count / 2;
        int merges = 0;
        h = 20 + GROWTH_FACTOR * level;
        w = std::round(1.618 * h * 2);

        // create all the cells
        cells = std::vector<std::vector<std::shared_ptr<Cell>>>(h, std::vector<std::shared_ptr<Cell>>(w));
        for(unsigned int y = 0; y < cells.size(); ++y) {
            for(unsigned int x = 0; x < cells[y].size(); ++x) {
                cells[y][x] = std::make_shared<Cell>(x, y, *this);
            }
        }

        // distributions for room generation
        std::uniform_int_distribution<int> rx(0, w - 1);
        std::uniform_int_distribution<int> ry(0, h - 1);
        std::normal_distribution<float> rw(16, 1);
        std::normal_distribution<float> rh(6, 1);
        std::bernoulli_distribution rcoll(0.9);

        // generate rooms
        auto rm_boxes = std::vector<Rectangle>();
        for(int i = 0; i < rm_count; ++i) {
            int xx, yy, ww, hh;
            int collides = 0;
            do {
                xx = rx(rng);
                yy = ry(rng);
                ww = std::ceil(rw(rng)) + 2;
                hh = std::ceil(rh(rng)) + 2;
                // only allow collisions sometimes
                collides = std::count_if(rm_boxes.begin(), rm_boxes.end(), [&xx, &yy, &ww, &hh] (Rectangle rm) {
                    return
                        rm.x + rm.w >= xx && rm.x < xx + ww &&
                        rm.y + rm.h >= yy && rm.y < yy + hh;
                });
                bool allow_merge = merges + collides <= max_merges && !rcoll(rng);
                bool out_of_bounds = xx + ww >= w || yy + hh > h;
                if(collides && allow_merge && !out_of_bounds) {
                    merges += collides;
                }
                collides = (collides && !allow_merge) || out_of_bounds;
            } while(collides);
            rm_boxes.emplace_back(Rectangle{ xx, yy, ww, hh });
        }

        // set cell types
        for(auto box : rm_boxes) {
            for(int y = box.y; y < box.y + box.h; ++y) {
                for(int x = box.x; x < box.x + box.w; ++x) {
                    if(cells[y][x]->type != Cell::Type::Room) {
                        if((x == box.x || x == box.x + box.w - 1) && (y == box.y || y == box.y + box.h - 1)) {
                            cells[y][x]->type = Cell::Type::Corner;
                        } else if(x == box.x || x == box.x + box.w - 1) {
                            if(cells[y][x]->type == Cell::Type::WallH || cells[y][x]->type == Cell::Type::Corner)
                                cells[y][x]->type = Cell::Type::Corner;
                            else
                                cells[y][x]->type = Cell::Type::WallV;
                        } else if(y == box.y || y == box.y + box.h - 1) {
                            if(cells[y][x]->type == Cell::Type::WallV || cells[y][x]->type == Cell::Type::Corner)
                                cells[y][x]->type = Cell::Type::Corner;
                            else
                                cells[y][x]->type = Cell::Type::WallH;
                        } else {
                            cells[y][x]->type = Cell::Type::Room;
                        }
                    }
                }
            }
        }

        // build cell graph
        std::vector<std::vector<int>> cl_graph(h, std::vector<int>(w, 0));
        int r = 0;
        for(int y = 0; y < h; ++y) {
            for(int x = 0; x < w; ++x) {
                if(cl_graph[y][x] == 0 && cells[y][x]->type == Cell::Type::Room) {
                    flood_fill(cl_graph, x, y, ++r, [&cl_graph, this] (int x, int y) { return cl_graph[y][x] == 0 && cells[y][x]->type == Cell::Type::Room; });
                }
            }
        }

        // figure out which cells are in each room
        std::vector<std::vector<Point>> rm_cells(r);
        for(int y = 0; y < h; ++y) {
            for(int x = 0; x < w; ++x) {
                if(cl_graph[y][x] != 0) {
                    rm_cells[cl_graph[y][x] - 1].emplace_back(Point{ x, y });
                }
            }
        }

        // add hallways
        std::vector<int> connected(r);
        for(int i = 0; i < r; ++i) { connected[i] = i + 1; }
        std::vector<std::array<int, 4>> tunnels(r, std::array<int, 4>{ 0, 0, 0, 0 });
        std::uniform_int_distribution<int> rd(0, 3);
        std::binomial_distribution<bool> rcont(rm_count / MAX_ROOMS / 4);
        std::map<int, int> tunnel_merges;
        std::function<int(int)> compute_tunnel_index = [&tunnel_merges, &compute_tunnel_index] (int r) {
            const int t = tunnel_merges[r];
            return t == 0 || t == r ? r : compute_tunnel_index(t);
        };
        while(!all_of(connected.begin(), connected.end(), [](int is){ return is == 1; }) || rcont(rng)) {
            ++r;
            std::uniform_int_distribution<int> rr(0, rm_cells.size() - 1);
            int xx, yy, rm, dir;
            do {
                // NOTE: if you're unlucky this will go on for a long time...
                dir = rd(rng);
                do {
                    rm = rr(rng);
                } while(rm_cells[rm].empty());
                std::uniform_int_distribution<int> rc(0, rm_cells[rm].size() - 1);
                int p = rc(rng);
                xx = rm_cells[rm][p].x;
                yy = rm_cells[rm][p].y;
            } while(!(
                cl_graph[yy][xx] != 0 && // in a room
                tunnels[cl_graph[yy][xx] - 1][dir] == 0 && // not already a tunnel in this direction
                (cells[yy][xx]->type == Cell::Type::Room || cells[yy][xx]->type == Cell::Type::Hall) // actually in the room part
            ));

            tunnels.emplace_back(std::array<int, 4>{ !(dir % 2), dir % 2, !(dir % 2), dir % 2 });
            rm_cells.emplace_back(std::vector<Point>());

            connected.emplace_back(connected[rm]);
            ++tunnels[rm][dir];
            if(cells[yy][xx]->type == Cell::Type::Hall) {
                tunnel_merges[compute_tunnel_index(cl_graph[yy][xx])] = compute_tunnel_index(r);
            }
            while(xx >= 0 && xx < w && yy >= 0 && yy < h) {
                switch(cells[yy][xx]->type) {
                case Cell::Type::Empty:
                    cells[yy][xx]->type = Cell::Type::Hall;
                    cl_graph[yy][xx] = r;
                    rm_cells.back().emplace_back(Point{ xx, yy });
                    break;
                case Cell::Type::WallH:
                case Cell::Type::WallV:
                    cells[yy][xx]->type = Cell::Type::Door;
                    rm_cells.back().emplace_back(Point{ xx, yy });
                    break;
                case Cell::Type::Hall:
                    if(cl_graph[yy][xx] - 1 != rm) {
                        tunnel_merges[compute_tunnel_index(cl_graph[yy][xx])] = compute_tunnel_index(r);
                        rm_cells.back().emplace_back(Point{ xx, yy });
                    }
                case Cell::Type::Room:
                    if(cl_graph[yy][xx] - 1 != rm) {
                        int low = std::min(connected[cl_graph[yy][xx] - 1], connected[rm]);
                        int hi = std::max(connected[cl_graph[yy][xx] - 1], connected[rm]);
                        std::replace(connected.begin(), connected.end(), hi, low);
                        ++tunnels[cl_graph[yy][xx] - 1][(dir + 2) % 4];
                        goto quit;
                    }
                    break;
                default:
                    goto quit;
                }
                switch(dir) {
                    case 0: xx += 1; break;
                    case 1: yy += 1; break;
                    case 2: xx -= 1; break;
                    case 3: yy -= 1; break;
                }
            }
        quit:;
        }

        // remove dead ends/fat hallways
        for(int y = 0; y < h; ++y) {
            for(int x = 0; x < w; ++x) {
                if(is_fat_hallway(cells, x, y)) {
                    remove_fat_hallways(cells, rm_cells, x, y);
                }
            }
        }
        for(int y = 0; y < h; ++y) {
            for(int x = 0; x < w; ++x) {
                if(is_dead_end(cells, x, y)) {
                    remove_dead_end(cells, rm_cells, x, y);
                }
            }
        }

        // perform the tunnel merges
        for(unsigned int i = 0; i < rm_cells.size(); ++i) {
            const unsigned int actual = compute_tunnel_index(i+1) - 1;
            if(actual != i) {
                auto & old = rm_cells[i];
                rm_cells[actual].reserve(rm_cells[actual].size() + old.size());
                rm_cells[actual].insert(rm_cells[actual].end(), old.begin(), old.end());
                old.clear();
            }
        }

        // add walls/doors/corners to their rooms
        for(int y = 0; y < h; ++y) {
            for(int x = 0; x < w; ++x) {
                switch(cells[y][x]->type) {
                case Cell::Type::WallH:
                    if(y != 0 && cells[y-1][x]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y-1][x]-1].emplace_back(Point{ x, y });
                    }
                    if(y != h - 1 && cells[y+1][x]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y+1][x]-1].emplace_back(Point{ x, y });
                    }
                    break;
                case Cell::Type::WallV:
                    if(x != 0 && cells[y][x-1]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y][x-1]-1].emplace_back(Point{ x, y });
                    }
                    if(x != w - 1 && cells[y][x+1]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y][x+1]-1].emplace_back(Point{ x, y });
                    }
                    break;
                case Cell::Type::Corner:
                    if(x != 0 && y != 0 && cells[y-1][x-1]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y-1][x-1]-1].emplace_back(Point{ x, y });
                    }
                    if(x != 0 && y != h - 1 && cells[y+1][x-1]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y+1][x-1]-1].emplace_back(Point{ x, y });
                    }
                    if(x != w - 1 && y != 0 && cells[y-1][x+1]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y-1][x+1]-1].emplace_back(Point{ x, y });
                    }
                    if(x != w - 1 && y != h - 1 && cells[y+1][x+1]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y+1][x+1]-1].emplace_back(Point{ x, y });
                    }
                    break;
                case Cell::Type::Door:
                    if(y != 0 && cells[y-1][x]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y-1][x]-1].emplace_back(Point{ x, y });
                    }
                    if(y != h - 1 && cells[y+1][x]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y+1][x]-1].emplace_back(Point{ x, y });
                    }
                    if(x != 0 && cells[y][x-1]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y][x-1]-1].emplace_back(Point{ x, y });
                    }
                    if(x != w - 1 && cells[y][x+1]->type == Cell::Type::Room) {
                        rm_cells[cl_graph[y][x+1]-1].emplace_back(Point{ x, y });
                    }
                default: break;
                }
            }
        }

        // assign Cells to Rooms
        rooms = std::vector<std::shared_ptr<Room>>();
        for(auto points : rm_cells) {
            std::vector<std::shared_ptr<Cell>> cls;
            for(Point p : points) {
                if(cells[p.y][p.x]->type != Cell::Type::Empty)
                    cls.emplace_back(cells[p.y][p.x]);
            }
            if(cls.size() > 1)
                rooms.emplace_back(std::make_shared<Room>(cls));
        }
    }

    std::vector<std::string> Map::to_strings() const {
        std::vector<std::string> strings(cells.size());
        std::transform(cells.begin(), cells.end(), strings.begin(), [](auto row) {
            return std::accumulate(row.begin(), row.end(), std::string(), [](std::string a, auto cell) {
                return a + cell->symbol();
            });
        });
        return strings;
    }

    std::vector<std::string> Map::object_strings() const {
        std::vector<std::string> strings(cells.size());
        std::transform(cells.begin(), cells.end(), strings.begin(), [](auto row) {
            return std::accumulate(row.begin(), row.end(), std::string(), [](std::string a, auto cell) {
                return a + (cell->visible && cell->contents ? cell->contents->symbol : ' ');
            });
        });
        return strings;
    }

    std::shared_ptr<Room> Map::choose_room() const {
        std::shared_ptr<Room> rm;
        std::uniform_int_distribution<int> rr(0, rooms.size() - 1);
        do {
            rm = rooms[rr(rng)];
        } while(rm->full());
        return rm;
    }

    std::shared_ptr<Cell> Map::cell_at(int x, int y) const {
        return cells[y][x];
    }

    Map::Iterator Map::begin() {
        return Iterator(0, 0, *this);
    }

    Map::Iterator Map::end() {
        return Iterator(0, h, *this);
    }

    Map::Iterator::Iterator(int x, int y, Map & map) : x{x}, y{y}, map{map} {}
    Map::Iterator Map::Iterator::operator++() {
        ++x;
        if(x == map.w) {
            x = 0;
            ++y;
        }
        return *this;
    }

    bool Map::Iterator::operator!=(Map::Iterator& o) {
        return !(x == o.x && y == o.y && &map == &o.map);
    }

    Cell& Map::Iterator::operator*() {
        return *map.cells[y][x];
    }

    std::shared_ptr<Object> Map::add(std::shared_ptr<Object> obj) {
        std::shared_ptr<Room> rm = choose_room();
        auto cell = rm->choose_cell();
        cell->set_contents(obj);

        return obj;
    }

    std::shared_ptr<Object> Map::add_avoiding(std::shared_ptr<Object> avoid, std::shared_ptr<Object> obj) {
        std::shared_ptr<Room> rm;
        do {
            rm = choose_room();
        } while(rm->contains(avoid));

        auto cell = rm->choose_cell();
        cell->set_contents(obj);

        return obj;
    }
}
