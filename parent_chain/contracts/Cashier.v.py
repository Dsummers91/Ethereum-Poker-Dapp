owner: public(address)
balances: uint256(wei)[address]

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

