# Breakblade

## About

Breakblade is a command-line utility for splitting up WAV files in musically useful ways.

## Usage

Breakblade has the following subcommands.

### `beats`

This command splits the loop into equal segments.

### `silence`

This command splits the audio on large portions of silence. The default parameters were chosen by trial
and error. You might have to tweak them for your purposes.

### `tempo`

This command splits the loop based on a provided BPM.

### `help`

This command provides help information. Use it followed by a subcommand to get more detailed information
on parameters and flags.