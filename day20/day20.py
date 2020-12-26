import sys
import math

class Tile:
    def __init__(self, id, data):
        self.id = id
        self.data = data

    def __repr__(self):
        return ("Tile %s:\n" % self.id) + "\n".join(self.data)

    def get_borders(self):
        top = self.data[0]
        right = "".join([x[9] for x in self.data])
        bottom = self.data[9]
        left = "".join([x[0] for x in self.data])
        return [top,right,bottom,left]

    def flip_x(self):
        self.data = [x[::-1] for x in self.data]

    def flip_y(self):
        self.data = list(reversed(self.data))

    def rotate(self):
        data = [[] for x in range(len(self.data))]
        for x in self.data:
            for i in range(len(self.data)):
                data[i].append(x[len(self.data)-1-i])
        self.data = ["".join(x) for x in data]

    def strip_borders(self):
        self.data = [x[1:-1] for x in self.data[1:-1]]

def parse_input():
    tiles = {}
    for tile in sys.stdin.read().split("\n\n"):
        lines = tile.splitlines()
        id = lines[0].split(" ")[-1][:-1]
        tiles[id] = Tile(id, lines[1:])
    return tiles

def solve_part1(tiles):
    corner_product = 1
    corner_tile_ids = []
    for id, tile in tiles.items():
        tile_borders = set(tile.get_borders())
        tile_flipped_borders = set([x[::-1] for x in reversed(tile.get_borders())])
        matches = 0
        for other_id, other_tile in tiles.items():
            if id == other_id:
                continue
            other_tile_borders = other_tile.get_borders()
            if tile_borders.intersection(other_tile_borders) or\
                    tile_flipped_borders.intersection(other_tile_borders):
                matches += 1
        if matches == 2:
            corner_product *= int(id)
            corner_tile_ids.append(id)

    return corner_product, corner_tile_ids

def solve_part2(tiles, corner_tile_id):
    # We want to make corner_tile the top left one and start from there,
    # to do that we identify which 2 sides of it match with other tiles
    # and rotate it so they are to the right and to the bottom
    corner_tile_borders = tiles[corner_tile_id].get_borders()
    corner_tile_matching_borders = [False] * 4
    for id, tile in tiles.items():
        if id == corner_tile_id:
            continue
        other_tile_all_borders = tile.get_borders() +\
                [x[::-1] for x in reversed(tile.get_borders())]
        for i in range(4):
            if corner_tile_borders[i] in other_tile_all_borders:
                corner_tile_matching_borders[i] = True
        if sum([int(x) for x in corner_tile_matching_borders]) == 2:
            break

    # Since this is a corner tile, it's always going to be 2 borders
    # that are adjacent to each other that have the neighbours
    # e.g. top and right, top and left, bottom and left, bottom and right
    # In order for this to be the top left one, we rotate it so it's
    # the right and bottom ones
    while corner_tile_matching_borders != [False, True, True, False]:
        corner_tile_matching_borders = corner_tile_matching_borders[1:] +\
                [corner_tile_matching_borders[0]]
        tiles[corner_tile_id].rotate()

    # Start finding and laying out the rest of the tiles
    num_rows = int(math.sqrt(len(tiles)))
    grid = [[None for x in range(num_rows)] for y in range(num_rows)]
    grid[0][0] = corner_tile_id
    taken_ids = [corner_tile_id]

    for row in range(num_rows):
        # Manually handle the first one as we are going to match it to the
        # one above, while all the other ones will be matched to the side
        if row != 0:
            border_above = tiles[grid[row-1][0]].get_borders()[2]
            found = False
            for id, tile in tiles.items():
                if id in taken_ids:
                    continue
                tile_borders = tile.get_borders()
                tile_flipped_borders = [x[::-1] for x in reversed(tile_borders)]
                if border_above in (tile_borders + tile_flipped_borders):
                    try:
                        side = tile_borders.index(border_above)
                    except:
                        tile.flip_x()
                        tile.flip_y()
                        side = tile.get_borders().index(border_above)
                    if side == 1:
                        tile.rotate()
                    if side == 2:
                        tile.flip_y()
                    if side == 3:
                        tile.flip_x()
                        tile.rotate()
                    found = True
                    break
            assert found
            grid[row][0] = id
            taken_ids.append(id)

        for col in range(1, num_rows):
            border_to_the_left = tiles[grid[row][col-1]].get_borders()[1]
            found = False
            for id, tile in tiles.items():
                if id in taken_ids:
                    continue
                tile_borders = tile.get_borders()
                tile_flipped_borders = [x[::-1] for x in reversed(tile_borders)]
                if border_to_the_left in (tile_borders + tile_flipped_borders):
                    try:
                        side = tile_borders.index(border_to_the_left)
                    except:
                        tile.flip_x()
                        tile.flip_y()
                        side = tile.get_borders().index(border_to_the_left)
                    if side == 0:
                        tile.rotate()
                        tile.flip_y()
                    if side == 1:
                        tile.flip_x()
                    if side == 2:
                        tile.flip_y()
                        tile.rotate()
                        tile.flip_y()
                    found = True
                    break
            assert found
            grid[row][col] = id
            taken_ids.append(id)

    # Now that the tile positions have been found, let's strip away the borders
    # and fit them into a full image
    image = [[] for i in range(num_rows * 8)]
    for row, cols in enumerate(grid):
        for col, tile_id in enumerate(cols):
            tiles[tile_id].strip_borders()
            for i in range(8):
                image[row*8+i] += tiles[tile_id].data[i]

    image = Tile(0, ["".join(x) for x in image])
    image.flip_y()
    image.rotate()

    monster = """
                  # 
#    ##    ##    ###
 #  #  #  #  #  #   """[1:].split("\n")
    monster_bits = []
    for row in range(3):
        for col, char in enumerate(monster[row]):
            if char == "#":
                monster_bits.append((row,col))

    num_monsters = 0
    num_rotations = 0
    monster_hashes = set()
    while num_monsters == 0:
        image.rotate()
        num_rotations += 1
        if num_rotations == 4:
            image.flip_x()
        if num_rotations == 9:
            print("Can't find any bloody monsters")
            break
        for row in range(len(image.data) - 3):
            for col in range(len(image.data[0]) - len(monster[0])):
                found_monster = True
                potential_monster_hashes = set()
                for bit in monster_bits:
                    if image.data[row+bit[0]][col+bit[1]] == "#":
                        potential_monster_hashes.add((row+bit[0], col+bit[1]))
                    else:
                        found_monster = False
                if found_monster:
                    monster_hashes = monster_hashes.union(potential_monster_hashes)
                    num_monsters += 1

    for each in monster_hashes:
        a = []
        for i, x in enumerate(image.data[each[0]]):
            if i != each[1]:
                a.append(x)
            else:
                a.append("O")
        image.data[each[0]] = "".join(a)

    roughness = 0
    for row, chars in enumerate(image.data):
        for col, char in enumerate(chars):
            if char == "#":
                roughness += 1

    return roughness

if __name__ == "__main__":
    tiles = parse_input()

    corner_product, corner_tile_ids = solve_part1(tiles)
    print("Part 1:", corner_product)

    print("Part 2:", solve_part2(tiles, corner_tile_ids[0]))
