const std = @import("std");

pub fn main() void {
    create_slice_can_change_values();
}

pub fn create_slice_cannot_change_values() void {

    // array

    // const x = [_]i32{ 1, 4, 5, 3, 7, 8 }; // array type is [6]const i32

    // slice of x
    // var end: usize = 2;
    // end += 1;
    // const y = x[1..end]; // slice type is []const i32
    // std.debug.print("slice y values are: {s}", .{y});

    // var z = x[1..end]; // slice type is []const i32

    // since z is variable slice, should be able to change the elements in it - but cannot
    // becasue it is a pointer to the x array and x is immutable.

    // z[1] = 6; // cannot assign to constant

    // std.debug.print("slice z values are: {s}", .{z});

    // to change the elements of the slice, the array should be a variable
}

pub fn create_slice_can_change_values() void {
    var x2 = [_]u8{ 1, 4, 5, 3, 7, 8 };
    std.debug.print("array x2:{any}\n", .{x2});

    // slice of x
    var end: usize = 3;
    end += 1;

    // creating the slice from x
    const y2 = x2[1..end];
    std.debug.print("slice y2:{any}\n", .{y2});

    // mutate x2
    x2[2] = 22; // changes the x2 as well as y2

    // mutate y2
    y2[2] = 6; // changes both x2 and y2

    std.debug.print("after changing x2 by x2[2] = 22:{any}\n", .{x2});
    std.debug.print("after changing y2 by y2[2] = 6:{any}\n", .{y2});

    // what if change x2 by x2[4]=10 ? what will happen to y2 since it oly has 3 items
    x2[4] = 10;
    std.debug.print("array after changing x2 by x2[4] = 10:{any}\n", .{x2});
    std.debug.print("slice after changing x2 by x2[4] = 10:{any}\n", .{y2});

    // doing the above won't affect the slice y2
}
