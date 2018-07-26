## Encryption Problems

1. [Card Revealing](#card-reveal)


## Card Reveal<a name="card-reveal"></a>
Scenario: Three players in Hand, Player A, B collude with eachother, Player C has no idea and is playing normally.


If Player B and C have money in the pot while hand is still being played, but realizes he will ultimately lose the hand. He may tell Player A not to reveal secret to decrypt the cards left in the deck. Leading to a deadlock due to not being able to confirm who won the hand

This also works is Player A and B are not colluding but Player A randomly disconnects and is unable to release secret.


Possible Resolutions:
- When Player A folds he must release his decyption secret
- In event Player A disconnects and is unable to fold, either Player A is punished. (Would be hard to make sure Player C is deserving of the pot, so when Player A is punished - does Player B also get kickbacks from punishment?)
- Each player may (optionally) select their own third-party overseer, that has access to decryption keys. In event Player A is unable to release secret, that third party may release decryption on Player A's behalf.
