# Enterprise Test December 6

*Store With Instantiation Limitation:*
junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

*Code ID*
3049

*Instantiation*
junod tx wasm instantiate 3049 '{"admin":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}' --label alxandria-test --admin juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 --from testnet --fees 500ujunox --gas 2000000 -y

https://testnet.juno.explorers.guru/transaction/B1FAD47E295175A14C5172D31F2BA669C3AABBBA8DC89EF1B75E90BE45A89953

*Contract Address*
juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh

*Register Profile Name*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"register_profile_name":{"profile_name":"Vitalik"}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/87453B0593434C03E1A89BFC8BD6A177F89CC10939867EAB5C6A7ADDA33A229F

*Register Profile*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"create_profile":{"bio":"I am Vitalik, creator of Ethereum", "profile_picture":"www.google.com", "cover_picture":"https://google.com"}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/6868CCEC812E4CEC143655EC6B64E137B8E0E1A0C114BCB6355E5FD4743CC8E3

*Try to Register same profile name with different account: should fail*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"register_profile_name":{"profile_name":"Vitalik"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/F344D42AC62660BF9906D96CF25952D79E2F98503FCB54717E7793A5CAD7DE2E

*Create Post:*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"create_post":{"editable":true,"post_title":"Post 1","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["pass","funds","attempt"]}}' --amount 1000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/A320F1A87907B40150081E72FFB10D8A78F4A7380E08920B9B3CAC5D0CD40FEA

*Query All Posts*
junod query wasm contract-state smart juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"all_posts":{}}'


# Full Test October 21

*Upload for migration*
https://explorer.testnet.sourceprotocol.io/juno/tx/E4EA5F6B0EFB4E1377A5ADF24A4D1B7DF6A8BD71AC65B33F1C1B2348B72A5EDA

*Code ID*
1258

*Migrate*
junod tx wasm migrate juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 1258 '{"migrate_msg":{}}' --from testnet --fees 500ujunox --gas 2000000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/01FC12FBC3E2961B470C7397D7A37E5B4A0493817C79CD72F975B6343C3CBF74

*Test Edit & Fee Split:*
junod tx wasm execute juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"edit_post":{"post_id":1,"external_id": "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text": "This edit should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 2000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/C1C589FA6D393C1831FF1FEA629B18549B97A4C9F30D65AEFB274AFCC63841C1

*Test Delete:*
junod tx wasm execute juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"delete_post":{"post_id":1}}' --amount 10000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/468A1077735D3E778A9F24CAD268FDCD7459B14294BC1EA2FF4F880A19617EEC

*Query One Post*
junod query wasm contract-state smart juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"post":{"post_id": 1}}'

*Withdraw*
junod tx wasm execute juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 '{"withdraw":{}}' --from testnet --fees 500ujunox --gas 200000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/D627D7FA21D950F1B8120601DD9E59A6ED6D00A0D301D79A1F479054A0020DF3
