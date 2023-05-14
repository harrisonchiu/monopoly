roll
end
buy <tile_id>
view <tile_id>
sell <tile_id>
mortgage <tile_id>

# commands only to get stats about the game or debug
status/redraw/etc.

# only 1 trade between any 2 players can occur at a time, so no need for tradeid
trade <player> [tiles|money] offer [{tiles}|money] request [{tiles}|money]
remove item <trade_item_id>
propose trade <player>
abort trade <player>
accept trade
reject trade

# DETAILED COMMAND FORMATS
# 90 characters long max for log outputs

roll
- Player <id> rolled X.
- Player <id> rolled X. Doubles! Roll again.

buy <tile_id>
- Player <id> bought <tile_id> for $X. Has $X left.
- Player <id> does not have enough money. Need $X more.

view <tile_id>
- Showing <tile_id> property information in detail.

end
- Player <id> ended their turn.
- Cannot end turn! Player <id> must do some action.

trade <player> offer [{tiles}|$money] request [{tiles}|$money]
## trade 1 offer {12 34 1 4 $20} request {22 $100 $20 3}
## Use `{}` because it looks similar to brace expansion and command grouping in bash
##    and implies a set in many languages and in maths
## Can use `[]`? because it implies array, similar use case
##    but not used to create datastructures like `{}` in C++, bash
## Do NOT use `()` because it could be used for function args or tuples (usually ordered)
##    but we do not care about order. Our use case is not tuple-based.
## Do NOT use `""` or `''` because it implies string. It is not a string.
## We could also just accept all bracket types, but that is more work in the codebase
##    Unneeded compatibility
## Can use this multiple times to offer/request more items in the trade
## Each thing offered/requested is called `item` and has a unique id for it `item_id`
- Player <id> started a trade with Player <id>
- Player <id> added more items to the trading table

remove item <item_id>
- Player <id> is no longer requesting Player <id> X
- Player <id> is no longer offering Player <id> X
- No trade has been started! Start one by using `trade` command