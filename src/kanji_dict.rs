use serde_xml_rs as serde_xml;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CpValue {
    pub cp_type: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CodePoint {
    pub cp_value: Vec<CpValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RadValue {
    pub rad_type: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Radical {
    pub rad_value: Vec<RadValue>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Misc {
    pub grade: Option<u32>,
    pub stroke_count: Vec<u32>,
    pub freq: Option<u32>,
    pub jlpt: Option<u32>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Reading {
    // important ones are ja_on and ja_kun
    pub r_type: String,
    #[serde(rename = "$value")]
    pub value: String,
}

fn default_m_lang() -> String {
    "en".to_string()
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Meaning {
    #[serde(default = "default_m_lang")]
    pub m_lang: String,
    #[serde(rename = "$value")]
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RmGroup {
    //there are very few but indeed there are some kanjis without reading entries present.
    pub reading: Option<Vec<Reading>>,
    pub meaning: Option<Vec<Meaning>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ReadingMeaning {
    pub rmgroup: RmGroup,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Character {
    pub literal: char,
    pub codepoint: CodePoint,
    pub radical: Radical,
    pub misc: Misc,
    pub reading_meaning: Option<ReadingMeaning>,
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct KanjiDictionary {
    pub character: Vec<Character>,
}

#[cfg(test)]
mod kanji_tests {
    use super::*;
    use serde_xml_rs as serde_xml;

    #[test]
    fn basic_deser() {
        let s = r#"
<kanjidic2>
<character>
<literal>亜</literal>
<codepoint>
<cp_value cp_type="ucs">4e9c</cp_value>
<cp_value cp_type="jis208">1-16-01</cp_value>
</codepoint>
<radical>
<rad_value rad_type="classical">7</rad_value>
<rad_value rad_type="nelson_c">1</rad_value>
</radical>
<misc>
<grade>8</grade>
<stroke_count>7</stroke_count>
<variant var_type="jis208">1-48-19</variant>
<freq>1509</freq>
<jlpt>1</jlpt>
</misc>
<dic_number>
<dic_ref dr_type="nelson_c">43</dic_ref>
<dic_ref dr_type="nelson_n">81</dic_ref>
<dic_ref dr_type="halpern_njecd">3540</dic_ref>
<dic_ref dr_type="halpern_kkd">4354</dic_ref>
<dic_ref dr_type="halpern_kkld">2204</dic_ref>
<dic_ref dr_type="halpern_kkld_2ed">2966</dic_ref>
<dic_ref dr_type="heisig">1809</dic_ref>
<dic_ref dr_type="heisig6">1950</dic_ref>
<dic_ref dr_type="gakken">1331</dic_ref>
<dic_ref dr_type="oneill_names">525</dic_ref>
<dic_ref dr_type="oneill_kk">1788</dic_ref>
<dic_ref dr_type="moro" m_vol="1" m_page="0525">272</dic_ref>
<dic_ref dr_type="henshall">997</dic_ref>
<dic_ref dr_type="sh_kk">1616</dic_ref>
<dic_ref dr_type="sh_kk2">1724</dic_ref>
<dic_ref dr_type="jf_cards">1032</dic_ref>
<dic_ref dr_type="tutt_cards">1092</dic_ref>
<dic_ref dr_type="kanji_in_context">1818</dic_ref>
<dic_ref dr_type="kodansha_compact">35</dic_ref>
<dic_ref dr_type="maniette">1827</dic_ref>
</dic_number>
<query_code>
<q_code qc_type="skip">4-7-1</q_code>
<q_code qc_type="sh_desc">0a7.14</q_code>
<q_code qc_type="four_corner">1010.6</q_code>
<q_code qc_type="deroo">3273</q_code>
</query_code>
<reading_meaning>
<rmgroup>
<reading r_type="pinyin">ya4</reading>
<reading r_type="korean_r">a</reading>
<reading r_type="korean_h">아</reading>
<reading r_type="vietnam">A</reading>
<reading r_type="vietnam">Á</reading>
<reading r_type="ja_on">ア</reading>
<reading r_type="ja_kun">つ.ぐ</reading>
<meaning>Asia</meaning>
<meaning>rank next</meaning>
<meaning>come after</meaning>
<meaning>-ous</meaning>
<meaning m_lang="fr">Asie</meaning>
<meaning m_lang="fr">suivant</meaning>
<meaning m_lang="fr">sub-</meaning>
<meaning m_lang="fr">sous-</meaning>
<meaning m_lang="es">pref. para indicar</meaning>
<meaning m_lang="es">venir después de</meaning>
<meaning m_lang="es">Asia</meaning>
<meaning m_lang="pt">Ásia</meaning>
<meaning m_lang="pt">próxima</meaning>
<meaning m_lang="pt">o que vem depois</meaning>
<meaning m_lang="pt">-ous</meaning>
</rmgroup>
<nanori>や</nanori>
<nanori>つぎ</nanori>
<nanori>つぐ</nanori>
</reading_meaning>
</character>
<!-- Entry for Kanji: 唖 -->
<character>
<literal>唖</literal>
<codepoint>
<cp_value cp_type="ucs">5516</cp_value>
<cp_value cp_type="jis208">1-16-2</cp_value>
</codepoint>
<radical>
<rad_value rad_type="classical">30</rad_value>
</radical>
<misc>
<stroke_count>10</stroke_count>
<variant var_type="jis212">1-21-64</variant>
<variant var_type="jis212">1-45-68</variant>
</misc>
<dic_number>
<dic_ref dr_type="nelson_c">939</dic_ref>
<dic_ref dr_type="nelson_n">795</dic_ref>
<dic_ref dr_type="heisig">2958</dic_ref>
<dic_ref dr_type="heisig6">2964</dic_ref>
<dic_ref dr_type="moro" m_vol="2" m_page="1066">3743</dic_ref>
</dic_number>
<query_code>
<q_code qc_type="skip">1-3-7</q_code>
<q_code qc_type="sh_desc">3d8.3</q_code>
<q_code qc_type="four_corner">6101.7</q_code>
</query_code>
<reading_meaning>
<rmgroup>
<reading r_type="pinyin">ya1</reading>
<reading r_type="korean_r">a</reading>
<reading r_type="korean_h">아</reading>
<reading r_type="vietnam">Á</reading>
<reading r_type="vietnam">Ớ</reading>
<reading r_type="vietnam">Ứ</reading>
<reading r_type="ja_on">ア</reading>
<reading r_type="ja_on">アク</reading>
<reading r_type="ja_kun">おし</reading>
<meaning>mute</meaning>
<meaning>dumb</meaning>
</rmgroup>
</reading_meaning>
</character>
</kanjidic2>
"#;
        let x: Result<KanjiDictionary, _> = serde_xml::from_str(s);

        let x = x.unwrap();
        let first = x.character.first().unwrap();
        assert_eq!('亜', first.literal);
    }
}
