<div align = "center">

![Wise Old Man](https://user-images.githubusercontent.com/3278148/86636807-a32b9f00-bfcc-11ea-963f-8fb2920447f4.png)

An Open Source CLI to interact with the Wise Old Man Old School Runescape progress tracker API.

The Wise Old Man is a web app (and API) that measures your Old School Runescape player progress. Built on top of the OSRS hiscores, it allows you to keep track of your gains, participate in group competitions, collect achievements and much more.

[Website](https://wiseoldman.net/) |
[Discord](https://discord.gg/Ky5vNt2) |
[Patreon](https://www.patreon.com/wiseoldman)

</div>

## Usage

```
Wise Old Man CLI 1.0.0
CLI Interface to the Wise Old Man OSRS API

USAGE:
    wom <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    achievements    Get achievments - representation of a milestone in the player's progression
    competitions    Get competition information. A competition is a comparison of a group of players' deltas, for a
                    specific metric, within a specific time range
    deltas          Get deltas - a representation of the difference between snapshots of a specific player. This can
                    be used to calculate the player's gained experience/score/kills in any metric and/or time
                    period, check a player's progress in a competition and generate records for the player (if new
                    delta is higher than record, update record)
    groups          Get groups - a list of players, with roles for each player
    help            Prints this message or the help of the given subcommand(s)
    names           Get player name change data
    players         Find data relating to a specific player
    records         Get records - a representation of a player's absolute best deltas for a specific period and
                    metric
    search          Searches for players given a partial username
    snapshots       Get snapshots - a representation of a player's account stats at any given point in time, this
                    currently includes the experience/score/kills and ranks of all the metrics
```

### Search
```
wom search -u lyn

wom search --username="lynx t"
```

### Players

#### Stats
```
wom players --username linxis stats

wom players -u="lynx titan" stats
```