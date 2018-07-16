owner: public(address)
balances: uint256(wei)[address]
verified_roots: bytes32[bool]

@public
def __init__():
  self.owner = msg.sender

@public 
def withdraw(amount: uint256(wei)):
    assert self.balances[msg.sender] >= amount
    self.balances[msg.sender] = self.balances[msg.sender] - amount

@public
@constant
def balanceOf(user: address) -> uint256(wei):
    return self.balances[user]

@public
@payable
def __default__():
  self.balances[msg.sender] = self.balances[msg.sender] + msg.value  


## Vyper port of https://github.com/OpenZeppelin/openzeppelin-solidity/blob/master/contracts/MerkleProof.sol
@constant
@public
def verify(proofs: bytes32[100], root: bytes32, leaf: bytes32) -> bool:
    computed_hash: bytes32 = leaf
    for proof in proofs:
        if leaf < proof:
            computed_hash = sha3(concat(computed_hash, proof))
        else:
            computed_hash = sha3(concat(proof, computed_hash))

    return computed_hash == root


