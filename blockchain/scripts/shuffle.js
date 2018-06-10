var deck = new Array(52).fill(null).map((c,i) => i);

module.exports = (callback) => {
	console.log(playerEncrypt(deck, web3.eth.accounts[1]));
	callback(); 
}

function playerEncrypt(deck, account) {
	deck.shuffle();
	return deck.map((card) => {
		return web3.sha3(card + "secret");
	});	
}

Array.prototype.shuffle = function() {
	for (let i = this.length - 1; i > 0; i--) {
		const j = Math.floor(Math.random() * (i + 1));
		[this[i], this[j]] = [this[j], this[i]];
	}
	return this;
}
