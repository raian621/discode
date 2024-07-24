# LeetCode Bot

## Purpose

A cool Discord bot (written in Rust!) used to help you and your server grind LeetCode.

## Features

- [x] Query the daily LeetCode problem description
- [x] Query problems descriptions by its title slug (e.g. the slug for "Jump Game" is "jump-game")
- [x] Query problems descriptions by its problem ID (e.g. the problem "Sqrt(x)" has an ID of 69)
- [ ] Query problems descriptions by its title (ex. "Jump Game")
- [ ] Link LeetCode accounts to members' Discord accounts
- [ ] Calculate scores for members with linked LeetCode accounts based on the number of questions and the difficulty of the questions they have completed upon request
- [ ] Generate a leaderboard based on member LeetCode scores upon request
- [ ] Scheduled announcements (Leaderboard results, daily LeetCode notifications, etc)
- [ ] Managed role used to pinging members when the daily LeetCode problem is updated

## Running

### Prerequisites

In order to run this bot, you must first create a Discord App on the Discord Developer Portal and procure a Discord API token:

> [!IMPORTANT] 
> Guide: https://discord.com/developers/docs/quick-start/getting-started

---

To run the bot, run the following command:

```sh
cargo run
```

### Environment Variables

Name            | Optional | Purpose
----------------|----------|--------------
`DISCORD_TOKEN` | no       | Used by the bot to authenticate to the Discord API
`GUILD_ID`      | yes      | ID of the server you are using to test the bot

