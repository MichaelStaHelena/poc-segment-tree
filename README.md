# Segment Tree (Java)

A **segment tree** is a binary tree data structure used to answer **range queries** (like sum, minimum, maximum, gcd) and apply **point/range updates** on an array efficiently.

## Why use a segment tree?

For an array of size `n`:
- Building the tree takes `O(n)`
- Querying a range takes `O(log n)`
- Updating a value takes `O(log n)`

This is much faster than recomputing results over a full range (`O(n)`) for each query.

## Common use cases

- Competitive programming / coding interviews
- Analytics on mutable arrays
- Real-time scoring/statistics systems
- Any scenario with frequent range queries + updates

## Java example (range sum + point update)

```java
public class SegmentTree {
    private final int[] tree;
    private final int n;

    public SegmentTree(int[] nums) {
        this.n = nums.length;
        this.tree = new int[4 * n];
        build(nums, 1, 0, n - 1);
    }

    private void build(int[] nums, int node, int left, int right) {
        if (left == right) {
            tree[node] = nums[left];
            return;
        }
        int mid = left + (right - left) / 2;
        build(nums, node * 2, left, mid);
        build(nums, node * 2 + 1, mid + 1, right);
        tree[node] = tree[node * 2] + tree[node * 2 + 1];
    }

    public int query(int ql, int qr) {
        return query(1, 0, n - 1, ql, qr);
    }

    private int query(int node, int left, int right, int ql, int qr) {
        if (qr < left || right < ql) return 0;          // no overlap
        if (ql <= left && right <= qr) return tree[node]; // total overlap

        int mid = left + (right - left) / 2;
        int sumLeft = query(node * 2, left, mid, ql, qr);
        int sumRight = query(node * 2 + 1, mid + 1, right, ql, qr);
        return sumLeft + sumRight;
    }

    public void update(int index, int value) {
        update(1, 0, n - 1, index, value);
    }

    private void update(int node, int left, int right, int index, int value) {
        if (left == right) {
            tree[node] = value;
            return;
        }
        int mid = left + (right - left) / 2;
        if (index <= mid) {
            update(node * 2, left, mid, index, value);
        } else {
            update(node * 2 + 1, mid + 1, right, index, value);
        }
        tree[node] = tree[node * 2] + tree[node * 2 + 1];
    }

    public static void main(String[] args) {
        int[] nums = {2, 4, 5, 7, 8, 9};
        SegmentTree st = new SegmentTree(nums);

        System.out.println(st.query(1, 4)); // 24 (4+5+7+8)
        st.update(2, 10);                   // nums[2] = 10
        System.out.println(st.query(1, 4)); // 29 (4+10+7+8)
    }
}
```
