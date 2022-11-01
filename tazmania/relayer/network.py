import near_api

class ContractCalls:

	def __init__(self, contract_id, signer_id, signer_key, provider):
		self.contract_id = contract_id
		self.signer_id = signer_id
		self.signer_key = signer_key
		self.provider = provider

		near_provider = near_api.providers.JsonProvider(provider)
		key_pair = near_api.signer.KeyPair(signer_key)
		signer = near_api.signer.Signer(signer_id, key_pair)

		self.near = near_api.account.Account(near_provider, signer, signer.account_id)

	def n_leaves(self):
		return self.near.function_call(self.contract_id, "n_leaves", {})
		 
	def get_leaves(self):
		return self.near.function_call(self.contract_id, "get_leaves", {})
