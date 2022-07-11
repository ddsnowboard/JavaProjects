const std = @import("std");
const expect = std.testing.expect;
const RndGen = std.rand.DefaultPrng;
var random = RndGen.init(82297);

const Player = struct {
    atBats: u32,
    doubles: u32,
    triples: u32,
    homeRuns: u32,
    singles: u32,

    fn bat(self: Player) AtBat {
        const roll = random.random().intRangeAtMost(u32, 0, self.atBats);
        return if (roll < self.singles)
            AtBat.Single
        else if (roll < self.singles + self.doubles)
            AtBat.Double
        else if (roll < self.singles + self.doubles + self.triples)
            AtBat.Triple
        else if (roll < self.singles + self.doubles + self.triples + self.homeRuns)
            AtBat.HomeRun
        else
            AtBat.Out;
    }
};

fn buildPlayer(atBats: u32, hits: u32, doubles: u32, triples: u32, homeRuns: u32) Player {
    return Player{ .atBats = atBats, .doubles = doubles, .triples = triples, .homeRuns = homeRuns, .singles = hits - doubles - triples - homeRuns };
}

const AtBat = enum { Out, Single, Double, Triple, HomeRun };

const batters = [_]Player{ buildPlayer(135, 24, 4, 0, 1), buildPlayer(315, 107, 28, 0, 19), buildPlayer(331, 85, 12, 3, 7), buildPlayer(77, 10, 3, 0, 1), buildPlayer(316, 92, 20, 1, 17), buildPlayer(166, 40, 6, 1, 4), buildPlayer(246, 63, 7, 3, 5), buildPlayer(238, 60, 17, 2, 5), buildPlayer(126, 25, 5, 0, 4) };

pub fn main() void {
    const totalInnings: u32 = 5000000;
    var iter = circularIterator(batters[0..]);
    var counter: u32 = 0;
    var totalScore: u32 = 0;
    while (counter < totalInnings) {
        totalScore += runInning(&iter);
        counter += 1;
    }
    std.debug.print("Scored {d} in {d} innings", .{ totalScore, totalInnings });
}

fn CircularIterator(comptime T: type) type {
    return struct {
        slice: []const T,
        index: usize = 0,
        pub fn next(self: *@This()) T {
            const out = self.slice[self.index];
            self.index = (self.index + 1) % self.slice.len;
            return out;
        }
    };
}

fn TypeOfSlice(comptime sliceType: type) type {
    return @typeInfo(@typeInfo(sliceType).Pointer.child).Array.child;
}

fn circularIterator(slice: anytype) CircularIterator(TypeOfSlice(@TypeOf(slice))) {
    return CircularIterator(TypeOfSlice(@TypeOf(slice))){ .slice = slice };
}

fn runInning(playerIterator: *CircularIterator(Player)) u32 {
    var first = false;
    var second = false;
    var third = false;
    var score: u32 = 0;
    var outs: u32 = 0;
    while (outs < 3) {
        const atBat = playerIterator.next().bat();
        switch (atBat) {
            .Out => {
                outs += 1;
            },
            .Single => {
                if (third) {
                    score += 1;
                }
                third = second;
                second = first;
                first = true;
            },
            .Double => {
                if (third) {
                    score += 1;
                }
                if (second) {
                    score += 1;
                }
                third = first;
                second = true;
                first = false;
            },
            .Triple => {
                if (third) {
                    score += 1;
                }
                if (second) {
                    score += 1;
                }
                if (first) {
                    score += 1;
                }
                third = true;
                first = false;
                second = false;
            },
            .HomeRun => {
                score += 1;
                if (third) {
                    score += 1;
                }
                if (second) {
                    score += 1;
                }
                if (first) {
                    score += 1;
                }
                first = false;
                third = false;
                second = false;
            },
        }
    }
    return score;
}
