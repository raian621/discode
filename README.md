[![codecov](https://codecov.io/github/raian621/discode/graph/badge.svg?token=DUYDAFXFAY)](https://codecov.io/github/raian621/discode)

# DisCode

## Purpose

A cool Discord bot (written in Rust!) used to help you and your server grind LeetCode.

## Features

- [x] Query the daily LeetCode problem description
- [x] Query problem descriptions by title slugs (e.g. the slug for "Jump Game" is "jump-game")
- [x] Query problem descriptions by problem IDs (e.g. the problem "Sqrt(x)" has an ID of 69)
- [ ] Query problem descriptions by title (ex. "Jump Game")
- [x] Link LeetCode accounts to members' Discord accounts
- [ ] Calculate scores for members with linked LeetCode accounts based on the number of questions and the difficulty of the questions they have completed upon request
- [ ] Generate a leaderboard based on member LeetCode scores upon request
- [ ] Scheduled announcements (Leaderboard results, daily LeetCode notifications, etc)
- [ ] Managed role used to pinging members when the daily LeetCode problem is updated

## Commands

### `/connect <leetcode-username>`

Connect your LeetCode account with your Discord account. Simply

- Run the command
- Get your DisCode validation token
- Add the DisCode validation token to the skills section of your LeetCode profile
- Re-run the command to verify that the validation token is in the skills section of your LeetCode profile
- Optionally remove the validation token from the skills section of your LeetCode profile

The account connection will be used to tabulate your score on DisCode and server leaderboards and allow you to flex your LeetCode stats.

### `/daily`

Display the description and stats for the daily LeetCode problem.

### `/problem <problem-identifier> [type]`

Search for a problem by its

- title slug i.e. `jump-game` (`type: slug`)
- id number i.e. `55` (`type: id`)

and display the description and stats for the problem if it exists.

## Running the Bot

### Prerequisites

- A Discord application set up with a Discord token in the Discord Developer 
Portal (see https://discord.com/developers/docs/quick-start/getting-started)
- A PostgreSQL database
    - We use Docker containers to run databases in development: (Docker install instructions for Ubuntu: https://docs.docker.com/engine/install/ubuntu/)
- Rust tools such as cargo, rustup, etc. (download instructions: https://www.rust-lang.org/tools/install)

### Required Packages

(non-exhaustive list, there may be more packages necessary to run the bot)

**Ubuntu**:
```sh
sudo apt-get install -y postgresql-client libssl-dev
```

### Database

DisCode uses a PostgreSQL relational database. In development or while testing,
you can use the included convenience scripts in located at 
`scripts/start-dev-db-container` and `scripts/start-test-db-container` to launch
Docker containers for the development database and testing database
respectively. In production, you can use whatever PostgeSQL server configuration
you want I guess.

Once DisCode's database is spun up for the first time and you have set the 
relevant [environment variables](#environment-variables) for connecting to the database,
you can apply DisCode's database migrations by running

```sh
sqlx migrate run
```

> [!TIP]
> You can install the `sqlx` cli tool by running
> ```sh
> cargo install sqlx-cli
> ```

To run the bot, make sure to set the relevant 
[environment variables](#environment-variables) and then run the following
command:

```sh
cargo run
```

### Environment Variables

Name            | Required  | Purpose
----------------|-----------|--------------
`DISCORD_TOKEN` | yes       | Used by the bot to authenticate to the Discord API
`GUILD_ID`      | no        | ID of the server you are using to test the bot
`DB_PASSWORD`   | yes       | Password for the bot's PostgreSQL account
`DB_USER`       | yes       | Username for the bot's PostgreSQL account
`DB_NAME`       | yes       | Name of the bot's PosgreSQL database
`DB_HOST`       | yes       | Hostname of the bot's PostgreSQL database server
`DB_PORT`       | no        | Port of the database server used in production (defaults to `5432`)
`DEV_DB_PORT`   | for dev   | Port of the database server used in development
`TEST_DB_PORT`  | for tests | Port of the database server used in development
`DATABASE_URL`  | for sqlx  | URL to the PostgreSQL database server used by the bot
