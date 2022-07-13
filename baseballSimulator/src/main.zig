const std = @import("std");
const expect = std.testing.expect;
const RndGen = std.rand.DefaultPrng;
var random = RndGen.init(1000);

const Player = struct {
    atBats: u32,
    doubles: u32,
    triples: u32,
    homeRuns: u32,
    singles: u32,

    fn bat(self: Player) AtBat {
        const luckRoll = random.random().intRangeAtMost(u32, 0, self.atBats);
        return if (luckRoll < self.singles)
            AtBat.Single
        else if (luckRoll < self.singles + self.doubles)
            AtBat.Double
        else if (luckRoll < self.singles + self.doubles + self.triples)
            AtBat.Triple
        else if (luckRoll < self.singles + self.doubles + self.triples + self.homeRuns)
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
    var totalLOB: u32 = 0;
    while (counter < totalInnings) {
        const inning = runInning(&iter);
        totalScore += inning.score;
        totalLOB += inning.leftOnBase;
        counter += 1;
    }
    std.debug.print("Scored {d} in {d} innings, leaving {d} on\n", .{ totalScore, totalInnings, totalLOB });
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

fn runInning(playerIterator: *CircularIterator(Player)) Inning {
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
                if (second) {
                    if (roll(50)) {
                        third = false;
                        score += 1;
                    } else {
                        third = true;
                    }
                }
                if (first and !third) {
                    // Goes from second
                    if (roll(60)) {
                        // Goes from third
                        if (roll(30)) {
                            score += 1;
                            second = false;
                            third = false;
                        } else {
                            second = false;
                            third = true;
                        }
                    } else {
                        second = true;
                    }
                }
                first = true;
            },
            .Double => {
                if (second) {
                    score += 1;
                }
                if (third) {
                    score += 1;
                }
                if (first) {
                    if (roll(80)) {
                        third = false;
                        score += 1;
                    } else {
                        third = true;
                    }
                }
                second = true;
                first = false;
            },
            .Triple => {
                if (first) {
                    score += 1;
                }
                if (second) {
                    score += 1;
                }
                if (third) {
                    score += 1;
                }
                third = true;
                first = false;
                second = false;
            },
            .HomeRun => {
                score += 1;
                if (first) {
                    score += 1;
                }
                if (second) {
                    score += 1;
                }
                if (third) {
                    score += 1;
                }
                first = false;
                third = false;
                second = false;
            },
        }
    }
    return Inning{ .score = score, .leftOnBase = (if (first) @as(u32, 1) else 0) + (if (second) @as(u32, 1) else 0) + (if (third) @as(u32, 1) else 0) };
}

fn roll(prb: u32) bool {
    return random.random().intRangeAtMost(u32, 0, 100) <= prb;
}

const Inning = struct { score: u32, leftOnBase: u32 };
