const std = @import("std");

pub fn main() !void {
    const arr = [_]u8{ 12, 44, 45, 1, 128, 76, 32, 255 };

    const value: u8 = 73;

    var i: u8 = 0;

    while (i < arr.len) : (i += 1) {
        if (arr[i] == value) {
            std.debug.print("Found the value, {d} at index: {d}", .{ value, i });

            break;
        }
    } else {
        std.debug.print("The value {d} isn't in the array", .{value});
    }
}
