use serde_xml_rs as serde_xml;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Path {
    pub d: String,
}

impl Path {
    pub fn extract_x_y(&self) -> (f64, f64) {
        let mut split = self.d.split(',');
        let first: &str = split.next().expect("no first coordinate?");
        let second: &str = split.next().expect("no second coordinate?");
        let first = &first[1..]; // pop first character which is 'M' always.
        let x = first
            .parse::<f64>()
            .expect("Couldnt parse out x coordinate");
        let position_of_c = second
            .chars()
            .position(|c| c == 'c' || c == 'C' || c == ' ')
            .unwrap_or(second.len());
        let y = second[..position_of_c]
            .parse::<f64>()
            .expect("couldnt parse out y coordinate");
        (x, y)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KanjiGroup {
    pub g: Option<Vec<KanjiGroup>>,
    pub path: Option<Vec<Path>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kanji {
    pub id: String,
    pub g: KanjiGroup,
}

impl Kanji {
    pub fn extract_subpaths(&self) -> Vec<Path> {
        let mut result = Vec::new();
        Self::extract_subpaths_impl(&self.g, &mut result);
        result
    }

    fn extract_subpaths_impl(g: &KanjiGroup, paths_so_far: &mut Vec<Path>) {
        if let Some(paths) = &g.path {
            for p in paths {
                paths_so_far.push(p.clone());
            }
        }

        if let Some(subgroup) = &g.g {
            for g in subgroup {
                Self::extract_subpaths_impl(g, paths_so_far);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Kanjivg {
    pub kanji: Vec<Kanji>,
}

#[cfg(test)]
mod kanjivg_parser_test {
    use super::*;
    use serde_xml_rs as serde_xml;

    #[test]
    fn basic_deser() {
        let example = r#"
<kanjivg xmlns:kvg='http://kanjivg.tagaini.net'>
<kanji id="kvg:kanji_00021">
<g id="kvg:00021">
	<path id="kvg:00021-s1" d="M54.5,15.79c0,6.07-0.29,55.49-0.29,60.55"/>
	<path id="kvg:00021-s2" d="M54.5,88 c -0.83,0 -1.5,0.67 -1.5,1.5 0,0.83 0.67,1.5 1.5,1.5 0.83,0 1.5,-0.67 1.5,-1.5 0,-0.83 -0.67,-1.5 -1.5,-1.5"/>
</g>
</kanji>
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
        "#;

        let x: Result<Kanjivg, _> = serde_xml::from_str(example);

        let x = x.unwrap();
        let first = x.kanji.first().unwrap();
        assert_eq!(first.id, "kvg:kanji_00021");
        let paths = first.g.path.as_ref().unwrap();
        assert_eq!(paths.len(), 2);
        assert_eq!(first.extract_subpaths().len(), 2);

        let second = x.kanji.get(1).unwrap();
        assert_eq!(second.id, "kvg:kanji_04e2a");
        assert!(second.g.path.is_none());

        assert_eq!(second.extract_subpaths().len(), 3);

        let x_ys: Vec<_> = second
            .extract_subpaths()
            .into_iter()
            .map(|p| p.extract_x_y())
            .collect();

        // 		<path id="kvg:04e2a-s1" kvg:type="㇒" d="M52.75,10.25c0.11,1.12,0,3.49-0.72,4.99C47.5,24.75,34.25,45,14.25,57.75"/>
        // 		<path id="kvg:04e2a-s2" kvg:type="㇏" d="M51.75,15.75c5.92,7.28,31.44,31.07,37.97,36.4c2.22,1.81,5.06,2.58,7.28,3.1"/>
        // 	</g>
        // 	<g id="kvg:04e2a-g2" kvg:element="丨" kvg:position="bottom" kvg:radical="tradit">
        // 		<path id="kvg:04e2a-s3" kvg:type="㇑" d="M51.87,46.25c1.09,0.5,1.74,2.25,1.96,3.25c0.22,1,0,43.25-0.22,49.5"/>
        let expected_x_ys = vec![(52.75, 10.25), (51.75, 15.75), (51.87, 46.25)];
        assert_eq!(x_ys, expected_x_ys);
    }
}
