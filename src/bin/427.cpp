#include <iostream>
#include <vector>
#include <algorithm>

using namespace std;

class Node {
 public:
  bool val;
  bool isLeaf;
  Node* topLeft;
  Node* topRight;
  Node* bottomLeft;
  Node* bottomRight;

  Node() : val(false), isLeaf(false), topLeft(nullptr), topRight(nullptr), bottomLeft(nullptr), bottomRight(nullptr) {}

  Node(bool _val, bool _isLeaf) : val(_val), isLeaf(_isLeaf), topLeft(nullptr), topRight(nullptr), bottomLeft(nullptr), bottomRight(nullptr) {}

  Node(bool _val, bool _isLeaf, Node* _topLeft, Node* _topRight, Node* _bottomLeft, Node* _bottomRight)
      : val(_val), isLeaf(_isLeaf), topLeft(_topLeft), topRight(_topRight), bottomLeft(_bottomLeft), bottomRight(_bottomRight) {}
};

class Solution {
 public:
  Node* construct(vector<vector<int>>& grid) {
    return helper(grid, 0, 0, grid.size());
  }

 private:
  Node* helper(const vector<vector<int>>& grid, int i, int j, int w) {
    if (allSame(grid, i, j, w))
      return new Node(grid[i][j] == 1, /*isLeaf=*/true);
    const int half = w / 2;
    return new Node(/*val=*/true, /*isLeaf=*/false,
                    /*topLeft=*/helper(grid, i, j, half),
                    /*topRight=*/helper(grid, i, j + half, half),
                    /*bottomLeft=*/helper(grid, i + half, j, half),
                    /*bottomRight=*/helper(grid, i + half, j + half, half));
  }

  bool allSame(const vector<vector<int>>& grid, int i, int j, int w) {
    return all_of(grid.begin() + i, grid.begin() + i + w,
                  [&](const vector<int>& row) {
      return all_of(row.begin() + j, row.begin() + j + w,
                    [&](int num) { return num == grid[i][j]; });
    });
  }
};

void printQuadTree(Node* node) {
  if (!node) return;
  if (node->isLeaf) {
    cout << "Leaf(val=" << node->val << ") ";
  } else {
    cout << "Node(val=" << node->val << ") { ";
    printQuadTree(node->topLeft);
    printQuadTree(node->topRight);
    printQuadTree(node->bottomLeft);
    printQuadTree(node->bottomRight);
    cout << "} ";
  }
}

int main() {
  vector<vector<int>> grid = {
      {1, 1, 0, 0},
      {1, 1, 0, 0},
      {0, 0, 1, 1},
      {0, 0, 1, 1}
  };

  Solution solution;
  Node* root = solution.construct(grid);

  cout << "Quad Tree structure: ";
  printQuadTree(root);

  return 0;
}
