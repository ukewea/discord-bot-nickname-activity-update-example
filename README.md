# Discord Bot Nickname and Activity Update Example

This example written in Rust will regularly updates its nickname and activity based on a specified schedule using the Rust programming language with the Serenity library.

## Features

- Automatically updates the bot's nickname to the current date and time.
- Sets a custom activity for the bot that reflects the current date and time.
- Iterates through all guilds the bot is part of to update settings.
- Utilizes asynchronous Rust features for efficient performance.

## Prerequisites

Before you begin, ensure you have the following installed:
- Rust and Cargo (Visit [Rust Installation](https://www.rust-lang.org/tools/install) for guide)
- An active Discord bot token

## Discord Bot Permissions

Ensure that the bot has the following permissions in each guild:
- Change Nickname

These permissions are necessary for the bot to operate correctly and update its status across all guilds.