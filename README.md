# Softsquirrel's PNGme implementation
Encode hidden messages in a PNG file!

## What is PNGme?
PNGme is a project for the rust programming language, in which you must implement a program which can encode and decode hidden messages in PNG files.

## How does it work?
PNG files consist of many "blocks" called chunks. A chunk can include information such as the payload, metadata for image editors, and pretty much anything. The only two parts that you need to know about right now are the *chunk type* and *payload*. Anyways, these chunks can encode *anything*. Yep. Anything. And that is exactly what this program takes advantage of. By using this program, all you have to do is provide a valid chunk code, and write your message!

## Guide
Currently this only works on Linux, but if you want Windows support just open an issue! Anyways, assuming you know how to install the program just use the little guide below:

## `sspngme encode <FILE NAME> <CHUNK NAME> <PAYLOAD IN QUOTES">`
Encode a message in a PNG file

## `sspngme decode <FILE NAME> <CHUNK TYPE>`
Decode a message

## `sspngme remove <CHUNK TYPE>`
Remove a chunk

# Todo
- Improve error handling