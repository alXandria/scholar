# Enterprise Test December 9

*Store With Instantiation Limitation:*
junod tx wasm store alxandria.wasm --from testnet --fees 500ujunox --gas 2000000 --instantiate-only-address juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 -y

*Code ID*
3254

*Instantiation*
junod tx wasm instantiate 3254 '{"admin":"juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4"}' --label alxandria-test --admin juno1w5aespcyddns7y696q9wlch4ehflk2wglu9vv4 --from testnet --fees 500ujunox --gas 2000000 -y

https://testnet.juno.explorers.guru/transaction/B42946C23D2789999C2F0BD612CDABBE38B1CD5A72AC11B49C807C1A44799374

*Contract Address*
juno1w0f4z22frw3udamluzy43k43w4u43ya8r2sjt88ncjcc955cnw3qwlmdy0

*Query All Posts*
junod query wasm contract-state smart juno1w0f4z22frw3udamluzy43k43w4u43ya8r2sjt88ncjcc955cnw3qwlmdy0 '{"all_posts":{}}'

*Register Profile*
junod tx wasm execute juno1w0f4z22frw3udamluzy43k43w4u43ya8r2sjt88ncjcc955cnw3qwlmdy0 '{"create_profile":{"profile_name":"Vitalik","bio":"a beast man","profile_picture":"placeholder","cover_picture":"placeholder"}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/D6A1627EC1A1B93BC107F3228C67BE859C0AE744BFDCCDECB8867FEEBD3596BB

*Try to Register same profile name with different account: should fail*
junod tx wasm execute juno1w0f4z22frw3udamluzy43k43w4u43ya8r2sjt88ncjcc955cnw3qwlmdy0 '{"create_profile":{"profile_name":"Vitalik","bio":"a beast man","profile_picture":"placeholder","cover_picture":"placeholder"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/7BF27C3601656180B6630326519ABE534B6963BB41C53ADBAD1C81A92A779172

*Create Post:*
junod tx wasm execute juno1w0f4z22frw3udamluzy43k43w4u43ya8r2sjt88ncjcc955cnw3qwlmdy0 '{"create_post":{"editable":true,"post_title":"Post 3","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["pass","funds","attempt"]}}' --amount 1000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/73562EAFDD8CED7A4ED50102652922F04581C53C6C822B2DB25406646BC12912

*Query All Posts*
junod query wasm contract-state smart juno1w0f4z22frw3udamluzy43k43w4u43ya8r2sjt88ncjcc955cnw3qwlmdy0 '{"all_posts":{}}'

*Query One Post*
junod query wasm contract-state smart juno1w0f4z22frw3udamluzy43k43w4u43ya8r2sjt88ncjcc955cnw3qwlmdy0 '{"post":{"post_id": 1}}'

# Enterprise Test December 6

*Register Second Profile Name To Test Edit*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"register_profile_name":{"profile_name":"Satoshi"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/3775985E0D52A1D8AE721F236B59C331B38D1F538EE5B4040836C8E0CB3E1A19

*Register Second Profile*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"create_profile":{"bio":"I am Vitalik, creator of Ethereum", "profile_picture":"www.google.com", "cover_picture":"https://google.com"}}' --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/C836BACF5DD2E732358071E4878F402B4F08CB97D1F4AF89DFBFE6600B8FAF88

*Withdraw*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"withdraw":{}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/8CFC8E0A9B24160A0F31C5AAC6D686E67A2A2B1E1B9797A1FEAF2905A7630E56

*Test Edit & Fee Split:*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"edit_post":{"post_id":1,"external_id": "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text": "This edit should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 2000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/EEBCE2CAA6D7EB0D600D6E6D9571F9828039F36FB0267AB9DAC48BB95A168AA9

*Test Delete:*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"delete_post":{"post_id":1}}' --amount 10000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/555548A718BDE5F6CBBC6E183BC16DB7EBB70B7E3A1C3B912E814B5891177C51

*Create Uneditable Post:*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"create_post":{"editable":false,"post_title":"Post 1","external_id":"https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text":"This post should pass!","tags":["pass","funds","attempt"]}}' --amount 5000000ujunox --from ADDRESS --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/27D05A1E40E7135754E734B37E471B9094AE15D55A1D5F4175A99D4D29460C20

*Attempt to edit: Should Fail*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"edit_post":{"post_id":2,"external_id": "https://alxandria.infura-ipfs.io/ipfs/QmQSXMeJRyodyVESWVXT8gd7kQhjrV7sguLnsrXSd6YzvT","text": "This edit should pass!","tags": ["pass", "double funds", "edited post"]}}' --amount 2000000ujunox --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/6AFD71EF70AAE24B5DF0F17CA3FB33463B0E3B7019F6B84E9016114BA2834999

*Test removing editable nature*
junod tx wasm execute juno1wfjhqdekmqnak2mdwasrdcmr8js6zeaz5hwcd0arvs4se4v2stxqe2gruh '{"unlock_article":{"post_id":2}}' --from testnet --fees 500ujunox --gas 200000 -y

https://testnet.juno.explorers.guru/transaction/B1032D0E359776D871D3A5CBF53E4D76AB3CF5990519473911E60F2D71094FA0

# Old Migration Example

*Upload for migration*
https://explorer.testnet.sourceprotocol.io/juno/tx/E4EA5F6B0EFB4E1377A5ADF24A4D1B7DF6A8BD71AC65B33F1C1B2348B72A5EDA

*Code ID*
1258

*Migrate*
junod tx wasm migrate juno1apd88gqrqms48n3nvkcnxsttt86w02syk4n4t9we0sadqwh9x46sevje08 1258 '{"migrate_msg":{}}' --from testnet --fees 500ujunox --gas 2000000 -y

https://explorer.testnet.sourceprotocol.io/juno/tx/01FC12FBC3E2961B470C7397D7A37E5B4A0493817C79CD72F975B6343C3CBF74




