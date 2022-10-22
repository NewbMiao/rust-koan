use itertools::Itertools;

fn main() {}
///
/// 0      4      8      
///  1   3   5   7  9
///   2        6            
//
///
/// 0      6         12      
///  1   5   7     11   13
///   2 4     8  10       14
///    3        9               

fn rails<'a>(num_rails: usize, text: &'a str) -> impl Iterator<Item = usize> + 'a {
    (0..num_rails)
        .map(move |rail| {
            let mut index = rail;
            let mut indices = vec![];
            let mut direction = rail;

            while index < text.len() {
                indices.push(index);
                index += match num_rails == direction + 1 {
                    true => 2 * (num_rails - 1),
                    false => 2 * (num_rails - 1 - direction),
                };
                direction = num_rails - 1 - direction;
            }

            indices
        })
        .flatten()
}

fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    rails(num_rails, text).map(|i| &text[i..i + 1]).collect()
}

fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    rails(num_rails, text)
        .zip(text.chars())
        .sorted_by_key(|&(i, _)| i)
        .map(|(_, v)| v)
        .collect()
}
// fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
//     if text == "" {
//         return "".to_string();
//     }
//     let mut res = repeat(vec![]).take(num_rails).collect::<Vec<Vec<char>>>();
//     let mut change_direction = false;
//     let mut i = 0;
//     text.chars().for_each(|c| {
//         res[i].push(c);
//         match change_direction {
//             false => {
//                 i += 1;

//                 if i == num_rails {
//                     change_direction = true;
//                     i -= 2;
//                 }
//             }
//             true => {
//                 if i == 0 {
//                     i += 1;
//                     change_direction = false;
//                 } else {
//                     i -= 1;
//                 }
//             }
//         }
//     });
//     res.iter().map(|v| v.iter().collect::<String>()).collect()
// }

// /// num_rails*n+(num_rails-2)*(n-1)=len-left
// /// 2*n*(num_rails-1)+2-num_rails=len-left
// fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
//     if text == "" {
//         return "".to_string();
//     }
//     let mut res = repeat(vec![]).take(num_rails).collect::<Vec<Vec<char>>>();
//     let mut change_direction = false;
//     let mut i = 0;
//     let last_n = (text.len() + num_rails - 2) / (2 * (num_rails - 1));
//     let mut left: i32 =
//         text.len() as i32 - (num_rails as i32 - 1) * last_n as i32 * 2 + num_rails as i32 - 2i32;
//     let left_num_in_half = left - num_rails as i32 + 1;

//     let mut end = text.len();
//     let mut start = 0;
//     (0..num_rails).rev().for_each(|x| match x == num_rails - 1 {
//         true => {
//             start = end - last_n;
//             res[x] = text[start..end].chars().rev().collect();
//             end = start;
//         }
//         false => {
//             if x == 0 {
//                 res[x] = text[..start].chars().rev().collect();
//                 if left > 0 {
//                     left -= 1;
//                 }
//             } else {
//                 start -= 2 * last_n - 1;
//                 if left > 0 {
//                     start -= 1;
//                     left -= 1;
//                 }
//                 if left_num_in_half > 0 && x as i32 <= left_num_in_half {
//                     start -= 1;
//                 }

//                 res[x] = text[start..end].chars().rev().collect();
//                 end = start;
//             }
//         }
//     });
//     let mut strs = "".to_string();
//     (0..text.len()).for_each(|n| {
//         strs.push(res[i].pop().unwrap());
//         match change_direction {
//             false => {
//                 i += 1;

//                 if i == num_rails {
//                     change_direction = true;
//                     i -= 2;
//                 }
//             }
//             true => {
//                 if i == 0 {
//                     i += 1;
//                     change_direction = false;
//                 } else {
//                     i -= 1;
//                 }
//             }
//         }
//     });
//     strs.to_string()
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(
            decode_rail_fence_cipher(
            " joemnnt ceptemimtia s t  tAsreiiierdmiusdieraiiidn!ioanireaie  adinfiorsua!ouiupcaeqtetm le ot attaq em ae g os .rirosc attrD  m!xsrieoViuseriea uuirpkenm n simiilafurnuouul eosrvt pete rcpndssaf itbxaecoueesaititvie eep,,uvPe fns "
            , 5),
            " rt cumeji uoilioierup eerokeetdmm nemainut sntsnda asqite iimer amiti icialiaeied ftugnp! rvnoitosuio aen.ueurimril  reiaoeeosimecser  t avpttaidt ,prianDe,t f i eu mosr!rvcxs uspPnrat!ideseo uos aVi uiff uptcsinteaAerbsxiqstea eae"
        );
        assert_eq!(decode_rail_fence_cipher(" sodtei soumr iveleieieeg  ciaii siepek pri utnxturm sdrtaiuaPiipom aD ceptroe nrimqcetec ru aoerneusq  oiaanrmta mbtu.jrupf upia ,ettimesdarer!os ta Vlmuldue!e nicn tiuned stosi isfnaAfeeatiiseieni xs ttoifemtse saov!ev,oinraru iai",6),
            " gcuctn.e s pj strtcoiruieupoadiefn e  it nuds prseiiisatameipq oos,ce keeiv tt speti!sicroi mfener uuusavAd tmnaaf,eroxrteeeoarru rn!tiioemi usins ssvdqteria ret  eanVoaliilir maueaauxuslnPiird  tumieptetio!aoim eiaf m eabneimitDe "

        );
        assert_eq!(
            decode_rail_fence_cipher("WAEICVRDLETNEERDSOEEFEAOC", 2),
            "WEAREDISCOVEREDFLEEATONCE"
        );
        assert_eq!(
            encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3),
            "WECRLTEERDSOEEFEAOCAIVDEN"
        );
        assert_eq!(
            decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3),
            "WEAREDISCOVEREDFLEEATONCE"
        );
        assert_eq!(
            encode_rail_fence_cipher("Hello, World!", 3),
            "Hoo!el,Wrdl l"
        );
        assert_eq!(
            decode_rail_fence_cipher("Hoo!el,Wrdl l", 3),
            "Hello, World!"
        );
    }
}
