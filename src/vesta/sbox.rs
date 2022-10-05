use super::BigInteger256;
use super::Felt;

use ark_ff::Field;

#[allow(unused)]
/// Exponent of the Anemoi S-Box
pub(crate) const ALPHA: u32 = 5;

#[allow(unused)]
/// Inverse exponent
pub(crate) const INV_ALPHA: [u64; 4] = [
    0xd69f2280cccccccd,
    0x4e9ee0c9a143ba4a,
    0x3333333333333333,
    0x3333333333333333,
];

/// Multiplier of the Anemoi S-Box
#[allow(unused)]
pub(crate) const BETA: u32 = 5;

/// First added constant of the Anemoi S-Box
pub(crate) const DELTA: Felt = Felt::new(BigInteger256([
    0x123bd95299999999,
    0xb83c0a9bfa40677b,
    0xcccccccccccccccc,
    0x0ccccccccccccccc,
]));

#[allow(unused)]
/// Second added constant of the Anemoi S-Box
pub(crate) const QUAD: u32 = 2;

#[inline(always)]
pub(crate) fn exp_inv_alpha(x: &Felt) -> Felt {
    let t2 = x.square(); //       1: 2
    let t0 = t2 * x; //           2: 3
    let t1 = t2.square(); //      3: 4
    let t0 = t1 * t0; //          4: 7
    let t3 = t0 * t2; //          5: 9
    let t1 = t3 * t1; //          6: 13
    let t0 = t1 * t0; //          7: 20
    let t11 = t0 * t3; //         8: 29
    let t3 = t11 * t3; //         9: 38
    let t6 = t3 * x; //          10: 39
    let t7 = t6 * t1; //         11: 52
    let t4 = t7 * t0; //         12: 72
    let t5 = t4 * t2; //         13: 74
    let t0 = t5 * t0; //         14: 94
    let t2 = t0 * t2; //         15: 96
    let t7 = t7.square(); //     16: 104
    let t8 = t0 * t6; //         17: 133
    let t12 = t8 * t4; //        18: 205
    let t4 = t7.square(); //     19: 208
    let t10 = t4 * t6; //        20: 247
    let t6 = t12 * t0; //        21: 299
    let t9 = t6 * t7; //         22: 403
    let t7 = t9 * t5; //         23: 477
    let t3 = t7 * t3; //         24: 515
    let t5 = t3 * t4; //         25: 723
    let t2 = t5 * t2; //         26: 819
    let t4 = t2 * t0; //         27: 913
    let mut t0 = t2.square(); // 28: 1638
    t0 = t0.square(); //         29: 3276
    t0 = t0.square(); //         30: 6552
    t0 = t0.square(); //         31: 13104
    t0 = t0.square(); //         32: 26208
    t0 = t0.square(); //         33: 52416
    t0 = t0.square(); //         34: 104832
    t0 = t0.square(); //         35: 209664
    t0 = t0.square(); //         36: 419328
    t0 = t0.square(); //         37: 838656
    t0 = t0.square(); //         38: 1677312
    t0 = t0.square(); //         39: 3354624
    t0 *= t2; //                 40: 3355443
    t0 = t0.square(); //         41: 6710886
    t0 = t0.square(); //         42: 13421772
    t0 = t0.square(); //         43: 26843544
    t0 = t0.square(); //         44: 53687088
    t0 = t0.square(); //         45: 107374176
    t0 = t0.square(); //         46: 214748352
    t0 = t0.square(); //         47: 429496704
    t0 = t0.square(); //         48: 858993408
    t0 = t0.square(); //         49: 1717986816
    t0 = t0.square(); //         50: 3435973632
    t0 = t0.square(); //         51: 6871947264
    t0 = t0.square(); //         52: 13743894528
    t0 *= t2; //                 53: 13743895347
    t0 = t0.square(); //         54: 27487790694
    t0 = t0.square(); //         55: 54975581388
    t0 = t0.square(); //         56: 109951162776
    t0 = t0.square(); //         57: 219902325552
    t0 = t0.square(); //         58: 439804651104
    t0 = t0.square(); //         59: 879609302208
    t0 = t0.square(); //         60: 1759218604416
    t0 = t0.square(); //         61: 3518437208832
    t0 = t0.square(); //         62: 7036874417664
    t0 = t0.square(); //         63: 14073748835328
    t0 = t0.square(); //         64: 28147497670656
    t0 = t0.square(); //         65: 56294995341312
    t0 *= t2; //                 66: 56294995342131
    t0 = t0.square(); //         67: 112589990684262
    t0 = t0.square(); //         68: 225179981368524
    t0 = t0.square(); //         69: 450359962737048
    t0 = t0.square(); //         70: 900719925474096
    t0 = t0.square(); //         71: 1801439850948192
    t0 = t0.square(); //         72: 3602879701896384
    t0 = t0.square(); //         73: 7205759403792768
    t0 = t0.square(); //         74: 14411518807585536
    t0 = t0.square(); //         75: 28823037615171072
    t0 = t0.square(); //         76: 57646075230342144
    t0 = t0.square(); //         77: 115292150460684288
    t0 = t0.square(); //         78: 230584300921368576
    t0 *= t2; //                 79: 230584300921369395
    t0 = t0.square(); //         80: 461168601842738790
    t0 = t0.square(); //         81: 922337203685477580
    t0 = t0.square(); //         82: 1844674407370955160
    t0 = t0.square(); //         83: 3689348814741910320
    t0 = t0.square(); //         84: 7378697629483820640
    t0 = t0.square(); //         85: 14757395258967641280
    t0 = t0.square(); //         86: 29514790517935282560
    t0 = t0.square(); //         87: 59029581035870565120
    t0 = t0.square(); //         88: 118059162071741130240
    t0 = t0.square(); //         89: 236118324143482260480
    t0 = t0.square(); //         90: 472236648286964520960
    t0 = t0.square(); //         91: 944473296573929041920
    t0 *= t2; //                 92: 944473296573929042739
    t0 = t0.square(); //         93: 1888946593147858085478
    t0 = t0.square(); //         94: 3777893186295716170956
    t0 = t0.square(); //         95: 7555786372591432341912
    t0 = t0.square(); //         96: 15111572745182864683824
    t0 = t0.square(); //         97: 30223145490365729367648
    t0 = t0.square(); //         98: 60446290980731458735296
    t0 = t0.square(); //         99: 120892581961462917470592
    t0 = t0.square(); //        100: 241785163922925834941184
    t0 = t0.square(); //        101: 483570327845851669882368
    t0 = t0.square(); //        102: 967140655691703339764736
    t0 = t0.square(); //        103: 1934281311383406679529472
    t0 = t0.square(); //        104: 3868562622766813359058944
    t0 *= t2; //                105: 3868562622766813359059763
    t0 = t0.square(); //        106: 7737125245533626718119526
    t0 = t0.square(); //        107: 15474250491067253436239052
    t0 = t0.square(); //        108: 30948500982134506872478104
    t0 = t0.square(); //        109: 61897001964269013744956208
    t0 = t0.square(); //        110: 123794003928538027489912416
    t0 = t0.square(); //        111: 247588007857076054979824832
    t0 = t0.square(); //        112: 495176015714152109959649664
    t0 = t0.square(); //        113: 990352031428304219919299328
    t0 = t0.square(); //        114: 1980704062856608439838598656
    t0 = t0.square(); //        115: 3961408125713216879677197312
    t0 = t0.square(); //        116: 7922816251426433759354394624
    t0 = t0.square(); //        117: 15845632502852867518708789248
    t0 *= t2; //                118: 15845632502852867518708790067
    t0 = t0.square(); //        119: 31691265005705735037417580134
    t0 = t0.square(); //        120: 63382530011411470074835160268
    t0 = t0.square(); //        121: 126765060022822940149670320536
    t0 = t0.square(); //        122: 253530120045645880299340641072
    t0 = t0.square(); //        123: 507060240091291760598681282144
    t0 = t0.square(); //        124: 1014120480182583521197362564288
    t0 = t0.square(); //        125: 2028240960365167042394725128576
    t0 = t0.square(); //        126: 4056481920730334084789450257152
    t0 = t0.square(); //        127: 8112963841460668169578900514304
    t0 = t0.square(); //        128: 16225927682921336339157801028608
    t0 = t0.square(); //        129: 32451855365842672678315602057216
    t0 = t0.square(); //        130: 64903710731685345356631204114432
    t0 *= t2; //                131: 64903710731685345356631204115251
    t0 = t0.square(); //        132: 129807421463370690713262408230502
    t0 = t0.square(); //        133: 259614842926741381426524816461004
    t0 = t0.square(); //        134: 519229685853482762853049632922008
    t0 = t0.square(); //        135: 1038459371706965525706099265844016
    t0 = t0.square(); //        136: 2076918743413931051412198531688032
    t0 = t0.square(); //        137: 4153837486827862102824397063376064
    t0 = t0.square(); //        138: 8307674973655724205648794126752128
    t0 = t0.square(); //        139: 16615349947311448411297588253504256
    t0 = t0.square(); //        140: 33230699894622896822595176507008512
    t0 = t0.square(); //        141: 66461399789245793645190353014017024
    t0 = t0.square(); //        142: 132922799578491587290380706028034048
    t0 = t0.square(); //        143: 265845599156983174580761412056068096
    t0 *= t2; //                144: 265845599156983174580761412056068915
    t0 = t0.square(); //        145: 531691198313966349161522824112137830
    t0 = t0.square(); //        146: 1063382396627932698323045648224275660
    t0 = t0.square(); //        147: 2126764793255865396646091296448551320
    t0 = t0.square(); //        148: 4253529586511730793292182592897102640
    t0 = t0.square(); //        149: 8507059173023461586584365185794205280
    t0 = t0.square(); //        150: 17014118346046923173168730371588410560
    t0 = t0.square(); //        151: 34028236692093846346337460743176821120
    t0 = t0.square(); //        152: 68056473384187692692674921486353642240
    t0 = t0.square(); //        153: 136112946768375385385349842972707284480
    t0 = t0.square(); //        154: 272225893536750770770699685945414568960
    t0 *= t12; //               155: 272225893536750770770699685945414569165
    t0 = t0.square(); //        156: 544451787073501541541399371890829138330
    t0 = t0.square(); //        157: 1088903574147003083082798743781658276660
    t0 = t0.square(); //        158: 2177807148294006166165597487563316553320
    t0 = t0.square(); //        159: 4355614296588012332331194975126633106640
    t0 = t0.square(); //        160: 8711228593176024664662389950253266213280
    t0 = t0.square(); //        161: 17422457186352049329324779900506532426560
    t0 = t0.square(); //        162: 34844914372704098658649559801013064853120
    t0 *= t11; //               163: 34844914372704098658649559801013064853149
    t0 = t0.square(); //        164: 69689828745408197317299119602026129706298
    t0 = t0.square(); //        165: 139379657490816394634598239204052259412596
    t0 = t0.square(); //        166: 278759314981632789269196478408104518825192
    t0 = t0.square(); //        167: 557518629963265578538392956816209037650384
    t0 = t0.square(); //        168: 1115037259926531157076785913632418075300768
    t0 = t0.square(); //        169: 2230074519853062314153571827264836150601536
    t0 = t0.square(); //        170: 4460149039706124628307143654529672301203072
    t0 = t0.square(); //        171: 8920298079412249256614287309059344602406144
    t0 = t0.square(); //        172: 17840596158824498513228574618118689204812288
    t0 = t0.square(); //        173: 35681192317648997026457149236237378409624576
    t0 *= t10; //               174: 35681192317648997026457149236237378409624823
    t0 = t0.square(); //        175: 71362384635297994052914298472474756819249646
    t0 = t0.square(); //        176: 142724769270595988105828596944949513638499292
    t0 = t0.square(); //        177: 285449538541191976211657193889899027276998584
    t0 = t0.square(); //        178: 570899077082383952423314387779798054553997168
    t0 = t0.square(); //        179: 1141798154164767904846628775559596109107994336
    t0 = t0.square(); //        180: 2283596308329535809693257551119192218215988672
    t0 = t0.square(); //        181: 4567192616659071619386515102238384436431977344
    t0 = t0.square(); //        182: 9134385233318143238773030204476768872863954688
    t0 = t0.square(); //        183: 18268770466636286477546060408953537745727909376
    t0 = t0.square(); //        184: 36537540933272572955092120817907075491455818752
    t0 = t0.square(); //        185: 73075081866545145910184241635814150982911637504
    t0 = t0.square(); //        186: 146150163733090291820368483271628301965823275008
    t0 = t0.square(); //        187: 292300327466180583640736966543256603931646550016
    t0 = t0.square(); //        188: 584600654932361167281473933086513207863293100032
    t0 *= t9; //                189: 584600654932361167281473933086513207863293100435
    t0 = t0.square(); //        190: 1169201309864722334562947866173026415726586200870
    t0 = t0.square(); //        191: 2338402619729444669125895732346052831453172401740
    t0 = t0.square(); //        192: 4676805239458889338251791464692105662906344803480
    t0 = t0.square(); //        193: 9353610478917778676503582929384211325812689606960
    t0 = t0.square(); //        194: 18707220957835557353007165858768422651625379213920
    t0 = t0.square(); //        195: 37414441915671114706014331717536845303250758427840
    t0 = t0.square(); //        196: 74828883831342229412028663435073690606501516855680
    t0 = t0.square(); //        197: 149657767662684458824057326870147381213003033711360
    t0 = t0.square(); //        198: 299315535325368917648114653740294762426006067422720
    t0 *= t8; //                199: 299315535325368917648114653740294762426006067422853
    t0 = t0.square(); //        200: 598631070650737835296229307480589524852012134845706
    t0 = t0.square(); //        201: 1197262141301475670592458614961179049704024269691412
    t0 = t0.square(); //        202: 2394524282602951341184917229922358099408048539382824
    t0 = t0.square(); //        203: 4789048565205902682369834459844716198816097078765648
    t0 = t0.square(); //        204: 9578097130411805364739668919689432397632194157531296
    t0 = t0.square(); //        205: 19156194260823610729479337839378864795264388315062592
    t0 = t0.square(); //        206: 38312388521647221458958675678757729590528776630125184
    t0 = t0.square(); //        207: 76624777043294442917917351357515459181057553260250368
    t0 = t0.square(); //        208: 153249554086588885835834702715030918362115106520500736
    t0 = t0.square(); //        209: 306499108173177771671669405430061836724230213041001472
    t0 = t0.square(); //        210: 612998216346355543343338810860123673448460426082002944
    t0 = t0.square(); //        211: 1225996432692711086686677621720247346896920852164005888
    t0 = t0.square(); //        212: 2451992865385422173373355243440494693793841704328011776
    t0 *= t7; //                213: 2451992865385422173373355243440494693793841704328012253
    t0 = t0.square(); //        214: 4903985730770844346746710486880989387587683408656024506
    t0 = t0.square(); //        215: 9807971461541688693493420973761978775175366817312049012
    t0 = t0.square(); //        216: 19615942923083377386986841947523957550350733634624098024
    t0 = t0.square(); //        217: 39231885846166754773973683895047915100701467269248196048
    t0 = t0.square(); //        218: 78463771692333509547947367790095830201402934538496392096
    t0 = t0.square(); //        219: 156927543384667019095894735580191660402805869076992784192
    t0 = t0.square(); //        220: 313855086769334038191789471160383320805611738153985568384
    t0 = t0.square(); //        221: 627710173538668076383578942320766641611223476307971136768
    t0 = t0.square(); //        222: 1255420347077336152767157884641533283222446952615942273536
    t0 = t0.square(); //        223: 2510840694154672305534315769283066566444893905231884547072
    t0 = t0.square(); //        224: 5021681388309344611068631538566133132889787810463769094144
    t0 *= t6; //                225: 5021681388309344611068631538566133132889787810463769094443
    t0 = t0.square(); //        226: 10043362776618689222137263077132266265779575620927538188886
    t0 = t0.square(); //        227: 20086725553237378444274526154264532531559151241855076377772
    t0 = t0.square(); //        228: 40173451106474756888549052308529065063118302483710152755544
    t0 = t0.square(); //        229: 80346902212949513777098104617058130126236604967420305511088
    t0 = t0.square(); //        230: 160693804425899027554196209234116260252473209934840611022176
    t0 = t0.square(); //        231: 321387608851798055108392418468232520504946419869681222044352
    t0 = t0.square(); //        232: 642775217703596110216784836936465041009892839739362444088704
    t0 = t0.square(); //        233: 1285550435407192220433569673872930082019785679478724888177408
    t0 = t0.square(); //        234: 2571100870814384440867139347745860164039571358957449776354816
    t0 = t0.square(); //        235: 5142201741628768881734278695491720328079142717914899552709632
    t0 = t0.square(); //        236: 10284403483257537763468557390983440656158285435829799105419264
    t0 *= t5; //                237: 10284403483257537763468557390983440656158285435829799105419987
    t0 = t0.square(); //        238: 20568806966515075526937114781966881312316570871659598210839974
    t0 = t0.square(); //        239: 41137613933030151053874229563933762624633141743319196421679948
    t0 = t0.square(); //        240: 82275227866060302107748459127867525249266283486638392843359896
    t0 = t0.square(); //        241: 164550455732120604215496918255735050498532566973276785686719792
    t0 = t0.square(); //        242: 329100911464241208430993836511470100997065133946553571373439584
    t0 = t0.square(); //        243: 658201822928482416861987673022940201994130267893107142746879168
    t0 = t0.square(); //        244: 1316403645856964833723975346045880403988260535786214285493758336
    t0 = t0.square(); //        245: 2632807291713929667447950692091760807976521071572428570987516672
    t0 = t0.square(); //        246: 5265614583427859334895901384183521615953042143144857141975033344
    t0 = t0.square(); //        247: 10531229166855718669791802768367043231906084286289714283950066688
    t0 *= t4; //                248: 10531229166855718669791802768367043231906084286289714283950067601
    t0 = t0.square(); //        249: 21062458333711437339583605536734086463812168572579428567900135202
    t0 = t0.square(); //        250: 42124916667422874679167211073468172927624337145158857135800270404
    t0 = t0.square(); //        251: 84249833334845749358334422146936345855248674290317714271600540808
    t0 = t0.square(); //        252: 168499666669691498716668844293872691710497348580635428543201081616
    t0 = t0.square(); //        253: 336999333339382997433337688587745383420994697161270857086402163232
    t0 = t0.square(); //        254: 673998666678765994866675377175490766841989394322541714172804326464
    t0 = t0.square(); //        255: 1347997333357531989733350754350981533683978788645083428345608652928
    t0 = t0.square(); //        256: 2695994666715063979466701508701963067367957577290166856691217305856
    t0 = t0.square(); //        257: 5391989333430127958933403017403926134735915154580333713382434611712
    t0 = t0.square(); //        258: 10783978666860255917866806034807852269471830309160667426764869223424
    t0 = t0.square(); //        259: 21567957333720511835733612069615704538943660618321334853529738446848
    t0 *= t3; //                260: 21567957333720511835733612069615704538943660618321334853529738447363
    t0 = t0.square(); //        261: 43135914667441023671467224139231409077887321236642669707059476894726
    t0 = t0.square(); //        262: 86271829334882047342934448278462818155774642473285339414118953789452
    t0 = t0.square(); //        263: 172543658669764094685868896556925636311549284946570678828237907578904
    t0 = t0.square(); //        264: 345087317339528189371737793113851272623098569893141357656475815157808
    t0 = t0.square(); //        265: 690174634679056378743475586227702545246197139786282715312951630315616
    t0 = t0.square(); //        266: 1380349269358112757486951172455405090492394279572565430625903260631232
    t0 = t0.square(); //        267: 2760698538716225514973902344910810180984788559145130861251806521262464
    t0 = t0.square(); //        268: 5521397077432451029947804689821620361969577118290261722503613042524928
    t0 = t0.square(); //        269: 11042794154864902059895609379643240723939154236580523445007226085049856
    t0 = t0.square(); //        270: 22085588309729804119791218759286481447878308473161046890014452170099712
    t0 = t0.square(); //        271: 44171176619459608239582437518572962895756616946322093780028904340199424
    t0 = t0.square(); //        272: 88342353238919216479164875037145925791513233892644187560057808680398848
    t0 *= t2; //                273: 88342353238919216479164875037145925791513233892644187560057808680399667
    t0 = t0.square(); //        274: 176684706477838432958329750074291851583026467785288375120115617360799334
    t0 = t0.square(); //        275: 353369412955676865916659500148583703166052935570576750240231234721598668
    t0 = t0.square(); //        276: 706738825911353731833319000297167406332105871141153500480462469443197336
    t0 = t0.square(); //        277: 1413477651822707463666638000594334812664211742282307000960924938886394672
    t0 = t0.square(); //        278: 2826955303645414927333276001188669625328423484564614001921849877772789344
    t0 = t0.square(); //        279: 5653910607290829854666552002377339250656846969129228003843699755545578688
    t0 = t0.square(); //        280: 11307821214581659709333104004754678501313693938258456007687399511091157376
    t0 = t0.square(); //        281: 22615642429163319418666208009509357002627387876516912015374799022182314752
    t0 = t0.square(); //        282: 45231284858326638837332416019018714005254775753033824030749598044364629504
    t0 = t0.square(); //        283: 90462569716653277674664832038037428010509551506067648061499196088729259008
    t0 = t0.square(); //        284: 180925139433306555349329664076074856021019103012135296122998392177458518016
    t0 = t0.square(); //        285: 361850278866613110698659328152149712042038206024270592245996784354917036032
    t0 *= t2; //                286: 361850278866613110698659328152149712042038206024270592245996784354917036851
    t0 = t0.square(); //        287: 723700557733226221397318656304299424084076412048541184491993568709834073702
    t0 = t0.square(); //        288: 1447401115466452442794637312608598848168152824097082368983987137419668147404
    t0 = t0.square(); //        289: 2894802230932904885589274625217197696336305648194164737967974274839336294808
    t0 = t0.square(); //        290: 5789604461865809771178549250434395392672611296388329475935948549678672589616
    t0 = t0.square(); //        291: 11579208923731619542357098500868790785345222592776658951871897099357345179232
    t0 = t0.square(); //        292: 23158417847463239084714197001737581570690445185553317903743794198714690358464
    t0 * t1 //                  293: 23158417847463239084714197001737581570690445185553317903743794198714690358477
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_ff::One;

    #[test]
    fn test_alpha() {
        let mut a = -Felt::one();
        for _ in 0..100 {
            assert_eq!(exp_inv_alpha(&a), a.pow(INV_ALPHA));
            a += a;
        }
    }
}
