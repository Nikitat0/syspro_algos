const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;
const Random = std.Random;
const assert = std.debug.assert;

const CartesianTree = @import("./tree.zig").CartesianTree;

pub fn CartesianTreeArray(comptime T: type, comptime op: fn (T, T) T) type {
    return struct {
        allocator: Allocator,
        random: Random,
        tree: Tree = Tree.empty,

        const Self = @This();
        const Tree = CartesianTree(T, op);

        pub fn init(allocator: Allocator, random: Random) Self {
            return .{ .allocator = allocator, .random = random };
        }

        pub fn initBySlice(
            allocator: Allocator,
            random: Random,
            values: []const T,
        ) Allocator.Error!Self {
            return .{
                .allocator = allocator,
                .random = random,
                .tree = try Tree.initBySlice(allocator, random, values),
            };
        }

        pub fn len(self: Self) usize {
            return self.tree.size();
        }

        pub fn query(self: *Self, from: usize, to: usize) T {
            assert(from <= to and to < self.len());
            const parts = self.tree.split(from);
            const rightParts = parts[1].split(to - from + 1);
            defer self.tree = parts[0].merge(rightParts[0]).merge(rightParts[1]);
            return rightParts[0].query().?;
        }

        pub fn append(self: *Self, value: T) Allocator.Error!void {
            try self.insert(self.len(), value);
        }

        pub fn erase(self: *Self, from: usize, count: usize) void {
            assert(0 <= from and from + count <= self.len());
            const parts = self.tree.split(from);
            const rightParts = parts[1].split(count);
            rightParts[0].deinit(self.allocator);
            self.tree = parts[0].merge(rightParts[1]);
        }

        pub fn insert(self: *Self, pos: usize, value: T) Allocator.Error!void {
            assert(pos <= self.len());
            const parts = self.tree.split(pos);
            self.tree = parts[0]
                .merge(try Tree.initBySlice(self.allocator, self.random, &.{value}))
                .merge(parts[1]);
        }

        pub fn remove(self: *Self, idx: usize) T {
            assert(idx < self.len());
            defer self.erase(idx, 1);
            return self.query(idx, idx);
        }

        pub fn deinit(self: Self) void {
            self.tree.deinit(self.allocator);
        }
    };
}

pub fn RangeSumQueryArray(comptime T: type) type {
    return CartesianTreeArray(T, struct {
        fn sum(a: T, b: T) T {
            return a + b;
        }
    }.sum);
}

pub fn RangeMinQueryArray(comptime T: type) type {
    return CartesianTreeArray(T, struct {
        fn min(a: T, b: T) T {
            return @min(a, b);
        }
    }.min);
}

pub fn RangeMaxQueryArray(comptime T: type) type {
    return CartesianTreeArray(T, struct {
        fn max(a: T, b: T) T {
            return @max(a, b);
        }
    }.max);
}

const CartesianTreeArrayTest = CartesianTreeArray(usize, struct {
    fn mul(a: usize, b: usize) usize {
        return a * b;
    }
}.mul);

fn mulQuery(array: []const usize, from: usize, to: usize) usize {
    assert(from <= to and to < array.len);
    var mul = array[from];
    for (array[from + 1 .. to + 1]) |item|
        mul *= item;
    return mul;
}

fn expectArray(expected: []const usize, actual: *CartesianTreeArrayTest) !void {
    try testing.expectEqual(expected.len, actual.len());
    for (0..expected.len) |i| for (i..expected.len) |j|
        try testing.expectEqual(mulQuery(expected, i, j), actual.query(i, j));
}

test "empty" {
    var random = std.Random.DefaultPrng.init(0);
    const empty = CartesianTreeArrayTest.init(testing.allocator, random.random());
    defer empty.deinit();
    try testing.expectEqual(0, empty.len());
}

test "query" {
    var random = std.Random.DefaultPrng.init(0);

    const reference: []const usize = &.{ 2, 3, 5, 7, 11 };
    var tree = CartesianTreeArrayTest.init(testing.allocator, random.random());
    defer tree.deinit();
    for (reference) |item|
        try tree.append(item);
    try expectArray(reference, &tree);
}

test "insert & append & erase & remove" {
    const allocator = testing.allocator;
    var random = std.Random.DefaultPrng.init(0);

    var reference = std.ArrayList(usize).init(allocator);
    try reference.appendSlice(&.{ 2, 5, 7 });
    defer reference.deinit();
    var tree = try CartesianTreeArrayTest.initBySlice(allocator, random.random(), reference.items);
    defer tree.deinit();
    try expectArray(reference.items, &tree);

    try reference.insert(1, 3);
    try tree.insert(1, 3);
    try expectArray(reference.items, &tree);

    try reference.append(11);
    try tree.append(11);
    try expectArray(reference.items, &tree);

    try reference.replaceRange(1, 2, &.{});
    tree.erase(1, 2);
    try expectArray(reference.items, &tree);

    try reference.replaceRange(1, 1, &.{});
    try testing.expectEqual(7, tree.remove(1));
    try expectArray(reference.items, &tree);
}
