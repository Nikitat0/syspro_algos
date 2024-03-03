const std = @import("std");
const testing = std.testing;
const mem = std.mem;
const Allocator = mem.Allocator;

const DynamicArrayError = error{
    IndexOutOfBounds,
    ArrayIsEmpty,
};

fn DynamicArray(comptime T: type) type {
    return struct {
        const Self = @This();

        data: []T,
        len: usize,
        allocator: Allocator,

        pub fn init(allocator: Allocator) Self {
            return Self{
                .data = &[_]T{},
                .len = 0,
                .allocator = allocator,
            };
        }

        pub fn push(self: *Self, item: T) !void {
            if (self.data.len == 0)
                self.data = try self.allocator.alloc(T, 8);
            if (self.len == self.data.len)
                self.data = try self.allocator.realloc(self.data, self.data.len * 2);

            self.data[self.len] = item;
            self.len += 1;
        }

        pub fn at(self: *Self, index: usize) !*T {
            if (index < self.len)
                return &self.data[index];
            return DynamicArrayError.IndexOutOfBounds;
        }

        pub fn pop(self: *Self) !T {
            if (self.len == 0)
                return DynamicArrayError.ArrayIsEmpty;
            self.len -= 1;
            return self.data[self.len];
        }

        pub fn deinit(self: *Self) void {
            if (self.data.len > 0) {
                self.allocator.free(self.data);
            }
            self.* = undefined;
        }
    };
}

test "dynamic array" {
    var dyn_arr = DynamicArray(u8).init(testing.allocator);
    defer dyn_arr.deinit();

    try dyn_arr.push(42);

    try testing.expectEqual((try dyn_arr.at(0)).*, 42);
    try testing.expectError(DynamicArrayError.IndexOutOfBounds, dyn_arr.at(1));

    try testing.expectEqual(try dyn_arr.pop(), 42);
    try testing.expectError(DynamicArrayError.ArrayIsEmpty, dyn_arr.pop());
}

test "dynamic array realloc" {
    var dyn_arr = DynamicArray(usize).init(testing.allocator);
    defer dyn_arr.deinit();

    for (0..15) |i| {
        try testing.expectError(DynamicArrayError.IndexOutOfBounds, dyn_arr.at(i));
        try dyn_arr.push(i);
        try testing.expectEqual((try dyn_arr.at(i)).*, i);
    }

    for (0..15) |_| {
        _ = try dyn_arr.pop();
    }
    try testing.expectError(DynamicArrayError.ArrayIsEmpty, dyn_arr.pop());
}
