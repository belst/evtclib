//! Tests to ensure that CM detection is working.
//!
//! CM logs should be placed in the logs/cms folder, unless they are fights which only exist in the
//! challenge mote (or mostly exist in CM), like the fractal CM fights.

use std::path::Path;

use evtclib::Compression;

macro_rules! test {
    ($name:ident, $log:expr) => {
        #[test]
        fn $name() {
            check_cm($log);
        }
    };
}

fn check_cm(path: &str) {
    let path = Path::new("tests").join(path);
    let log = evtclib::process_file(&path, Compression::Zip).unwrap();
    assert!(log.is_cm(), "expected {:?} to be a CM log", path);
}

test!(test_cairn_cm, "logs/cms/cairn.zevtc");
test!(test_mo_cm, "logs/cms/mo.zevtc");
test!(test_samarog_cm, "logs/cms/samarog.zevtc");
test!(test_deimos_cm, "logs/cms/deimos.zevtc");

test!(test_desmina_cm, "logs/cms/desmina.zevtc");
test!(test_dhuum_cm, "logs/cms/desmina.zevtc");

test!(test_ca_cm, "logs/cms/ca.zevtc");
test!(test_largos_cm, "logs/cms/largos.zevtc");
test!(test_qadim_cm, "logs/cms/qadim.zevtc");

test!(test_adina_cm, "logs/cms/adina.zevtc");
test!(test_sabir_cm, "logs/cms/sabir.zevtc");
test!(test_qadimp_cm, "logs/cms/qadimp.zevtc");

test!(test_skorvald_cm, "logs/skorvald-20200427.zevtc");
test!(test_artsariiv_cm, "logs/artsariiv-20200427.zevtc");
test!(test_arkk_cm, "logs/arkk-20200427.zevtc");

test!(test_mama_cm, "logs/mama-20200427.zevtc");
test!(test_siax_cm, "logs/siax-20200427.zevtc");
test!(test_ensolyss_cm, "logs/ensolyss-20200427.zevtc");
