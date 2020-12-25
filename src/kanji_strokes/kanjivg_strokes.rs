use std::collections::BTreeMap;

use super::kanjivg_parser;
use crate::kanji_strokes::Kanjivg;

struct KanjiDrawRecipe {
    paths: Vec<kanjivg_parser::Path>,
}

impl KanjiDrawRecipe {
    fn color_table() -> [&'static str; 74] {
        [
            "darkmagenta",
            "darkolivegreen",
            "darkorange",
            "darkorchid",
            "darkred",
            "darksalmon",
            "darkseagreen",
            "darkslateblue",
            "darkslategray",
            "darkslategrey",
            "darkturquoise",
            "darkviolet",
            "deeppink",
            "deepskyblue",
            "dimgray",
            "dimgrey",
            "dodgerblue",
            "firebrick",
            "floralwhite",
            "forestgreen",
            "fuchsia",
            "gainsboro",
            "ghostwhite",
            "gold",
            "goldenrod",
            "gray",
            "green",
            "greenyellow",
            "grey",
            "honeydew",
            "hotpink",
            "indianred",
            "indigo",
            "ivory",
            "khaki",
            "lavender",
            "lavenderblush",
            "lawngreen",
            "lemonchiffon",
            "lightblue",
            "lightcoral",
            "lightcyan",
            "lightgoldenrodyellow",
            "lightgray",
            "lightgreen",
            "lightgrey",
            "lightpink",
            "lightsalmon",
            "lightseagreen",
            "lightskyblue",
            "lightslategray",
            "lightslategrey",
            "lightsteelblue",
            "lightyellow",
            "lime",
            "limegreen",
            "linen",
            "magenta",
            "maroon",
            "mediumaquamarine",
            "mediumblue",
            "mediumorchid",
            "mediumpurple",
            "mediumseagreen",
            "mediumslateblue",
            "mediumspringgreen",
            "mediumturquoise",
            "mediumvioletred",
            "midnightblue",
            "mintcream",
            "mistyrose",
            "moccasin",
            "navajowhite",
            "navy",
        ]
    }
    fn generate_svg(&self) -> String {
        let header = r#"<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" version="1.1"  baseProfile="full">"#;
        let tail = r#"</svg>"#;

        let mut body = String::new();
        for (index, p) in self.paths.iter().enumerate() {
            let p: &kanjivg_parser::Path = p;
            let (x, y) = p.extract_x_y();
            body = format!(
                r#"{}
<path style="fill:none;stroke:{color};stroke-width:2" d="{}"/>
<text x="{x}" y="{y}" style="stroke:black" font-size="5">{index}</text>
<text x="{x}" y="{y}" style="fill:{color}" font-size="5">{index}</text>"#,
                body,
                p.d,
                x = x,
                y = y,
                color = Self::color_table()[index],
                index = index+1,
            )
        }

        format!(
            "{header}{body}\n{tail}",
            header = header,
            body = body,
            tail = tail
        )
    }
}

struct Strokes {
    dict: BTreeMap<char, KanjiDrawRecipe>,
}

fn parse_kanjivg() -> kanjivg_parser::Kanjivg {
    serde_xml_rs::from_reader(
        std::fs::File::open("kanjivg.xml").expect("Couldnt open kanjivg file"),
    )
    .expect("Couldnt parse kanjivg struct!")
}

fn kanjivg_into_strokes(kanjivg: &kanjivg_parser::Kanjivg) -> Strokes {
    let dict = kanjivg
        .kanji
        .iter()
        .map(|k: &kanjivg_parser::Kanji| {
            let unicode =
                k.id.strip_prefix("kvg:kanji_")
                    .expect("didt not contain kanji_ prefix?");
            let kanji = u32::from_str_radix(unicode, 16)
                .ok()
                .and_then(std::char::from_u32)
                .expect("couldnt parse out unicode");

            (
                kanji,
                KanjiDrawRecipe {
                    paths: k.extract_subpaths(),
                },
            )
        })
        .collect();

    Strokes { dict }
}

#[cfg(test)]
mod kanjivg_strokes {
    use super::*;
    #[test]
    fn kanji_stroke_test() {
        let parsed : kanjivg_parser::Kanjivg = serde_xml_rs::from_str(r#"
<kanjivg xmlns:kvg='http://kanjivg.tagaini.net'>
<kanji id="kvg:kanji_04e2a">
<g id="kvg:04e2a" kvg:element="个">
	<g id="kvg:04e2a-g1" kvg:element="人" kvg:position="top" kvg:radical="nelson">
		<path id="kvg:04e2a-s1" kvg:type="㇒" d="M52.75,10.25c0.11,1.12,0,3.49-0.72,4.99C47.5,24.75,34.25,45,14.25,57.75"/>
		<path id="kvg:04e2a-s2" kvg:type="㇏" d="M51.75,15.75c5.92,7.28,31.44,31.07,37.97,36.4c2.22,1.81,5.06,2.58,7.28,3.1"/>
	</g>
	<g id="kvg:04e2a-g2" kvg:element="丨" kvg:position="bottom" kvg:radical="tradit">
		<path id="kvg:04e2a-s3" kvg:type="㇑" d="M51.87,46.25c1.09,0.5,1.74,2.25,1.96,3.25c0.22,1,0,43.25-0.22,49.5"/>
	</g>
</g>
</kanji>
</kanjivg>
            "#
        ).unwrap();

        let strokes = kanjivg_into_strokes(&parsed);
        let draw_recipe: &KanjiDrawRecipe = strokes.dict.get(&'个').unwrap();

        let expected = r#"
<svg width="100" height="100" viewBox="0 0 100 100" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" xml:space="preserve" version="1.1"  baseProfile="full">
<path style="fill:none;stroke:darkmagenta;stroke-width:2" d="M52.75,10.25c0.11,1.12,0,3.49-0.72,4.99C47.5,24.75,34.25,45,14.25,57.75"/>
<text x="52.75" y="10.25" style="stroke:black" font-size="5">1</text>
<text x="52.75" y="10.25" style="fill:darkmagenta" font-size="5">1</text>
<path style="fill:none;stroke:darkolivegreen;stroke-width:2" d="M51.75,15.75c5.92,7.28,31.44,31.07,37.97,36.4c2.22,1.81,5.06,2.58,7.28,3.1"/>
<text x="51.75" y="15.75" style="stroke:black" font-size="5">2</text>
<text x="51.75" y="15.75" style="fill:darkolivegreen" font-size="5">2</text>
<path style="fill:none;stroke:darkorange;stroke-width:2" d="M51.87,46.25c1.09,0.5,1.74,2.25,1.96,3.25c0.22,1,0,43.25-0.22,49.5"/>
<text x="51.87" y="46.25" style="stroke:black" font-size="5">3</text>
<text x="51.87" y="46.25" style="fill:darkorange" font-size="5">3</text>
</svg>
        "#.trim();

        assert_eq!(draw_recipe.generate_svg(), expected)
    }
}
