# Documentation

This markdown file is for myself to log technical details and things that I need to work on.
You may find this file interesting, but anything logged here is not intended for external readers.

## TODO

## Implementation

Have seperate processes for:

- Creating and modifying polls schedules (reads user input, saves to DB)
- Creating the polls (reads from the DB, creates a poll)
- Update the polls as users react (updates the poll information)

The idea is seperation of concerns for each process.
