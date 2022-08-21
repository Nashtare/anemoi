use super::BigInteger384;
use super::Felt;
use super::NUM_HASH_ROUNDS;

/// Additive round constants C for Anemoi.
pub(crate) const C: [Felt; NUM_HASH_ROUNDS] = [
    Felt::new(BigInteger384([
        0x554e0000006f54e6,
        0xfb9801b4a468005b,
        0x5e4d31fe251edcdf,
        0xec5b788a48618a47,
        0x01eb303433447efa,
        0x129aa068fb2122b2,
    ])),
    Felt::new(BigInteger384([
        0xaf8842374610fd4f,
        0x44977c689d0bfbff,
        0x3219561f9f572518,
        0x219ad8d001cefc41,
        0xa788eb5936eb3069,
        0x114906e42d3525b2,
    ])),
    Felt::new(BigInteger384([
        0xe3b9df48c8a2dce4,
        0xbb2c1b9b44487dd4,
        0x3f7c3a95f53d835f,
        0xf7f5035d40a06106,
        0x092ea343ea1e465c,
        0x0ce327ca3934c193,
    ])),
    Felt::new(BigInteger384([
        0x3016ef9979d96cf6,
        0x5856c522bae21569,
        0x80391bdcca543195,
        0x0b8877e14e2ab858,
        0xe493ffb341473727,
        0x11182fcaaa29983d,
    ])),
    Felt::new(BigInteger384([
        0xab21fd59b9c21f9a,
        0xcbdc24518e56b16d,
        0x1802e4c76b194656,
        0x91ee92ec97003e5b,
        0x1617a16309943af8,
        0x06a590e9f424abbf,
    ])),
    Felt::new(BigInteger384([
        0xa56afd4698b3e027,
        0x82e066b3a4a90aad,
        0x98a848c960ea604f,
        0xdebc932ee883dd9b,
        0xaf162edfb610ad02,
        0x122690f274fb48f1,
    ])),
    Felt::new(BigInteger384([
        0x0ac633ec0f9ed5c6,
        0x2d4975e1432bc04e,
        0x886a94c421734588,
        0x9628d9f0fe1e020c,
        0xe540a1f286234168,
        0x1872e7c1a537f17b,
    ])),
    Felt::new(BigInteger384([
        0x1266bf394049156f,
        0x23d6e9c0313b88a5,
        0x996e80b65d93210a,
        0x739ce296e4c945de,
        0xdab1df119a7c3760,
        0x05feecdf2b61e634,
    ])),
    Felt::new(BigInteger384([
        0xc0d83808eb7f604f,
        0x6d93e2aee6304bfc,
        0x6596c5b8c5da2f8b,
        0xf5ea136838bdf4df,
        0xf9ed67ab58387808,
        0x0a1a018aca7dc1db,
    ])),
    Felt::new(BigInteger384([
        0x9becb1b618c73764,
        0x256667fd6b696a00,
        0x7f1157ab9fdd2607,
        0x67817025c977bcca,
        0x26dc483c1d3480cc,
        0x0a8b3a522660ef70,
    ])),
    Felt::new(BigInteger384([
        0x89422710ae50060c,
        0x0ec87f403bafe63a,
        0x59b0f6af25e5ec0b,
        0x51dadef6ecdbfdc2,
        0xba19f0a40779927d,
        0x18a860ddbb27de9d,
    ])),
    Felt::new(BigInteger384([
        0xc528ae62d491768d,
        0xf820b0da1c6d3d7e,
        0x27c0edb2252e9c9c,
        0x62c17bd6c5a72c9e,
        0xb35ad2b9c4d8178e,
        0x0507bf0ef358f4a2,
    ])),
    Felt::new(BigInteger384([
        0xe32ee000f77d2c46,
        0xed46afaf8b2fdb60,
        0x16a888f52804ef4c,
        0x98638bb83592844a,
        0xdd2c3cb0e15663aa,
        0x0ef4b01c44becd36,
    ])),
    Felt::new(BigInteger384([
        0x08db0fcf6c4b8dde,
        0xc2a195af50242d1a,
        0xa85e363998db58f0,
        0xbe468471b6ed37db,
        0xc88d958f2f5dc602,
        0x1208ca208a34bf63,
    ])),
    Felt::new(BigInteger384([
        0xbc585312cfd38e97,
        0x261fcc33b76a8335,
        0x924c4402ae766e84,
        0x7d3cc3843d5abb75,
        0x28606f15f50717c0,
        0x19bbbd8d6982a5c1,
    ])),
    Felt::new(BigInteger384([
        0x041b6b988c893b25,
        0x429d4bcf47f3d128,
        0x5925471329191ea0,
        0xcf1f5f4199430bf8,
        0x4c9fecc2ff6ad097,
        0x13cdc5feeb62d1ee,
    ])),
    Felt::new(BigInteger384([
        0x7b7a27016674ae38,
        0xc357c349616224c3,
        0x2114788ab45a2248,
        0x5102a4f24e373085,
        0x9d458799a8bf079a,
        0x14b5713b6fc8feb6,
    ])),
    Felt::new(BigInteger384([
        0xb20710c6b61713bf,
        0x8a4d5a00813b4c13,
        0x0b61b46904e652ac,
        0x55ab8580c0426531,
        0x2d77cfb1b92db6bb,
        0x06dc21ac79425615,
    ])),
    Felt::new(BigInteger384([
        0x00375272078a84f7,
        0x44a96c5e53e10731,
        0x15237560088c5469,
        0xce3b582f4acf1cbc,
        0xe8ba11de880bdba2,
        0x14877ab5da8d8068,
    ])),
];

/// Additive round constants D for Anemoi.
pub(crate) const D: [Felt; NUM_HASH_ROUNDS] = [
    Felt::new(BigInteger384([
        0xb35300000070ff8f,
        0x623c01bb2dc4005c,
        0x5a5914d953aa0e2b,
        0xf606fef186c82c8a,
        0x8a60e9a4e2ca1ec5,
        0x109546d5dba1a1ae,
    ])),
    Felt::new(BigInteger384([
        0x0edfd8aed14d0eee,
        0x4c7b3e49117fb384,
        0x9190d375aab1c894,
        0x5435c20ee7d640b1,
        0xd761c6e55eaf83ed,
        0x12b482e32d994227,
    ])),
    Felt::new(BigInteger384([
        0xc3b6cc6ac460a022,
        0x33b6d050390ca37d,
        0xc7aa0ed3955e9331,
        0xd7baa33253367979,
        0x72c71fdee90300e5,
        0x06e787b65f0cc438,
    ])),
    Felt::new(BigInteger384([
        0xdf0be418b6896b42,
        0x198729d410874d6d,
        0xdcb6cbaa24bfa62f,
        0xb956d82a9c0f6c14,
        0x0f74e838700ce8d9,
        0x15ce449721dac59b,
    ])),
    Felt::new(BigInteger384([
        0x13c51b58762fd55d,
        0xc3f606e37698802e,
        0xe9b046d1d108470c,
        0xc8392d18cc8c585f,
        0xb7950454f1315b70,
        0x0d9c127033530639,
    ])),
    Felt::new(BigInteger384([
        0x97e30dd5766cd52b,
        0x688796158c8e7c75,
        0x949dbff26481cfa7,
        0xe03ce17569f74fee,
        0x6271027b9f20ca9d,
        0x17ace94ee4fbe578,
    ])),
    Felt::new(BigInteger384([
        0x4525df0da55703d6,
        0x85b8dce787244883,
        0x9502c02e3d3f66de,
        0x8b203eb5fd5fa145,
        0x4f06439f378c9cfb,
        0x0d8976760fd1ca51,
    ])),
    Felt::new(BigInteger384([
        0x2e8317697e8b6706,
        0x39ab42ba083f5c4a,
        0x609389e3b7c9a834,
        0x254adc127a0aedb8,
        0x3c19a386dcd0505f,
        0x0f4a62f3881118a5,
    ])),
    Felt::new(BigInteger384([
        0x9a098a2d602289d6,
        0x7ba26524bbe1e5e8,
        0xb261465b415562dc,
        0xfab789eac1fe80de,
        0x64811fa26204834c,
        0x06a949c0aa9bb083,
    ])),
    Felt::new(BigInteger384([
        0x43b3ff031221a4f3,
        0xb9cdc9b4d7d468b5,
        0xa0b3b117183bb241,
        0x4e6c9a6a99587f8f,
        0x02dac9201be99ea1,
        0x02462176aff78b2a,
    ])),
    Felt::new(BigInteger384([
        0xbaf7d8bc5ba7297a,
        0xb57d81f53aeec342,
        0x1a8037194545e1f9,
        0xe8d9ccfced7cb40c,
        0xba40ab707f8d639e,
        0x028de20f08b4f146,
    ])),
    Felt::new(BigInteger384([
        0x260a92001e59e9b9,
        0xdc2ff74a6f0ce95a,
        0x26fcbd7a4015358c,
        0xcdc7700834b14e1d,
        0xa3e86b47f253c68f,
        0x06ccb1287c06f3d2,
    ])),
    Felt::new(BigInteger384([
        0xece5834812aadf17,
        0x51fe2f0d8ead9756,
        0xca72ffc9909d44f6,
        0xf2681e718ccbfbb7,
        0x78e8b18b481b4c3b,
        0x027d887050a0dbe2,
    ])),
    Felt::new(BigInteger384([
        0x4ceca524fa99fe91,
        0xae9ac237d73b90d3,
        0x54cc06f376136fec,
        0x038fb4403aa690f5,
        0x1a6170dfce6c9a62,
        0x013bcd7163aa64eb,
    ])),
    Felt::new(BigInteger384([
        0xb80c73aa3811622f,
        0xf6bd9715221e74ce,
        0xad3f171aa938d5f9,
        0x94a2a717c7b4fd7f,
        0x9df68e169aa52750,
        0x15697484e5e55082,
    ])),
    Felt::new(BigInteger384([
        0xbc38024bf4e4a89a,
        0x94e04a94bc7937fe,
        0x773ab50ca134ef65,
        0x5376c02740ef99a2,
        0xa118eaf2f31cb27c,
        0x0bced46a044dd940,
    ])),
    Felt::new(BigInteger384([
        0x6ea1e2c19ca402c6,
        0x12d27016ddfa37ae,
        0xc7f795aef539207e,
        0xb0ffe10ef254a666,
        0x576bb57b2616b7a4,
        0x070dfb659aff6a3e,
    ])),
    Felt::new(BigInteger384([
        0x7493a3dc01f029f0,
        0x170a64d38e3e8abe,
        0x86e8fb3c43f77749,
        0x27d5e66434ce65c6,
        0xbaac19cab24a2454,
        0x06cad8c7f0b122c7,
    ])),
    Felt::new(BigInteger384([
        0x8f67ee83ebf9f5b1,
        0x9a41596e7111e930,
        0xd102e32b7b688f54,
        0x595c699ca70c12e0,
        0x826a5e2f5de83c9d,
        0x06f4f2aae4a0014d,
    ])),
];
