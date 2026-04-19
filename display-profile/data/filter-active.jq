# Command:
#     jq -f display-profile/data/filter-active.jq display-profile/data/PROFILE/dump.json -C | less -R
# or:
#     OTHER-COMMAND | jq -f display-profile/data/filter-active.jq -C | less -R

map(
    select(
        .flags | any(. == "ACTIVE")
    )
)
