const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;
const Random = std.Random;
const Tuple = std.meta.Tuple;
const assert = std.debug.assert;

pub fn CartesianTree(comptime T: type, comptime op: fn (T, T) T) type {
    return struct {
        root: ?*Node = null,

        const Self = @This();
        const Node = CartesianTreeNode(T, op);

        pub const empty: Self = .{};

        pub fn initBySlice(
            allocator: Allocator,
            random: Random,
            values: []const T,
        ) Allocator.Error!Self {
            return initBySliceImpl(allocator, random, values, 0);
        }

        fn initBySliceImpl(
            allocator: Allocator,
            random: Random,
            _values: []const T,
            parent_prio: usize,
        ) Allocator.Error!Self {
            var values = _values;
            if (values.len == 0)
                return empty;
            var current = Self.fromNode(try Node.create(allocator, values[0], random.int(usize)));
            errdefer current.deinit(allocator);
            values = values[1..];
            while (current.root.?.priority >= parent_prio) {
                var next = try initBySliceImpl(allocator, random, values, parent_prio);
                if (next.size() == 0)
                    break;
                values = values[next.size()..];
                current = current.merge(next);
            }
            return current;
        }

        fn fromNode(node: ?*Node) Self {
            return .{ .root = node };
        }

        pub fn size(self: Self) usize {
            return (self.root orelse return 0).size;
        }

        pub fn query(self: Self) ?T {
            return if (self.root) |root| root.query else null;
        }

        fn applyLeft(self: Self, value: T) T {
            return if (self.root) |root| op(root.query, value) else value;
        }

        fn applyRight(self: Self, value: T) T {
            return if (self.root) |root| op(value, root.query) else value;
        }

        pub fn merge(self: Self, other: Self) Self {
            const left_root = self.root orelse return other;
            const right_root = other.root orelse return self;
            return Self.fromNode(left_root.merge(right_root));
        }

        pub fn split(self: Self, boundary: usize) Tuple(&.{ Self, Self }) {
            const original_size = self.size();
            assert(boundary <= original_size);
            const root = self.root orelse return .{ empty, empty };
            const nodes = root.split(boundary);
            return .{ Self.fromNode(nodes[0]), Self.fromNode(nodes[1]) };
        }

        pub fn clone(self: Self, allocator: Allocator) Allocator.Error!Self {
            return if (self.root) |root| Self.fromNode(try root.clone(allocator)) else empty;
        }

        pub fn deinit(self: Self, allocator: Allocator) void {
            if (self.root) |root| root.destroy(allocator);
        }
    };
}

fn CartesianTreeNode(comptime T: type, comptime op: fn (T, T) T) type {
    return struct {
        value: T,
        query: T,
        priority: usize,
        size: usize = 1,
        left: Tree = Tree.empty,
        right: Tree = Tree.empty,

        const Self = @This();
        const Tree = CartesianTree(T, op);

        pub fn create(allocator: Allocator, value: T, priority: usize) Allocator.Error!*Self {
            const self = try allocator.create(Self);
            self.* = .{
                .value = value,
                .query = value,
                .priority = priority,
            };
            return self;
        }

        pub fn merge(self: *Self, other: *Self) *Self {
            if (self.priority <= other.priority) {
                self.right = self.right.merge(Tree.fromNode(other));
                self.update();
                return self;
            } else {
                other.left = Tree.fromNode(self).merge(other.left);
                other.update();
                return other;
            }
        }

        pub fn split(self: *Self, boundary: usize) Tuple(&.{ ?*Self, ?*Self }) {
            assert(boundary <= self.size);
            if (boundary <= self.left.size()) {
                const parts = self.left.split(boundary);
                self.left = parts[1];
                self.update();
                return .{ parts[0].root, self };
            } else {
                const parts = self.right.split(boundary - self.left.size() - 1);
                self.right = parts[0];
                self.update();
                return .{ self, parts[1].root };
            }
        }

        fn update(self: *Self) void {
            self.size = 1 + self.left.size() + self.right.size();
            self.query = self.right.applyRight(self.left.applyLeft(self.value));
        }

        pub fn clone(self: *Self, allocator: Allocator) Allocator.Error!*Self {
            const left_cloned = try self.left.clone(allocator);
            errdefer left_cloned.deinit(allocator);
            const right_cloned = try self.left.clone(allocator);
            errdefer right_cloned.deinit(allocator);
            const cloned = try allocator.create(Self);
            cloned.* = .{
                .value = self.value,
                .query = self.query,
                .priority = self.priority,
                .size = self.size,
                .left = left_cloned,
                .right = right_cloned,
            };
            return cloned;
        }

        pub fn destroy(self: *Self, allocator: Allocator) void {
            self.left.deinit(allocator);
            self.right.deinit(allocator);
            allocator.destroy(self);
        }
    };
}

fn NopTree(comptime T: type) type {
    return CartesianTree(T, struct {
        fn nop(_: T, _: T) T {
            return undefined;
        }
    }.nop);
}

test "split empty" {
    const empty = NopTree(void).empty;
    try testing.expectEqualDeep(.{ empty, empty }, empty.split(0));
}

test "split single" {
    const allocator = testing.allocator;
    var random = std.Random.DefaultPrng.init(0);

    const empty = NopTree(u0).empty;
    defer empty.deinit(allocator);
    const single = try NopTree(u0).initBySlice(allocator, random.random(), &.{0});
    defer single.deinit(allocator);

    const split1 = (try single.clone(allocator)).split(0);
    defer split1[0].deinit(allocator);
    defer split1[1].deinit(allocator);
    try testing.expectEqualDeep(.{ empty, single }, split1);

    const split2 = (try single.clone(allocator)).split(1);
    defer split2[0].deinit(allocator);
    defer split2[1].deinit(allocator);
    try testing.expectEqualDeep(.{ single, empty }, split2);
}

test "merge with empty" {
    const allocator = testing.allocator;
    var random = std.Random.DefaultPrng.init(0);

    const empty = NopTree(u0).empty;
    defer empty.deinit(allocator);
    const single = try NopTree(u0).initBySlice(allocator, random.random(), &.{0});
    defer single.deinit(allocator);

    const mergeLeft = empty.merge(try single.clone(allocator));
    defer mergeLeft.deinit(allocator);
    try testing.expectEqualDeep(single, mergeLeft);

    const mergeRight = (try single.clone(allocator)).merge(empty);
    defer mergeRight.deinit(allocator);
    try testing.expectEqualDeep(single, mergeRight);
}
