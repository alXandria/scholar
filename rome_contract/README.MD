# Enterprise Test December 9

*Store With Instantiation Limitation:*
junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

*Code ID*
3255

*Instantiation*
junod tx wasm instantiate 3255 '{"admin":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}' --label alxandria-test --admin juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 --from testnet --fees 500ujunox --gas 2000000 -y

https://testnet.juno.explorers.guru/transaction/87921F6AF8C423638B80A49ECE358588332ADB1DAA50D4231FD0E26BA1808131

*Contract Address*
juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd

*Register Profile*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"create_profile":{"profile_name":"Vitalik","bio":"a beast man","profile_picture":"placeholder","cover_picture":"placeholder"}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/A3DB35000BDABAAA2CE80529B1963C7E86F80B93B95691806F21B9B45E7B5864

*Try to Register same profile name with different account: should fail*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"create_profile":{"profile_name":"Vitalik","bio":"a beast man","profile_picture":"placeholder","cover_picture":"placeholder"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/9FC44427037E037509FA6EB54CE1381AC91972F193E51CBC46ECD882B889B017

*Create Post:*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"create_post":{"editable":true,"post_title":"Post 3","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["pass","funds","attempt"]}}' --amount 1000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/0A4CFCCD84071B065C38184CE625478ADA931F661072DF209A3DE813C6F5338D

*Query All Posts*
junod query wasm contract-state smart juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"all_posts":{}}'

*Query One Post*
junod query wasm contract-state smart juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"post":{"post_id": 1}}'

*Register Second Profile*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"create_profile":{"profile_name":"sato  shi","bio":"a beast man","profile_picture":"placeholder","cover_picture":"placeholder"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/0DEBB1014AF87DD476CDD3ECE413F28A49A4E0CAF368969DC83927E21CAAD50E

*Test Edit & Fee Split:*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"edit_post":{"post_id":1,"external_id": "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text": "This edit should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 2000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/0AF99B50BCEFCECA0D9601B1EB0AE39AFB095CAC98127A99E662228045A5017C

*Test Delete:*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"delete_post":{"post_id":1}}' --amount 10000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/60B01086ECF12CC8966F22DA698E277F44C046E1A2DFF76A45F40492EF6B1B38

*Query Article Count*
junod query wasm contract-state smart juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"article_count":{}}'

*Create Uneditable Post:*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"create_post":{"editable":false,"post_title":"Post 1","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["pass","funds","attempt"]}}' --amount 5000000ujunox --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/900823C82A609F24B8A67D2FFA12FAD66CAE376399EE2CCBDF68D3C8C2BCB39D

*Attempt to edit: Should Fail*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"edit_post":{"post_id":2,"external_id": "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text": "This edit should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 2000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/B7F235E3889A5D43590AE3D7594A1F2FB6CAD5BEC226C899EA46F874713C731C

*Test removing editable nature*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"unlock_article":{"post_id":2}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/DA5565A4C4F1D4E68E5BD09C351C1D7FF9CCED6F8140FF78EE3BA78A4C4974FF

*Withdraw*
junod tx wasm execute juno1a822ztnp3nxpfvk36tv8e2765t0x6vpd9ct5t7rn30w9azhmqr4q0rzgsd '{"withdraw":{}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/8769EDB37718E0BDD9F81D04DAE636FCA931E85985F812097550F0FDDEE56C60

# Old Migration Example

*Upload for migration*
https://explorer.testnet.sourceprotocol.io/juno/tx/E4EA5F6B0EFB4E1377A5ADF24A4D1B7DF6A8BD71AC65B33F1C1B2348B72A5EDA

*Code ID*
1258

*Migrate*
junod tx wasm migrate juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 1258 '{"migrate_msg":{}}' --from testnet --fees 500ujunox --gas 2000000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/01FC12FBC3E2961B470C7397D7A37E5B4A0493817C79CD72F975B6343C3CBF74




