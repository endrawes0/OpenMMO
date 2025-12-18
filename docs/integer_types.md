# Integer Types Policy

This document defines the canonical integer type usage across the server and client.

## Wire Protocol
- IDs and timestamps use `u64`.
- IDs sent to the client must be `<= i64::MAX` to fit Godot `int`.
- Levels, health, and resource values use `u32`.
- Experience uses `u64`.

## Server
- Account and character identifiers remain UUIDs internally and in the database.
- ECS entity identifiers use `u64` (`EntityId`).
- Conversions from database models to wire types are validated; no truncation is allowed.

## Client
- Godot `int` values are treated as signed 64-bit.
- All incoming `u64` values are validated to be non-negative and `<= i64::MAX`.
