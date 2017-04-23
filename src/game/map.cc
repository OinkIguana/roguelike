#include "map.h"
#include "cell.h"
#include "room.h"

#include <functional>
#include <cmath>
#include <algorithm>
#include <random>
#include <numeric>
#include <iostream>
#include <array>

std::default_random_engine rng;

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

namespace Game {
    Map::Map(int level) {
        rng.seed(level);
        struct Rectangle { int x, y, w, h; };
        struct Point { int x, y; };

        const int rm_count = std::min(MIN_ROOMS + level / 3, MAX_ROOMS);
        const int max_merges = rm_count / 2;
        int merges = 0;
        h = 20 + GROWTH_FACTOR * level;
        w = std::round(1.618 * h * 2);

        // create all the cells
        cells = std::vector<std::vector<std::shared_ptr<Cell>>>(h, std::vector<std::shared_ptr<Cell>>(w));
        for(unsigned int y = 0; y < cells.size(); ++y) {
            for(unsigned int x = 0; x < cells[y].size(); ++x) {
                cells[y][x] = std::make_shared<Cell>(x, y);
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
            bool collides = false;
            do {
                xx = rx(rng);
                yy = ry(rng);
                ww = ceil(rw(rng)) + 2;
                hh = ceil(rh(rng)) + 2;
                // only allow collisions sometimes
                bool allow_merge = merges < max_merges && !rcoll(rng);
                collides = any_of(rm_boxes.begin(), rm_boxes.end(), [&xx, &yy, &ww, &hh] (Rectangle rm) {
                    return
                        rm.x + rm.w >= xx && rm.x < xx + ww &&
                        rm.y + rm.h >= yy && rm.y < yy + hh;
                });
                bool out_of_bounds = xx + ww >= w || yy + hh > h;
                if(collides && allow_merge && !out_of_bounds) {
                    collides = false;
                    ++merges;
                }
                collides = (collides && !allow_merge) || out_of_bounds;
            } while(collides);
            rm_boxes.emplace_back(Rectangle{ xx, yy, ww, hh });
        }

        // set cell types
        for(auto box : rm_boxes) {
            for(int y = box.y; y < box.y + box.h; ++y) {
                for(int x = box.x; x < box.x + box.w; ++x) {
                    if(cells[y][x]->type != Cell::Type::Room && cells[y][x]->type != Cell::Type::Room) {
                        if((x == box.x || x == box.x + box.w - 1) && (y == box.y || y == box.y + box.h - 1)) {
                            cells[y][x]->type = Cell::Type::Corner;
                        } else if(x == box.x || x == box.x + box.w - 1) {
                            cells[y][x]->type = Cell::Type::WallV;
                        } else if(y == box.y || y == box.y + box.h - 1) {
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
        std::vector<std::array<int, 4>> tunnels(r - 1, std::array<int, 4>{ 0, 0, 0, 0 });
        std::uniform_int_distribution<int> rd(0, 3);
        for(unsigned int i = 0; i < connected.size(); ++i) { connected[i] = i + 1; }
        while(!all_of(connected.begin(), connected.end(), [](int is){ return is == 1; })) {
            ++r;
            std::uniform_int_distribution<int> rr(0, rm_cells.size() - 1);
            int xx, yy, rm, dir;
            do {
                dir = rd(rng);
                rm = rr(rng);
                std::uniform_int_distribution<int> rc(0, rm_cells[rm].size() - 1);
                int p = rc(rng);
                xx = rm_cells[rm][p].x;
                yy = rm_cells[rm][p].y;
            } while(!(
                cl_graph[yy][xx] != 0 && // in a room
                tunnels[cl_graph[yy][xx] - 1][dir] == 0 && // not already a tunnel in this direction
                (cells[yy][xx]->type == Cell::Type::Room || cells[yy][xx]->type != Cell::Type::Hall) // actually in the room part
            ));

            tunnels.emplace_back(std::array<int, 4>{ !(dir % 2), dir % 2, !(dir % 2), dir % 2 });
            rm_cells.emplace_back(std::vector<Point>());

            connected.emplace_back(connected[rm]);
            ++tunnels[rm][dir];
            while(xx >= 0 && xx < w && yy >= 0 && yy < h) {
                if(cells[yy][xx]->type == Cell::Type::Empty) {
                    cells[yy][xx]->type = Cell::Type::Hall;
                    cl_graph[yy][xx] = r;
                    rm_cells.back().emplace_back(Point{ xx, yy });
                } else if(cells[yy][xx]->type == Cell::Type::WallH || cells[yy][xx]->type == Cell::Type::WallV) {
                    cells[yy][xx]->type = Cell::Type::Door;
                    rm_cells.back().emplace_back(Point{ xx, yy });
                } else if((cells[yy][xx]->type == Cell::Type::Room || cells[yy][xx]->type == Cell::Type::Hall) && cl_graph[yy][xx] - 1 != rm) {
                    int low = std::min(connected[cl_graph[yy][xx] - 1], connected[rm]);
                    int hi = std::max(connected[cl_graph[yy][xx] - 1], connected[rm]);
                    std::replace(connected.begin(), connected.end(), hi, low);
                    ++tunnels[cl_graph[yy][xx] - 1][(dir + 2) % 4];
                    break;
                } else if(cells[yy][xx]->type == Cell::Type::Corner) {
                    break;
                }
                switch(dir) {
                    case 0: xx += 1; break;
                    case 1: yy += 1; break;
                    case 2: xx -= 1; break;
                    case 3: yy -= 1; break;
                }
            }
        }

        // remove dead ends

        // assign Cells to Rooms
        rooms = std::vector<std::shared_ptr<Room>>(r);
        std::transform(rm_cells.begin(), rm_cells.end(), rooms.begin(), [this] (std::vector<Point> points) {
            std::vector<std::shared_ptr<Cell>> cls(points.size());
            std::transform(points.begin(), points.end(), cls.begin(), [this] (Point point) { return cells[point.y][point.x]; });
            return std::make_shared<Room>(cls);
        });
    }

    std::string Map::to_string() const {
        std::string x = std::accumulate(cells.begin(), cells.end(), std::string(), [](std::string a, auto row) {
            return a + std::accumulate(row.begin(), row.end(), std::string(), [](std::string a, auto cell) {
                return a + static_cast<char>(cell->type);
            }) + "\n";
        });
        std::cerr << x;
        return x;
    }
}
