use super::BigInteger256;
use super::Felt;
use super::NUM_HASH_ROUNDS;

/// Additive round constants C for Anemoi.
pub(crate) const C: [Felt; NUM_HASH_ROUNDS] = [
    Felt::new(BigInteger256([
        0x00000055ffffffaa,
        0xde4ae6fe008d1856,
        0xca916d4cc3a16e35,
        0x0e29ca100517e9be,
    ])),
    Felt::new(BigInteger256([
        0x39e0de29f4b12254,
        0x4542c782afdc7771,
        0x73b230448757841f,
        0x4c66a29cf76de864,
    ])),
    Felt::new(BigInteger256([
        0x3fc21fde7d7b78d8,
        0x82f0645e8f5af33c,
        0x1bb34fcb9aea8258,
        0x15a7d78b0c310f52,
    ])),
    Felt::new(BigInteger256([
        0xe7c71a3a93f9cf8d,
        0xc1a4d5f501e1e624,
        0xe0520e391ee539f2,
        0x5ce1f5aa1353f292,
    ])),
    Felt::new(BigInteger256([
        0xf468ec640f9392e9,
        0xd0b2b188f86888b7,
        0x32641a368b203537,
        0x63a9c811eda5f734,
    ])),
    Felt::new(BigInteger256([
        0xaea1c1ed34038852,
        0x8fee23ccb54da3d2,
        0x223ee1a3d2627714,
        0x0e328961d797ccde,
    ])),
    Felt::new(BigInteger256([
        0xc4534349519b429e,
        0x7afc9f1db435b2c5,
        0x90bddf0600dc3a58,
        0x73050f54a903dff4,
    ])),
    Felt::new(BigInteger256([
        0x1f77830c66fb7746,
        0xc7e57426850a228b,
        0x90c8fd2720008c56,
        0x0db8e1cc4926a688,
    ])),
    Felt::new(BigInteger256([
        0x80c25874b4297a46,
        0xaa9f314b56e67a01,
        0xc2cfcec4f5772235,
        0x19b1f168c24e4630,
    ])),
    Felt::new(BigInteger256([
        0x7179a4ce184eefbc,
        0x347ac5811ccf5b5d,
        0xc914af5ca815aa17,
        0x6a3578c4c95b5a78,
    ])),
    Felt::new(BigInteger256([
        0x794b4100dd1885cb,
        0x473889e579f746ef,
        0xd268e9e7d52bfbd4,
        0x3523006b457febe7,
    ])),
    Felt::new(BigInteger256([
        0xc55c39240eeaba51,
        0xf27f43ba16fde720,
        0xe5ff54914a397668,
        0x38718ba071c2dc66,
    ])),
    Felt::new(BigInteger256([
        0x740cea485117c11a,
        0x4d6661c82aac9ac0,
        0x4627ef655e4f1327,
        0x6a10d5663c8a24a3,
    ])),
    Felt::new(BigInteger256([
        0xf9a548af2e4428ed,
        0x7e8a037c215f10bd,
        0x8c39efe749e53632,
        0x1aa0a3d77819d265,
    ])),
    Felt::new(BigInteger256([
        0x74073016720a1350,
        0x3d923251644d8036,
        0xa9747caea29d99c7,
        0x071a6ea4666478dc,
    ])),
    Felt::new(BigInteger256([
        0xae91a1d77faf6323,
        0x1c99942a8063cf67,
        0x0a03777e699efe83,
        0x38411bddbcc55532,
    ])),
    Felt::new(BigInteger256([
        0x06cff957e7e8f721,
        0x36864d1fbcf36304,
        0xa1429057a9d4289a,
        0x4d4209ab37c8a348,
    ])),
    Felt::new(BigInteger256([
        0xef00b0381bff71d6,
        0x1bec1a892294bda2,
        0x510d3f697aaa5ee7,
        0x16a8f36730e1f797,
    ])),
    Felt::new(BigInteger256([
        0xa7f0da0e84adb570,
        0x3824e496e12e0ffc,
        0xfb70dfe5ca0fe1be,
        0x6039283ff91f9f2a,
    ])),
];

/// Additive round constants D for Anemoi.
pub(crate) const D: [Felt; NUM_HASH_ROUNDS] = [
    Felt::new(BigInteger256([
        0xdb6db731db6db685,
        0x7142c545dbfb8331,
        0x900aec24bf80a3ea,
        0x010d14a33c66b40d,
    ])),
    Felt::new(BigInteger256([
        0x85a7e15aaf2d443b,
        0x117700705caabf2b,
        0x388a69df469d9aaa,
        0x24a3065923591607,
    ])),
    Felt::new(BigInteger256([
        0x64d2592ebf0f4b82,
        0xe00a6585e62ef6c5,
        0x018a3ce5c7ba4395,
        0x58df94b12b8f8584,
    ])),
    Felt::new(BigInteger256([
        0x881777a8f8b525a4,
        0x0cecfabfd8b3dad2,
        0x8309a9d37073249b,
        0x468825ca5ab5c71d,
    ])),
    Felt::new(BigInteger256([
        0xf6f0aa2500024218,
        0x4265762a8c34c425,
        0x79d9852ea691db8b,
        0x40cff07e2f0a974f,
    ])),
    Felt::new(BigInteger256([
        0x41dad06db733ecfe,
        0xd08ca59bf4b3c172,
        0x5ea6eae943e9a327,
        0x4ca798897a1986e9,
    ])),
    Felt::new(BigInteger256([
        0xacc654df74dce163,
        0xade94d12e983c7d7,
        0xc5606bd87f206595,
        0x63f66c0a34d7ab35,
    ])),
    Felt::new(BigInteger256([
        0xda40257681aa35eb,
        0x506dc70d67f989a9,
        0x2fd11bf58d484e28,
        0x156d5f9b7f802afa,
    ])),
    Felt::new(BigInteger256([
        0x4b322f3ccd0ecf67,
        0x797ce549c39bcbef,
        0x343809622d01f632,
        0x2a0f6ef12a584db9,
    ])),
    Felt::new(BigInteger256([
        0x94d8dd7553f1e7b5,
        0x81132674f7e8767a,
        0x94adad7a6ef16db8,
        0x704845e0ca98ec0b,
    ])),
    Felt::new(BigInteger256([
        0x6fb35712e61106d3,
        0x5b6816cc8e042489,
        0x30e28a835048c5a8,
        0x5cff3fcdcf9efb5d,
    ])),
    Felt::new(BigInteger256([
        0xca468ceb9c49f34f,
        0xacd4547375fb7978,
        0x72723646766d9b70,
        0x3a4d7885d712c459,
    ])),
    Felt::new(BigInteger256([
        0xbb633db4dffe7db8,
        0x36bb142399035de3,
        0x01d18bbd42d58989,
        0x2b4cba1eae3364cd,
    ])),
    Felt::new(BigInteger256([
        0x7cb7dfe6170f2533,
        0x6264668431f21f7d,
        0x1db2258208960f22,
        0x6b161522ae6c6270,
    ])),
    Felt::new(BigInteger256([
        0xc29a141c0fb82c00,
        0xb333a6c30d4188af,
        0x814d5ff59454ff21,
        0x0132789dc94536c2,
    ])),
    Felt::new(BigInteger256([
        0x8af61c0668d61aa6,
        0xe0813377b62d328e,
        0xa44b08c61acc31e8,
        0x3475fa1554677d3d,
    ])),
    Felt::new(BigInteger256([
        0x624ba433b8ba647b,
        0xba17d48ad0e545ec,
        0x0007af245e943fc5,
        0x48b10de0932c24e3,
    ])),
    Felt::new(BigInteger256([
        0xbd3a2c24afa9f346,
        0x6822c237b2f39f09,
        0x65d4b0e7d31fce3e,
        0x0bd42e53c5fa917e,
    ])),
    Felt::new(BigInteger256([
        0xa6e9676aba54a90a,
        0x960a5f6ab82187d9,
        0x7549262feed32b37,
        0x437420ee3a3561e0,
    ])),
];
