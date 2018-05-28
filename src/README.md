# Insecure Poker Dapp
Poker Dapp that probably wont be secure but should be fun making it #yolo


## Future Goals
Each Table as its own state channel
Staking mechanism
Tournaments, maybe? - would need a central entity to enforce player positions on table
Satellite Tournaments, which tokens would represent ticket for entry in place of a currency


## Encryption
Encryption will be probably similar but less secure than Virtue Pokers implementation
Each player can create or generate a secret to each card on the deck
  - Shuffle and hash each card with the secret (and position?)
  - When player recieves card, that cards secret is given to expose the card to only that specific user
