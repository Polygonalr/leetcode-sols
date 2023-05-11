class Solution:
    def floodFill(self, image: List[List[int]], sr: int, sc: int, color: int) -> List[List[int]]:
        s = set()
        queue = [(sr, sc)]
        to_replace = image[sr][sc]
        while len(queue) != 0:
            (cx, cy) = queue[0]
            queue = queue[1:]
            s.add((cx, cy))
            for (dx, dy) in [(0,1), (0,-1), (1,0), (-1,0)]:
                (nx, ny) = (cx+dx, cy+dy)
                if nx == len(image) or nx == -1 or ny == len(image[0]) or ny == -1:
                    continue
                if (nx, ny) not in s and image[nx][ny] == to_replace:
                    s.add((nx, ny))
                    queue.append((nx, ny))
        for (x, y) in s:
            image[x][y] = color
        return image
