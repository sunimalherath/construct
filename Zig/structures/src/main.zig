const std = @import("std");

pub fn main() !void {
    const usr = User{ .name = "John" };
    // std.debug.print("User's name: {s}\n", .{usr.name});

    usr.printUserName();

    User.printUserRank(usr);

    const newUsr = User.init(6, "Jane");
    newUsr.printUserRank();
    newUsr.printUserName();

    const anotherUsr = User{
        .name = "Zib",
        .rank = 2,
    };
    anotherUsr.printUserName();
    anotherUsr.printUserRank();

    // trying to change user properties
    // newUsr.name = "Someone";
    // newUsr.printUserName();
    // cannot assign to const

    var varUsr = User{ .name = "temp name", .rank = 2 };
    varUsr.printUserName();
    varUsr.name = "new name";
    varUsr.printUserName();

    // even though the struct was declared as a const, variable created from it allows to change the properties

    const p1 = Player{ .name = "Novak", .rank = 1 };
    const p2: Player = Player{ .name = "Federer", .rank = 2 };
    const p3: Player = .{ .name = "Nadal", .rank = 3 };
}

pub const User = struct {
    rank: u8 = 4,
    name: []const u8,

    pub fn printUserName(user: User) void {
        std.debug.print("User name: {s}\n", .{user.name});
    }

    pub fn printUserRank(user: User) void {
        std.debug.print("User rank {d}\n", .{user.rank});
    }

    pub fn init(rank: u8, name: []const u8) User {
        return User{
            .rank = rank,
            .name = name,
        };
    }
};

pub const Player = struct {
    name: []const u8,
    rank: u8,

    pub fn init(name: []const u8, rank: u8) Player {
        // can also return like this without specifying the type.
        return .{
            .name = name,
            .rank = rank,
        };
    }
};
