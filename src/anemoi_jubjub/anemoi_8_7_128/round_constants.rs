use super::BigInteger256;
use super::Felt;
use super::{NUM_COLUMNS, NUM_HASH_ROUNDS};

/// Additive round constants C for Anemoi.
pub(crate) const C: [[Felt; NUM_COLUMNS]; NUM_HASH_ROUNDS] = [
    [
        Felt::new(BigInteger256([
            0x00000055ffffffaa,
            0xde4ae6fe008d1856,
            0xca916d4cc3a16e35,
            0x0e29ca100517e9be,
        ])),
        Felt::new(BigInteger256([
            0x8e4b366c91d85f64,
            0xd1725408b713a189,
            0x191fa3dbeb1f02f3,
            0x2db6b209c4f75e49,
        ])),
        Felt::new(BigInteger256([
            0x13d14eb0d83b43c1,
            0xa60125e90d5dd57e,
            0x22fd7ac9dc4fccac,
            0x6e1747653a40418a,
        ])),
        Felt::new(BigInteger256([
            0x316cec0bb9c99ec0,
            0xa0dbb0cf0332f3dd,
            0x04e467ed0c38f1c0,
            0x19ee9dfd7f7a32a3,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x39e0de29f4b12254,
            0x4542c782afdc7771,
            0x73b230448757841f,
            0x4c66a29cf76de864,
        ])),
        Felt::new(BigInteger256([
            0x07cc39a6994baddc,
            0x5cfc96417918d480,
            0x5cf7196e6c84dfed,
            0x68d156f877ea8a5f,
        ])),
        Felt::new(BigInteger256([
            0xaba41242b5d9cbce,
            0x1195689caaaf19d9,
            0xb5e2169250897bc3,
            0x45861d6fa89188b4,
        ])),
        Felt::new(BigInteger256([
            0xa1ae62bc65becb2d,
            0xd0b11db20f05c876,
            0x60a366226c125c6f,
            0x368ad1de7f9590bb,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x3fc21fde7d7b78d8,
            0x82f0645e8f5af33c,
            0x1bb34fcb9aea8258,
            0x15a7d78b0c310f52,
        ])),
        Felt::new(BigInteger256([
            0xe85354c6303129ed,
            0x4a4c3ada62e4e51d,
            0xb822a76308dd3aaf,
            0x38aaca5df031d0d9,
        ])),
        Felt::new(BigInteger256([
            0x8435fe2c10700949,
            0xf6fe29ff645ee889,
            0x8fd110e7eab0d3e3,
            0x1b020d3c46635672,
        ])),
        Felt::new(BigInteger256([
            0xe436d1c5ae60c47b,
            0xed56ef3694796632,
            0x5f83d10bf1fc4b17,
            0x2c850ea591eaf0d3,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xe7c71a3a93f9cf8d,
            0xc1a4d5f501e1e624,
            0xe0520e391ee539f2,
            0x5ce1f5aa1353f292,
        ])),
        Felt::new(BigInteger256([
            0xa6c8d78f14ddbc80,
            0x42963f76b3326fb7,
            0x22d86558b3d7dddd,
            0x4ea73ae6d7a1f859,
        ])),
        Felt::new(BigInteger256([
            0x5a2b58450721ebe6,
            0x6cad5145624f3d8e,
            0x3e52d82126b6a2cf,
            0x4d34ae72d0f3467a,
        ])),
        Felt::new(BigInteger256([
            0x828f1ff2b91a9ffb,
            0x3814880a48bf1cbe,
            0x7eea491d7cbf0383,
            0x1dfca94d6ed235fc,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xf468ec640f9392e9,
            0xd0b2b188f86888b7,
            0x32641a368b203537,
            0x63a9c811eda5f734,
        ])),
        Felt::new(BigInteger256([
            0x2994338bf99e4923,
            0x2b69c199a60c5746,
            0x9357e42aed43113a,
            0x2578a301f83072a8,
        ])),
        Felt::new(BigInteger256([
            0x140a214191847cca,
            0xf7f8ab922ed97e6d,
            0x3d7f42f9e8159775,
            0x737ba0f24074df71,
        ])),
        Felt::new(BigInteger256([
            0x3c96450530ee18e7,
            0xb35c71daa68c5991,
            0x7c9f2c6c88379676,
            0x409a4e587643bbcf,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xaea1c1ed34038852,
            0x8fee23ccb54da3d2,
            0x223ee1a3d2627714,
            0x0e328961d797ccde,
        ])),
        Felt::new(BigInteger256([
            0xf0c935cc3a1e02e5,
            0x253d70a70c145e9c,
            0x4719f12fdf39facc,
            0x2ff8e18bc4cffb1c,
        ])),
        Felt::new(BigInteger256([
            0x44d9763f2f258b03,
            0xb8f61709ff0e31b3,
            0xa640b3f4e2abb508,
            0x0b6d001e6433bea0,
        ])),
        Felt::new(BigInteger256([
            0x15ae957581ae0ace,
            0xe07b1155d294d343,
            0x41f7294f7b107ab4,
            0x53f171e831f8e2ca,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xc4534349519b429e,
            0x7afc9f1db435b2c5,
            0x90bddf0600dc3a58,
            0x73050f54a903dff4,
        ])),
        Felt::new(BigInteger256([
            0x1e1a7421bb7bfd58,
            0xe3d16bed961a9a0a,
            0x488c10f297eef93c,
            0x6b357fc8861ef0d8,
        ])),
        Felt::new(BigInteger256([
            0x7c4d3c2fd64b70f2,
            0x16386c266b3946e9,
            0x852ca096d6330a9d,
            0x4005e6938e30d279,
        ])),
        Felt::new(BigInteger256([
            0x93006b1c3df7f286,
            0xa4758fc18631e75c,
            0x406aba4d4a3847f1,
            0x28c1fbe509ff91a9,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x1f77830c66fb7746,
            0xc7e57426850a228b,
            0x90c8fd2720008c56,
            0x0db8e1cc4926a688,
        ])),
        Felt::new(BigInteger256([
            0x732855acb4b0ecb4,
            0x2ccd4c938458f9b7,
            0x0413cda2c83f417d,
            0x442a3022818fe55b,
        ])),
        Felt::new(BigInteger256([
            0x401095eda9d1a761,
            0x105779290e1f4fc9,
            0x76ef041eaba32a24,
            0x322f5b569046af91,
        ])),
        Felt::new(BigInteger256([
            0x1eb3dc7c876eaa4e,
            0xbd9050d000ee166d,
            0xcb0db03b4be7892f,
            0x1f33f48d374cc015,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x80c25874b4297a46,
            0xaa9f314b56e67a01,
            0xc2cfcec4f5772235,
            0x19b1f168c24e4630,
        ])),
        Felt::new(BigInteger256([
            0x6fd624771d471e51,
            0x87836a0ff5ceb3b7,
            0x71d3d846a2949e7b,
            0x2cd481e87b3d8b56,
        ])),
        Felt::new(BigInteger256([
            0xf2a85ed31e3ae87a,
            0x95fff2c83231e524,
            0xbffb90dd4b31ddae,
            0x1fa5d67a744b80c0,
        ])),
        Felt::new(BigInteger256([
            0xe4c7816582cd62c1,
            0x4f57dc6274cab93a,
            0x93a6972e17b8e603,
            0x65fceedd20f2f6ff,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x7179a4ce184eefbc,
            0x347ac5811ccf5b5d,
            0xc914af5ca815aa17,
            0x6a3578c4c95b5a78,
        ])),
        Felt::new(BigInteger256([
            0xf965ecd5a3b011dc,
            0xc82cc1c4799cfc98,
            0xe9b90a31b6fae240,
            0x24c94dafbaa90180,
        ])),
        Felt::new(BigInteger256([
            0x0c0452a8956c16a0,
            0x4333226e20b87da1,
            0x55c92bdfcd61552d,
            0x70a8c87bfd68d0dc,
        ])),
        Felt::new(BigInteger256([
            0xd4740996274d08cd,
            0x666ab5dfe441de0b,
            0x45dca09d79bd25d6,
            0x1d6b4020ad25cc7e,
        ])),
    ],
];

/// Additive round constants D for Anemoi.
pub(crate) const D: [[Felt; NUM_COLUMNS]; NUM_HASH_ROUNDS] = [
    [
        Felt::new(BigInteger256([
            0xdb6db731db6db685,
            0x7142c545dbfb8331,
            0x900aec24bf80a3ea,
            0x010d14a33c66b40d,
        ])),
        Felt::new(BigInteger256([
            0x8454e7e5ad0ee33d,
            0x3c9b7db4e87bdf02,
            0x624ff7a9d9eff4f0,
            0x35b144c687f7dcc0,
        ])),
        Felt::new(BigInteger256([
            0xc0e75f1cac365d77,
            0x896b9efbe71d902a,
            0xf8d4ed7f2abfc47a,
            0x2308ff32c74c2af4,
        ])),
        Felt::new(BigInteger256([
            0x2781c62c8f5c4e8b,
            0x62a601b773f915e7,
            0x07c58ab521ba1f76,
            0x0ebf490e42211b72,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x85a7e15aaf2d443b,
            0x117700705caabf2b,
            0x388a69df469d9aaa,
            0x24a3065923591607,
        ])),
        Felt::new(BigInteger256([
            0x6e2f377493909cc1,
            0x01621a937be0eed7,
            0xa58627ff1ebcb2c0,
            0x562502de2f876c2a,
        ])),
        Felt::new(BigInteger256([
            0xc9136f0268e35091,
            0x81f9e05855cd0d63,
            0xbe521c126c022c6c,
            0x53be95b953d752bb,
        ])),
        Felt::new(BigInteger256([
            0x081c89321a5fe604,
            0xcbb7c940512bc75f,
            0x62e343ad44fa6afb,
            0x10b4961836d8dcde,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x64d2592ebf0f4b82,
            0xe00a6585e62ef6c5,
            0x018a3ce5c7ba4395,
            0x58df94b12b8f8584,
        ])),
        Felt::new(BigInteger256([
            0x27ff88b4b18dc994,
            0x2bd9e3630fb45f45,
            0xee76916b1efce02f,
            0x1d0c285a71a47deb,
        ])),
        Felt::new(BigInteger256([
            0x7aee910c4a913ece,
            0xa48ac5f1b9843be3,
            0x8605f1df6a11573a,
            0x2048379cbb7eebc0,
        ])),
        Felt::new(BigInteger256([
            0x23ee2e5aea199015,
            0x794362fe80a520ea,
            0x82c26216386e0456,
            0x71aa2c493ca18585,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x881777a8f8b525a4,
            0x0cecfabfd8b3dad2,
            0x8309a9d37073249b,
            0x468825ca5ab5c71d,
        ])),
        Felt::new(BigInteger256([
            0x61b52f9ab961df95,
            0x660fafa5dffe3702,
            0x4946d5e8f85784cd,
            0x4d64b330aab5810c,
        ])),
        Felt::new(BigInteger256([
            0xcc240f42646aa4d9,
            0x5c25b4de3770de0b,
            0x24a23fa0d4772796,
            0x6cd6f32097afb769,
        ])),
        Felt::new(BigInteger256([
            0x3d86a0a617faef02,
            0xb22f1f75b4e8c89a,
            0x5f0988a7e7eee62c,
            0x099039eb418c2907,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xf6f0aa2500024218,
            0x4265762a8c34c425,
            0x79d9852ea691db8b,
            0x40cff07e2f0a974f,
        ])),
        Felt::new(BigInteger256([
            0x46b7ebea29d5c550,
            0x754dd19f8fd26551,
            0x5e842418fba673d5,
            0x17b61397c546c6ec,
        ])),
        Felt::new(BigInteger256([
            0xe83a38927a808ed4,
            0xba1e0afec0f709ab,
            0x9552a1cf5617ffe2,
            0x12b03698d7969ea8,
        ])),
        Felt::new(BigInteger256([
            0x59c5260b1b81c106,
            0x53e1a91ccfb04c2d,
            0x017c3b54bd4b34cb,
            0x1fadd74243007a6b,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x41dad06db733ecfe,
            0xd08ca59bf4b3c172,
            0x5ea6eae943e9a327,
            0x4ca798897a1986e9,
        ])),
        Felt::new(BigInteger256([
            0x9e9e3eeafd17348e,
            0xea4f99d7a175f2da,
            0xd3fef7633a110b21,
            0x0f979189c965ec07,
        ])),
        Felt::new(BigInteger256([
            0xa9bade4faae3528a,
            0x4a0733a43cc59f23,
            0xf306b117a6c3a335,
            0x0bf07c805c7297c7,
        ])),
        Felt::new(BigInteger256([
            0xc38ec73bff036869,
            0xfc2e61c2a7544c11,
            0x888cfe7cfc97c6c3,
            0x20663a3a36353e0d,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xacc654df74dce163,
            0xade94d12e983c7d7,
            0xc5606bd87f206595,
            0x63f66c0a34d7ab35,
        ])),
        Felt::new(BigInteger256([
            0x212980551e86691b,
            0xeeef6547216281b9,
            0x00e572bb0924e0c1,
            0x713e24a79da47042,
        ])),
        Felt::new(BigInteger256([
            0x3668a754f21a7293,
            0xed5558e99ed707cb,
            0xfd66f94eb0a9cff8,
            0x66f357d6995f3a1e,
        ])),
        Felt::new(BigInteger256([
            0x961a9ff75b5e8a3b,
            0x0634b05750d7b39c,
            0xb274eb0fe21e6b30,
            0x1ba0b918212b7b6a,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0xda40257681aa35eb,
            0x506dc70d67f989a9,
            0x2fd11bf58d484e28,
            0x156d5f9b7f802afa,
        ])),
        Felt::new(BigInteger256([
            0x488cf2b40f287857,
            0x8d86eadebd423373,
            0x26d2c1672878bf96,
            0x60f5f61b439b1df5,
        ])),
        Felt::new(BigInteger256([
            0xcc8191e6bd0dc8e2,
            0x3d100addef5e62b7,
            0x598eeed2751d8614,
            0x6fdfedb345fad067,
        ])),
        Felt::new(BigInteger256([
            0xf423a22b9c4261e3,
            0x74eb1657793534b9,
            0xa77d72f9d2d14302,
            0x28d5d2d9f8fe6307,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x4b322f3ccd0ecf67,
            0x797ce549c39bcbef,
            0x343809622d01f632,
            0x2a0f6ef12a584db9,
        ])),
        Felt::new(BigInteger256([
            0x54e1f5dc75f54070,
            0x2e926972b87dd842,
            0x66f2e7d9cd112ec0,
            0x5249479a6ef94707,
        ])),
        Felt::new(BigInteger256([
            0x8ec08f2a2fada077,
            0x090de5949d36e2e2,
            0x74fb975fdeef4bca,
            0x65ff68905bb024ad,
        ])),
        Felt::new(BigInteger256([
            0xc9de7b7395d7b0d1,
            0xf94a5efe76d96657,
            0x0f3c9db35f43d9fb,
            0x045a258feab79fc0,
        ])),
    ],
    [
        Felt::new(BigInteger256([
            0x94d8dd7553f1e7b5,
            0x81132674f7e8767a,
            0x94adad7a6ef16db8,
            0x704845e0ca98ec0b,
        ])),
        Felt::new(BigInteger256([
            0x3761201a1f1bd6d3,
            0xecf66e1caaafea53,
            0x3908dd4570c86229,
            0x3ff362f54798473c,
        ])),
        Felt::new(BigInteger256([
            0x010be4dfc99c7174,
            0xe03e1e2cfa22e88f,
            0x31c01ddae6cddae7,
            0x38ca02d25463818b,
        ])),
        Felt::new(BigInteger256([
            0x127a65825d14f9b6,
            0xe1d5897454b2b057,
            0x4edd42ab5a3ae178,
            0x256b6dba39bb7c91,
        ])),
    ],
];
