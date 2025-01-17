use crate::lookup::MagicLookup;

#[rustfmt::skip]
pub const ROOK_MAGICS: [MagicLookup; 64] = [
    MagicLookup { magic: 17539749073859413632, length: 13, offset: 0 },
    MagicLookup { magic: 18428168386530501225, length: 12, offset: 8192 },
    MagicLookup { magic: 12702083310888480839, length: 12, offset: 12288 },
    MagicLookup { magic: 3846109785921932284, length: 12, offset: 16384 },
    MagicLookup { magic: 16861490204039056361, length: 12, offset: 20480 },
    MagicLookup { magic: 9285291086477813540, length: 12, offset: 24576 },
    MagicLookup { magic: 7429954234262713712, length: 12, offset: 28672 },
    MagicLookup { magic: 3329005740130798227, length: 13, offset: 32768 },
    MagicLookup { magic: 1298725636013758581, length: 12, offset: 40960 },
    MagicLookup { magic: 9193095890061664258, length: 11, offset: 45056 },
    MagicLookup { magic: 1319328018402494852, length: 11, offset: 47104 },
    MagicLookup { magic: 2939137941690521164, length: 11, offset: 49152 },
    MagicLookup { magic: 6064108122372835925, length: 11, offset: 51200 },
    MagicLookup { magic: 13055278107152740950, length: 11, offset: 53248 },
    MagicLookup { magic: 10035146660281323534, length: 10, offset: 55296 },
    MagicLookup { magic: 3335759964198617856, length: 11, offset: 56320 },
    MagicLookup { magic: 1004122965041884545, length: 12, offset: 58368 },
    MagicLookup { magic: 4403068103255187912, length: 11, offset: 62464 },
    MagicLookup { magic: 16208349368206360414, length: 11, offset: 64512 },
    MagicLookup { magic: 6168229617208161888, length: 11, offset: 66560 },
    MagicLookup { magic: 4016775631525709824, length: 11, offset: 68608 },
    MagicLookup { magic: 9336123771319246053, length: 11, offset: 70656 },
    MagicLookup { magic: 16811827459153922600, length: 10, offset: 72704 },
    MagicLookup { magic: 15792393262197490695, length: 11, offset: 73728 },
    MagicLookup { magic: 3330302892402002429, length: 12, offset: 75776 },
    MagicLookup { magic: 15617419618970615812, length: 11, offset: 79872 },
    MagicLookup { magic: 5005252727203154999, length: 11, offset: 81920 },
    MagicLookup { magic: 6014342957716807168, length: 10, offset: 83968 },
    MagicLookup { magic: 16547418057126236375, length: 11, offset: 84992 },
    MagicLookup { magic: 9677004249293159488, length: 10, offset: 87040 },
    MagicLookup { magic: 16799966458950257200, length: 10, offset: 88064 },
    MagicLookup { magic: 3760757348168369551, length: 11, offset: 89088 },
    MagicLookup { magic: 17903314760131816170, length: 12, offset: 91136 },
    MagicLookup { magic: 14350903318476798948, length: 11, offset: 95232 },
    MagicLookup { magic: 4735014135806558311, length: 11, offset: 97280 },
    MagicLookup { magic: 2657793590518776756, length: 11, offset: 99328 },
    MagicLookup { magic: 10407513077741261344, length: 10, offset: 101376 },
    MagicLookup { magic: 7781325019400924169, length: 11, offset: 102400 },
    MagicLookup { magic: 967948822200193046, length: 10, offset: 104448 },
    MagicLookup { magic: 16990689051604418891, length: 11, offset: 105472 },
    MagicLookup { magic: 13943104455253819376, length: 12, offset: 107520 },
    MagicLookup { magic: 16726374042622762720, length: 11, offset: 111616 },
    MagicLookup { magic: 4020995869991312101, length: 11, offset: 113664 },
    MagicLookup { magic: 6443816560033073312, length: 11, offset: 115712 },
    MagicLookup { magic: 12813145568846974906, length: 11, offset: 117760 },
    MagicLookup { magic: 10290579580686138261, length: 11, offset: 119808 },
    MagicLookup { magic: 8425920614473924656, length: 10, offset: 121856 },
    MagicLookup { magic: 8404627409860952091, length: 11, offset: 122880 },
    MagicLookup { magic: 6760168650055185920, length: 11, offset: 124928 },
    MagicLookup { magic: 2862249266461106688, length: 10, offset: 126976 },
    MagicLookup { magic: 6292234918110093824, length: 10, offset: 128000 },
    MagicLookup { magic: 7559003079236952576, length: 10, offset: 129024 },
    MagicLookup { magic: 3317000373142597633, length: 11, offset: 130048 },
    MagicLookup { magic: 3002202615898640888, length: 11, offset: 132096 },
    MagicLookup { magic: 17426459281259111424, length: 10, offset: 134144 },
    MagicLookup { magic: 3500935622424859136, length: 11, offset: 135168 },
    MagicLookup { magic: 17173581100111364293, length: 12, offset: 137216 },
    MagicLookup { magic: 3398494792583266694, length: 11, offset: 141312 },
    MagicLookup { magic: 16424199035247690098, length: 11, offset: 143360 },
    MagicLookup { magic: 7127372050157055082, length: 12, offset: 145408 },
    MagicLookup { magic: 12730666794481791482, length: 12, offset: 149504 },
    MagicLookup { magic: 8578794375355328282, length: 11, offset: 153600 },
    MagicLookup { magic: 16560931033644437548, length: 11, offset: 155648 },
    MagicLookup { magic: 1821617319595737974, length: 12, offset: 157696 },
];

#[rustfmt::skip]
pub const BISHOP_MAGICS: [MagicLookup; 64] = [
    MagicLookup { magic: 1366898566665339138, length: 6, offset: 0 },
    MagicLookup { magic: 4830191987871583564, length: 5, offset: 64 },
    MagicLookup { magic: 16397677352899150309, length: 5, offset: 96 },
    MagicLookup { magic: 4451843696377169575, length: 5, offset: 128 },
    MagicLookup { magic: 10282857188858408109, length: 5, offset: 160 },
    MagicLookup { magic: 2595485845829137617, length: 5, offset: 192 },
    MagicLookup { magic: 2061523843168719409, length: 5, offset: 224 },
    MagicLookup { magic: 1481255985310801776, length: 6, offset: 256 },
    MagicLookup { magic: 15921971394886510728, length: 5, offset: 320 },
    MagicLookup { magic: 15270357581212025218, length: 5, offset: 352 },
    MagicLookup { magic: 7000159543455926366, length: 5, offset: 384 },
    MagicLookup { magic: 14452653891298651526, length: 5, offset: 416 },
    MagicLookup { magic: 18400876866511362692, length: 5, offset: 448 },
    MagicLookup { magic: 8970147934190991857, length: 5, offset: 480 },
    MagicLookup { magic: 16931357342619156650, length: 5, offset: 512 },
    MagicLookup { magic: 4537329897815613641, length: 5, offset: 544 },
    MagicLookup { magic: 14296756457248662553, length: 5, offset: 576 },
    MagicLookup { magic: 8429682354882616653, length: 5, offset: 608 },
    MagicLookup { magic: 94580505624233968, length: 7, offset: 640 },
    MagicLookup { magic: 16449407002020532232, length: 7, offset: 768 },
    MagicLookup { magic: 5896340019757713921, length: 7, offset: 896 },
    MagicLookup { magic: 11942139000137320291, length: 7, offset: 1024 },
    MagicLookup { magic: 17745320328850324510, length: 5, offset: 1152 },
    MagicLookup { magic: 11142470959473894487, length: 5, offset: 1184 },
    MagicLookup { magic: 17307403831492776218, length: 5, offset: 1216 },
    MagicLookup { magic: 18399790855240030760, length: 5, offset: 1248 },
    MagicLookup { magic: 768866500869834448, length: 7, offset: 1280 },
    MagicLookup { magic: 6717006759734514951, length: 10, offset: 1408 },
    MagicLookup { magic: 10652574429445382146, length: 9, offset: 2432 },
    MagicLookup { magic: 4997593717075446425, length: 7, offset: 2944 },
    MagicLookup { magic: 2340503621917603710, length: 5, offset: 3072 },
    MagicLookup { magic: 3290167199918197786, length: 5, offset: 3104 },
    MagicLookup { magic: 6084407214550590628, length: 5, offset: 3136 },
    MagicLookup { magic: 17099117889692926955, length: 5, offset: 3168 },
    MagicLookup { magic: 2716198547247071619, length: 7, offset: 3200 },
    MagicLookup { magic: 16297154895310160156, length: 9, offset: 3328 },
    MagicLookup { magic: 10686205762927433856, length: 10, offset: 3840 },
    MagicLookup { magic: 4196510779984118017, length: 7, offset: 4864 },
    MagicLookup { magic: 2691283413901051954, length: 5, offset: 4992 },
    MagicLookup { magic: 16513633883365966608, length: 5, offset: 5024 },
    MagicLookup { magic: 17704824500593495255, length: 5, offset: 5056 },
    MagicLookup { magic: 1468395614774682645, length: 5, offset: 5088 },
    MagicLookup { magic: 11221160759407579139, length: 7, offset: 5120 },
    MagicLookup { magic: 13704507680420202498, length: 7, offset: 5248 },
    MagicLookup { magic: 1882070414759508994, length: 7, offset: 5376 },
    MagicLookup { magic: 10742820326401508865, length: 7, offset: 5504 },
    MagicLookup { magic: 6905157357534723041, length: 5, offset: 5632 },
    MagicLookup { magic: 618971222050478352, length: 5, offset: 5664 },
    MagicLookup { magic: 16457858467383059897, length: 5, offset: 5696 },
    MagicLookup { magic: 13926541888104957655, length: 5, offset: 5728 },
    MagicLookup { magic: 14404845883347721614, length: 5, offset: 5760 },
    MagicLookup { magic: 1128309078476963503, length: 5, offset: 5792 },
    MagicLookup { magic: 2402301147746667501, length: 5, offset: 5824 },
    MagicLookup { magic: 6892262353783690671, length: 5, offset: 5856 },
    MagicLookup { magic: 3236984310253486117, length: 5, offset: 5888 },
    MagicLookup { magic: 101674155388567759, length: 5, offset: 5920 },
    MagicLookup { magic: 10061605016418374670, length: 6, offset: 5952 },
    MagicLookup { magic: 4981570264122790094, length: 5, offset: 6016 },
    MagicLookup { magic: 16863897654955575304, length: 5, offset: 6048 },
    MagicLookup { magic: 16955468953262524469, length: 5, offset: 6080 },
    MagicLookup { magic: 5268193513159124474, length: 5, offset: 6112 },
    MagicLookup { magic: 1627423186726827527, length: 5, offset: 6144 },
    MagicLookup { magic: 2878275765051851432, length: 5, offset: 6176 },
    MagicLookup { magic: 10472127181420855554, length: 6, offset: 6208 },
];

// pub fn find_magic_number(maps: Vec<Bitboard>, index_size: u8) -> u64 {
//     loop {
//         let magic_number = rand::random();
//         let mut check_array = Vec::new();
//         let mut success = true;
//         'inner: for map in maps.iter() {
//             let magicked = map.inner().wrapping_mul(magic_number);
//             let index = magicked >> (64 - index_size as u32);
//
//             // if magic_number % 9999999 == 0 {
//             //     println!(
//             //         "{:064b}, {:064b}, {:064b}, {:b}",
//             //         magic_number,
//             //         map,
//             //         map.wrapping_mul(magic_number),
//             //         index,
//             //     );
//             //     // dbg!(magicked, magicked.leading_zeros());
//             // }
//
//             if check_array.contains(&index) {
//                 success = false;
//                 break 'inner;
//             }
//
//             check_array.push(index);
//         }
//
//         if success {
//             return magic_number;
//         }
//     }
// }
//

// fn find_all_magic_numbers() {
//     let rook_magics = Arc::new(Mutex::new(vec![None; 64]));
//     let bishop_magics = Arc::new(Mutex::new(vec![None; 64]));
//
//     let mut handles = Vec::new();
//     for i in 0..64 {
//         let rm = rook_magics.clone();
//         let bm = bishop_magics.clone();
//
//         handles.push(std::thread::spawn(move || {
//             let maps = generate_obstruction_maps(rook_occupancy_mask(i.into()));
//             let mut length = 14;
//             loop {
//                 if 1 << length < maps.len() {
//                     println!("Found best magic number for rook square {}.", i);
//                     break;
//                 }
//
//                 let magic = find_magic_number(maps.clone(), length);
//                 println!(
//                     "Magic number length {} for rook square {}: {}",
//                     length, i, magic
//                 );
//                 rm.lock().unwrap()[i as usize] = Some((magic, length));
//                 length -= 1;
//             }
//         }));
//
//         handles.push(std::thread::spawn(move || {
//             let maps = generate_obstruction_maps(bishop_occupancy_mask(i.into()));
//             let mut length = 11;
//             loop {
//                 if 1 << length < maps.len() {
//                     println!("Found best magic number for rook square {}.", i);
//                     break;
//                 }
//
//                 let magic = find_magic_number(maps.clone(), length);
//                 println!(
//                     "Magic number length {} for bishop square {}: {}",
//                     length, i, magic
//                 );
//                 bm.lock().unwrap()[i as usize] = Some((magic, length));
//                 length -= 1;
//             }
//         }));
//     }
//
//     loop {
//         let waiting_on_rooks = rook_magics
//             .lock()
//             .unwrap()
//             .iter()
//             .filter(|m| m.is_none())
//             .count();
//
//         let waiting_on_bishops = bishop_magics
//             .lock()
//             .unwrap()
//             .iter()
//             .filter(|m| m.is_none())
//             .count();
//
//         if waiting_on_bishops + waiting_on_rooks > 0 {
//             println!(
//                 "Waiting on {} rooks and {} bishops...",
//                 waiting_on_rooks, waiting_on_bishops
//             );
//         } else {
//             println!("Rook Magics:");
//             let mut rook_total_length = 0;
//             for magic in rook_magics.lock().unwrap().iter() {
//                 let Some((magic, length)) = magic else {
//                     unreachable!()
//                 };
//                 println!(
//                     "MagicNumber {{ value: {}, length: {}, offset: {} }},",
//                     magic, length, rook_total_length,
//                 );
//                 rook_total_length += 1 << length;
//             }
//             println!("Bishop Magics:");
//             let mut bishop_total_length = 0;
//             for magic in bishop_magics.lock().unwrap().iter() {
//                 let Some((magic, length)) = magic else {
//                     unreachable!()
//                 };
//                 println!(
//                     "MagicNumber {{ value: {}, length: {}, offset: {} }},",
//                     magic, length, bishop_total_length,
//                 );
//                 bishop_total_length += 1 << length;
//             }
//
//             println!(
//                 "Rook lookup array length is {} ({} kB).",
//                 rook_total_length,
//                 (rook_total_length * 8) as f32 / 1000.
//             );
//
//             println!(
//                 "Bishop lookup array length is {} ({} kB).",
//                 bishop_total_length,
//                 (bishop_total_length * 8) as f32 / 1000.
//             );
//         }
//
//         std::thread::sleep(Duration::from_secs(30));
//     }
// }
