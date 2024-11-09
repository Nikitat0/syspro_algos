const std = @import("std");
const testing = std.testing;
const Allocator = std.mem.Allocator;
const ArrayListUnmanaged = std.ArrayListUnmanaged;
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
            var builder = try Builder.init(allocator, random, values);
            defer builder.deinit();
            return builder.build();
        }

        const Builder = struct {
            allocator: Allocator,
            random: Random,
            values: []const T,
            nodes: ArrayListUnmanaged(*Node),

            pub fn init(
                allocator: Allocator,
                random: Random,
                values: []const T,
            ) Allocator.Error!Builder {
                const nodes = try ArrayListUnmanaged(*Node).initCapacity(
                    allocator,
                    2 * std.math.log2(values.len),
                );
                return .{
                    .allocator = allocator,
                    .random = random,
                    .values = values,
                    .nodes = nodes,
                };
            }

            pub fn build(self: *Builder) Allocator.Error!Self {
                while (self.nextValue()) |value| {
                    const node = try Node.create(self.allocator, value, self.random.int(usize));
                    errdefer node.destroy(self.allocator);
                    try self.appendNode(node);
                }
                if (self.nodes.items.len == 0)
                    return empty;
                while (self.nodes.items.len != 1) {
                    const child = self.nodes.pop();
                    var parent = self.nodes.getLast();
                    parent.right = fromNode(child);
                    parent.update();
                }
                return fromNode(self.nodes.pop());
            }

            fn appendNode(self: *Builder, node: *Node) Allocator.Error!void {
                while (self.parentPriority() > node.priority) {
                    const last = self.nodes.pop();
                    last.right = node.left;
                    last.update();
                    node.left = fromNode(last);
                }
                node.update();
                try self.nodes.append(self.allocator, node);
            }

            fn nextValue(self: *Builder) ?T {
                if (self.values.len == 0)
                    return null;
                defer self.values = self.values[1..];
                return self.values[0];
            }

            fn parentPriority(self: *Builder) usize {
                return if (self.nodes.getLastOrNull()) |last| last.priority else 0;
            }

            pub fn deinit(self: *Builder) void {
                for (self.nodes.items) |node|
                    node.destroy(self.allocator);
                self.nodes.deinit(self.allocator);
            }
        };

        fn fromNode(node: ?*Node) Self {
            return .{ .root = node };
        }

        fn update(self: *Self) void {
            if (self.root) |root| root.update();
        }

        pub fn isEmpty(self: Self) bool {
            return self.root == null;
        }

        pub fn size(self: Self) usize {
            return if (self.root) |root| root.size else 0;
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
            return fromNode(left_root.merge(right_root));
        }

        pub fn split(self: Self, boundary: usize) Tuple(&.{ Self, Self }) {
            assert(boundary <= self.size());
            const root = self.root orelse return .{ empty, empty };
            const nodes = root.split(boundary);
            return .{ fromNode(nodes[0]), fromNode(nodes[1]) };
        }

        pub fn clone(self: Self, allocator: Allocator) Allocator.Error!Self {
            return if (self.root) |root| fromNode(try root.clone(allocator)) else empty;
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
